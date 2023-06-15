use fnv::FnvHashMap;
use std::mem::{swap, take};
use std::rc::Rc;

use crate::block::{Block, BlockOrValue, Comparator};
use crate::errorkey::ErrorKey;
use crate::errors::{error, warn, warn_info};
use crate::fileset::FileEntry;
use crate::token::{Loc, Token};

#[derive(Copy, Clone, Debug)]
enum State {
    Neutral,
    QString,
    Id,
    Comparator,
    Calculation,
    CalculationId,
    Comment,
}

#[derive(Copy, Clone, Debug)]
enum CalculationOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[allow(clippy::wrong_self_convention)]
trait CharExt {
    fn is_id_char(self) -> bool;
    fn is_comparator_char(self) -> bool;
}

impl CharExt for char {
    fn is_id_char(self) -> bool {
        self.is_alphabetic()
            || self.is_ascii_digit()
            || self == '.'
            || self == ':'
            || self == '_'
            || self == '-'
            || self == '&'
            || self == '/'
            || self == '|'
            || self == '\''
            || self == '%' // added for parsing .gui files
            || self == '[' // added for parsing .gui files
            || self == ']' // added for parsing .gui files
    }

    fn is_comparator_char(self) -> bool {
        self == '<' || self == '>' || self == '!' || self == '=' || self == '?'
    }
}

#[derive(Clone, Debug, Default)]
pub struct LocalMacros {
    values: FnvHashMap<String, f64>,
    text: FnvHashMap<String, String>,
}

impl LocalMacros {
    fn get_value(&self, key: &str) -> Option<f64> {
        // key can be a local macro or a literal numeric value
        self.values.get(key).copied().or_else(|| key.parse().ok())
    }

    fn get_as_string(&self, key: &str) -> Option<String> {
        if let Some(value) = self.values.get(key) {
            Some(value.to_string())
        } else if let Some(value) = self.text.get(key) {
            Some(value.to_string())
        } else {
            None
        }
    }

    fn insert(&mut self, key: &str, value: &str) {
        if let Ok(value) = value.parse::<f64>() {
            self.values.insert(key.to_string(), value);
        } else {
            self.text.insert(key.to_string(), value.to_string());
        }
    }
}

struct ParseLevel {
    block: Block,
    key: Option<Token>,
    comp: Option<(Comparator, Token)>,
    tag: Option<Token>,
    contains_macro_parms: bool,
}

struct Parser {
    current: ParseLevel,
    stack: Vec<ParseLevel>,
    local_macros: LocalMacros,
    calculation_op: CalculationOp,
    calculation: f64,
}

impl Parser {
    fn unknown_char(c: char, loc: Loc) {
        let token = Token::new(c.to_string(), loc);
        error(
            token,
            ErrorKey::ParseError,
            &format!("Unrecognized character {c}"),
        );
    }

    fn calculation_start(&mut self) {
        self.calculation = 0.0;
        self.calculation_op = CalculationOp::Add;
    }

    fn calculation_op(&mut self, op: CalculationOp) {
        self.calculation_op = op;
    }

    fn calculation_next(&mut self, local_macro: &Token) {
        if let Some(value) = self.local_macros.get_value(local_macro.as_str()) {
            match self.calculation_op {
                CalculationOp::Add => self.calculation += value,
                CalculationOp::Subtract => self.calculation -= value,
                CalculationOp::Multiply => self.calculation *= value,
                CalculationOp::Divide => self.calculation /= value,
            }
        } else {
            let msg = format!("local value {local_macro} not defined");
            error(local_macro, ErrorKey::ParseError, &msg);
        }
    }

    fn calculation_result(&mut self) -> f64 {
        take(&mut self.calculation)
    }

    fn token(&mut self, token: Token) {
        // Special case parsing of color = hsv { ... }
        if token.is("hsv") || token.is("rgb") || token.is("hsv360") {
            self.current.tag = Some(token);
            return;
        }
        if self.stack.is_empty() && self.current.contains_macro_parms {
            error(
                &token,
                ErrorKey::ParseError,
                "$-substitutions only work inside blocks, not at top level",
            );
            self.current.contains_macro_parms = false;
        }
        if let Some(key) = self.current.key.take() {
            if let Some((comp, _)) = self.current.comp.take() {
                if let Some(local_macro) = key.as_str().strip_prefix('@') {
                    self.local_macros.insert(local_macro, token.as_str());
                } else if let Some(local_macro) = token.as_str().strip_prefix('@') {
                    // Check for a '!' to avoid looking up macros in gui code that uses @icon! syntax
                    if !token.as_str().contains('!') {
                        if let Some(value) = self.local_macros.get_as_string(local_macro) {
                            let token = Token::new(value, token.loc);
                            self.current
                                .block
                                .add_key_value(key, comp, BlockOrValue::Value(token));
                        } else {
                            error(token, ErrorKey::ParseError, "local value not defined");
                        }
                    } else {
                        self.current
                            .block
                            .add_key_value(key, comp, BlockOrValue::Value(token));
                    }
                } else {
                    self.current
                        .block
                        .add_key_value(key, comp, BlockOrValue::Value(token));
                }
            } else {
                self.current.block.add_value(BlockOrValue::Value(key));
                self.current.key = Some(token);
            }
        } else {
            self.current.key = Some(token);
        }
    }

    fn block_value(&mut self, mut block: Block) {
        // Like token(), but block values cannot become keys
        if let Some(tag) = self.current.tag.take() {
            block.tag = Some(tag);
        }
        if let Some(key) = self.current.key.take() {
            if let Some((comp, _)) = self.current.comp.take() {
                self.current
                    .block
                    .add_key_value(key, comp, BlockOrValue::Block(block));
            } else {
                self.current.block.add_value(BlockOrValue::Value(key));
                self.current.block.add_value(BlockOrValue::Block(block));
            }
        } else {
            self.current.block.add_value(BlockOrValue::Block(block));
        }
    }

    fn comparator(&mut self, token: Token) {
        let cmp = Comparator::from_token(&token).unwrap_or_else(|| {
            error(
                &token,
                ErrorKey::ParseError,
                &format!("Unrecognized comparator '{token}'"),
            );
            Comparator::Eq
        });

        if self.current.key.is_none() {
            let msg = format!("Unexpected comparator '{token}'");
            error(token, ErrorKey::ParseError, &msg);
        } else {
            if self.current.comp.is_some() {
                let msg = &format!("Double comparator '{token}'");
                error(&token, ErrorKey::ParseError, msg);
            }
            self.current.comp = Some((cmp, token));
        }
    }

    fn end_assign(&mut self) {
        if let Some(key) = self.current.key.take() {
            if let Some((_, comp_token)) = self.current.comp.take() {
                error(comp_token, ErrorKey::ParseError, "Comparator without value");
            }
            self.current.block.add_value(BlockOrValue::Value(key));
        }
    }

    fn open_brace(&mut self, loc: Loc) {
        let mut new_level = ParseLevel {
            block: Block::new(loc),
            key: None,
            comp: None,
            tag: None,
            contains_macro_parms: false,
        };
        swap(&mut new_level, &mut self.current);
        self.stack.push(new_level);
    }

    fn close_brace(&mut self, loc: Loc, content: &str) {
        self.end_assign();
        if let Some(mut prev_level) = self.stack.pop() {
            swap(&mut self.current, &mut prev_level);
            if self.stack.is_empty() && prev_level.contains_macro_parms {
                // skip the { } in constructing s
                let s = content[prev_level.block.loc.offset + 1..loc.offset].to_string();
                let mut loc = prev_level.block.loc.clone();
                loc.offset += 1;
                loc.column += 1;
                let token = Token::new(s, prev_level.block.loc.clone());
                prev_level.block.source = Some((token, self.local_macros.clone()));
            } else {
                self.current.contains_macro_parms |= prev_level.contains_macro_parms;
            }
            self.block_value(prev_level.block);
            if loc.column == 1 && !self.stack.is_empty() {
                warn_info(Token::new("}".to_string(), loc),
                ErrorKey::BracePlacement,
                "possible brace error",
                "This closing brace is at the start of a line but does not end a top-level item."
                );
            }
        } else {
            error(
                Token::new("}".to_string(), loc),
                ErrorKey::ParseError,
                "Unexpected }",
            );
        }
    }

    fn eof(mut self) -> Block {
        self.end_assign();
        while let Some(mut prev_level) = self.stack.pop() {
            error(
                &Token::new("{".to_string(), self.current.block.loc.clone()),
                ErrorKey::ParseError,
                "Opening { was never closed",
            );
            swap(&mut self.current, &mut prev_level);
            self.block_value(prev_level.block);
        }
        self.current.block
    }
}

#[allow(clippy::too_many_lines)] // many lines are natural for state machines
fn parse(blockloc: Loc, inputs: &[Token], local_macros: LocalMacros) -> Block {
    let mut parser = Parser {
        current: ParseLevel {
            block: Block::new(blockloc.clone()),
            key: None,
            comp: None,
            tag: None,
            contains_macro_parms: false,
        },
        stack: Vec::new(),
        local_macros: local_macros,
        calculation: 0.0,
        calculation_op: CalculationOp::Add,
    };
    let mut state = State::Neutral;
    let mut token_start = blockloc.clone();
    let mut calculation_start = blockloc;
    let mut current_id = String::new();

    for token in inputs {
        let content = token.as_str();
        let mut loc = token.loc.clone();
        let loc_start = loc.offset;

        for (i, c) in content.char_indices() {
            loc.offset = loc_start + i;

            match state {
                State::Neutral => {
                    current_id.clear();
                    if c.is_whitespace() {
                    } else if c == '"' {
                        state = State::QString;
                    } else if c == '#' {
                        state = State::Comment;
                    } else if c.is_comparator_char() {
                        state = State::Comparator;
                        current_id.push(c);
                    } else if c == '@' {
                        // @ can start tokens but is special
                        calculation_start = loc.clone();
                        current_id.push(c);
                        state = State::Id;
                    } else if c == '$' {
                        parser.current.contains_macro_parms = true;
                        state = State::Id;
                        current_id.push(c);
                    } else if c.is_id_char() {
                        state = State::Id;
                        current_id.push(c);
                    } else if c == '{' {
                        parser.open_brace(loc.clone());
                    } else if c == '}' {
                        parser.close_brace(loc.clone(), content);
                    } else {
                        Parser::unknown_char(c, loc.clone());
                    }
                    token_start = loc.clone();
                }
                State::Comment => {
                    if c == '\n' {
                        state = State::Neutral;
                    }
                }
                State::QString => {
                    if c == '"' {
                        state = State::Id;
                    } else if c == '\n' {
                        let token = Token::new(take(&mut current_id), token_start.clone());
                        warn(token, ErrorKey::ParseError, "Quoted string not closed");
                    } else {
                        current_id.push(c);
                    }
                }
                State::Id => {
                    if c == '"' {
                        // The quoted string actually becomes part of this id
                        state = State::QString;
                    } else if c == '$' {
                        parser.current.contains_macro_parms = true;
                        current_id.push(c);
                    } else if c == '['
                        && loc.offset == token_start.offset + 1
                        && current_id.starts_with('@')
                    {
                        state = State::Calculation;
                        parser.calculation_start();
                    } else if c.is_id_char() {
                        current_id.push(c);
                    } else {
                        let token = Token::new(take(&mut current_id), token_start.clone());
                        parser.token(token);

                        if c.is_comparator_char() {
                            current_id.push(c);
                            state = State::Comparator;
                        } else if c.is_whitespace() {
                            state = State::Neutral;
                        } else if c == '#' {
                            state = State::Comment;
                        } else if c == '{' {
                            parser.open_brace(loc.clone());
                            state = State::Neutral;
                        } else if c == '}' {
                            parser.close_brace(loc.clone(), content);
                            state = State::Neutral;
                        } else {
                            Parser::unknown_char(c, loc.clone());
                            state = State::Neutral;
                        }
                        token_start = loc.clone();
                    }
                }
                State::Calculation => {
                    current_id.clear();
                    if c.is_whitespace() {
                    } else if c == '+' {
                        parser.calculation_op(CalculationOp::Add);
                    } else if c == '-' {
                        parser.calculation_op(CalculationOp::Subtract);
                    } else if c == '*' {
                        parser.calculation_op(CalculationOp::Multiply);
                    } else if c == '/' {
                        parser.calculation_op(CalculationOp::Divide);
                    } else if c == ']' {
                        let token = Token::new(
                            parser.calculation_result().to_string(),
                            calculation_start.clone(),
                        );
                        parser.token(token);
                        state = State::Neutral;
                    } else if c.is_id_char() {
                        state = State::CalculationId;
                        current_id.push(c);
                    }
                    token_start = loc.clone();
                }
                State::CalculationId => {
                    if c.is_whitespace() || c == '+' || c == '/' || c == '*' || c == '-' {
                        let token = Token::new(take(&mut current_id), token_start.clone());
                        parser.calculation_next(&token);
                        state = State::Calculation;
                        if c == '+' {
                            parser.calculation_op(CalculationOp::Add);
                        } else if c == '-' {
                            parser.calculation_op(CalculationOp::Subtract);
                        } else if c == '*' {
                            parser.calculation_op(CalculationOp::Multiply);
                        } else if c == '/' {
                            parser.calculation_op(CalculationOp::Divide);
                        }
                    } else if c == ']' {
                        let token = Token::new(take(&mut current_id), token_start.clone());
                        parser.calculation_next(&token);

                        let token = Token::new(
                            parser.calculation_result().to_string(),
                            calculation_start.clone(),
                        );
                        parser.token(token);
                        state = State::Neutral;
                    } else if c.is_id_char() {
                        current_id.push(c);
                    } else {
                        Parser::unknown_char(c, loc.clone());
                        current_id.clear();
                        state = State::Neutral;
                    }
                }
                State::Comparator => {
                    if c.is_comparator_char() {
                        current_id.push(c);
                    } else {
                        let token = Token::new(take(&mut current_id), token_start.clone());
                        parser.comparator(token);

                        if c == '"' {
                            state = State::QString;
                        } else if c == '@' {
                            // @ can start tokens but is special
                            calculation_start = loc.clone();
                            current_id.push(c);
                            state = State::Id;
                        } else if c == '$' {
                            parser.current.contains_macro_parms = true;
                            state = State::Id;
                            current_id.push(c);
                        } else if c.is_id_char() {
                            state = State::Id;
                            current_id.push(c);
                        } else if c.is_whitespace() {
                            state = State::Neutral;
                        } else if c == '#' {
                            state = State::Comment;
                        } else if c == '{' {
                            parser.open_brace(loc.clone());
                            state = State::Neutral;
                        } else if c == '}' {
                            parser.close_brace(loc.clone(), content);
                            state = State::Neutral;
                        } else {
                            Parser::unknown_char(c, loc.clone());
                            state = State::Neutral;
                        }
                        token_start = loc.clone();
                    }
                }
            }

            if c == '\n' {
                loc.line += 1;
                loc.column = 1;
            } else {
                loc.column += 1;
            }
        }
    }

    // Deal with state at end of file
    match state {
        State::QString => {
            let token = Token::new(current_id, token_start);
            error(&token, ErrorKey::ParseError, "Quoted string not closed");
            parser.token(token);
        }
        State::Id => {
            let token = Token::new(current_id, token_start);
            parser.token(token);
        }
        State::Comparator => {
            let token = Token::new(current_id, token_start);
            parser.comparator(token);
        }
        _ => (),
    }

    parser.eof()
}

#[allow(clippy::module_name_repetitions)]
pub fn parse_pdx(entry: &FileEntry, content: &str) -> Block {
    let blockloc = Loc::for_entry(entry);
    let mut loc = blockloc.clone();
    loc.line = 1;
    loc.column = 1;
    parse(
        blockloc,
        &[Token::new(content.to_string(), loc)],
        LocalMacros::default(),
    )
}

pub fn parse_pdx_macro(inputs: &[Token], local_macros: LocalMacros) -> Block {
    parse(inputs[0].loc.clone(), inputs, local_macros)
}

// Simplified parsing just to get the macro arguments
pub fn split_macros(content: &Token, link: Option<&Token>) -> Vec<Token> {
    #[derive(Eq, PartialEq)]
    enum State {
        Normal,
        InQString,
        InComment,
    }
    let mut state = State::Normal;
    let mut vec = Vec::new();
    let mut loc = content.loc.clone();
    if let Some(link) = link {
        loc.link = Some(Rc::new(link.loc.clone()));
    }
    let mut last_loc = loc.clone();
    let mut last_pos = 0;
    for (i, c) in content.as_str().char_indices() {
        loc.offset = content.loc.offset + i;
        match state {
            State::InComment => {
                if c == '\n' {
                    state = State::Normal;
                }
            }
            State::InQString => {
                if c == '\n' || c == '"' {
                    state = State::Normal;
                }
            }
            State::Normal => {
                if c == '#' {
                    state = State::InComment;
                } else if c == '"' {
                    state = State::InQString;
                } else if c == '$' {
                    vec.push(Token::new(
                        content.as_str()[last_pos..i].to_string(),
                        last_loc,
                    ));
                    last_loc = loc.clone();
                    // Skip the current '$'
                    last_loc.column += 1;
                    last_loc.offset += 1;
                    last_pos = i + 1;
                }
            }
        }
        if c == '\n' {
            loc.column = 1;
            loc.line += 1;
        } else {
            loc.column += 1;
        }
    }
    vec.push(Token::new(
        content.as_str()[last_pos..].to_string(),
        last_loc,
    ));
    vec
}
