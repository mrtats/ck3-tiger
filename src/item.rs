use std::fmt::{Display, Formatter};
use strum_macros::IntoStaticStr;

/// "items" are all the things that can be looked up in string-indexed databases.
/// There is some overlap with scopes, but the difference is that scopes are runtime values
/// while items are always strings.
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoStaticStr, Hash, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum Item {
    Amenity,
    AccoladeCategory,
    AccoladeParameter,
    AccoladeType,
    ActivityIntent,
    ActivityLocale,
    ActivityOption,
    ActivityOptionCategory,
    ActivityPhase,
    ActivityState,
    ActivityType,
    Artifact,
    ArtifactCategory,
    ArtifactFeature,
    ArtifactFeatureGroup,
    ArtifactHistory,
    ArtifactModifier,
    ArtifactRarity,
    ArtifactSlot,
    ArtifactTemplate,
    ArtifactVisual,
    Building,
    BuildingFlag,
    BuildingGfx,
    CasusBelli,
    Catalyst,
    Character,
    CharacterTemplate,
    ClothingGfx,
    Coa,
    CoaGfx,
    CouncilPosition,
    CouncilTask,
    CourtPosition,
    CourtPositionCategory,
    CourtSceneGroup,
    CourtType,
    Culture,
    CultureEra,
    CultureParameter,
    CulturePillar,
    CultureTradition,
    DangerType,
    DeathReason,
    Decision,
    Define,
    DiarchyMandate,
    DiarchyParameter,
    DiarchyType,
    Dlc,
    DlcFeature,
    Dna,
    Doctrine,
    DoctrineParameter,
    Dynasty,
    DynastyLegacy,
    DynastyPerk,
    EducationFocus,
    EffectLocalization,
    Event,
    EventBackground,
    EventTheme,
    EventTransition,
    Faction,
    Faith,
    FaithIcon,
    File,
    Focus,
    GameConcept,
    GameRule,
    GeneAgePreset,
    GeneAttribute,
    GeneCategory,
    GeneTemplate,
    GfxPortraitsAccessories,
    GovernmentType,
    GovernmentFlag,
    GraphicalFaith,
    GuestSubset,
    Holding,
    HolySite,
    HolySiteFlag,
    Hook,
    House,
    ImportantAction,
    Innovation,
    InnovationFlag,
    Inspiration,
    Interaction,
    InteractionCategory,
    Language,
    Law,
    LawFlag,
    Lifestyle,
    Localization,
    MapMode,
    MemoryCategory,
    MemoryType,
    MenAtArms,
    MenAtArmsBase,
    Modifier,
    Music,
    NameList,
    Nickname,
    OnAction,
    Perk,
    PerkTree,
    PrisonType,
    Province,
    Region,
    Relation,
    RelationFlag,
    Religion,
    ReligiousFamily,
    RewardItem,
    Scheme,
    ScriptedEffect,
    ScriptedGui,
    ScriptedList,
    ScriptedModifier,
    ScriptedRule,
    ScriptedTrigger,
    ScriptValue,
    Secret,
    Sexuality,
    Skill,
    SpecialBuilding,
    Story,
    Struggle,
    StrugglePhase,
    StrugglePhaseParameter,
    Suggestion,
    Terrain,
    Title,
    TitleHistory,
    TitleHistoryType,
    TitleLaw,
    TitleLawFlag,
    Tradition,
    Trait,
    TraitCategory,
    TraitFlag,
    TraitTrack,
    TravelOption,
    TravelPlanModifier,
    TriggerLocalization,
    UnitGfx,
    VassalContractFlag,
    VassalObligation,
    VassalObligationLevel,
    VassalStance,
}

use crate::item::Item::*;

impl Item {
    pub fn path(self) -> &'static str {
        #[allow(clippy::match_same_arms)]
        match self {
            AccoladeCategory => "common/accolade_types/",
            AccoladeParameter => "common/accolade_types/",
            AccoladeType => "common/accolade_types/",
            ActivityIntent => "common/activities/activity_types/",
            ActivityLocale => "common/activities/activity_locales/",
            ActivityOption => "common/activities/activity_types/",
            ActivityOptionCategory => "common/activities/activity_types/",
            ActivityPhase => "common/activities/activity_types/",
            ActivityState => "",
            ActivityType => "common/activities/activity_types/",
            Amenity => "common/court_amenities/",
            Artifact => "common/artifacts/types/",
            ArtifactCategory => "common/artifacts/",
            ArtifactFeature => "common/artifacts/features/",
            ArtifactFeatureGroup => "common/artifacts/feature_groups/",
            ArtifactHistory => "",
            ArtifactModifier => "common/artifacts/",
            ArtifactRarity => "common/artifacts/",
            ArtifactSlot => "common/artifacts/slots/",
            ArtifactTemplate => "common/artifacts/templates/",
            ArtifactVisual => "common/artifacts/visuals/",
            Building => "common/buildings/",
            BuildingFlag => "common/buildings/",
            BuildingGfx => "common/culture/cultures/",
            CasusBelli => "common/casus_belli_types/",
            Catalyst => "common/struggle/catalysts/",
            Character => "history/characters/",
            CharacterTemplate => "common/scripted_character_templates/",
            ClothingGfx => "common/culture/cultures/",
            Coa => "common/coat_of_arms/coat_of_arms/",
            CoaGfx => "common/culture/cultures/",
            CouncilPosition => "common/council_positions/",
            CouncilTask => "common/council_tasks/",
            CourtPosition => "common/court_positions/types/",
            CourtPositionCategory => "common/court_positions/categories/",
            CourtSceneGroup => "gfx/court_scene/character_groups/",
            CourtType => "common/court_types/",
            Culture => "common/culture/cultures/",
            CultureEra => "common/culture/eras/",
            CultureParameter => "common/culture/cultures/",
            CulturePillar => "common/culture/pillars/",
            CultureTradition => "common/culture/traditions/",
            DangerType => "",
            DeathReason => "common/deathreasons/",
            Decision => "common/decisions/",
            Define => "common/defines/",
            DiarchyMandate => "common/diarchies/diarchy_mandates/",
            DiarchyParameter => "common/diarchies/diarchy_types/",
            DiarchyType => "common/diarchies/diarchy_types/",
            Dlc => "",
            DlcFeature => "",
            Dna => "common/dna_data/",
            Doctrine => "common/religion/doctrines/",
            DoctrineParameter => "common/religion/doctrines/",
            Dynasty => "common/dynasties/",
            DynastyLegacy => "common/dynasty_legacies/",
            DynastyPerk => "common/dynasty_perks/",
            EducationFocus => "common/focuses/",
            EffectLocalization => "common/effect_localization/",
            Event => "events/",
            EventBackground => "common/event_backgrounds/",
            EventTheme => "common/event_themes/",
            EventTransition => "common/event_transitions/",
            Faith => "common/religion/religions/",
            FaithIcon => "common/religion/religions/",
            Faction => "common/factions/",
            File => "",
            Focus => "common/focuses/",
            GameConcept => "common/game_concepts/",
            GameRule => "common/game_rules/",
            GeneAgePreset => "common/genes/",
            GeneAttribute => "common/genes/",
            GeneCategory => "common/genes/",
            GeneTemplate => "common/genes/",
            GfxPortraitsAccessories => "gfx/portraits/accessories/",
            GovernmentType => "common/governments/",
            GovernmentFlag => "common/governments/",
            GraphicalFaith => "common/religion/religions/",
            GuestSubset => "common/activities/activity_types/",
            Holding => "",
            HolySite => "common/religion/holy_sites/",
            HolySiteFlag => "common/religion/holy_sites/",
            Hook => "common/hook_types/",
            House => "common/dynasty_houses/",
            ImportantAction => "common/important_actions/",
            Innovation => "common/culture/innovations/",
            InnovationFlag => "common/culture/innovations/",
            Inspiration => "common/inspirations/",
            Interaction => "common/character_interactions/",
            InteractionCategory => "common/character_interaction_categories/",
            Language => "common/culture/pillars/",
            Law => "common/laws/",
            LawFlag => "common/laws/",
            Lifestyle => "common/lifestyles/",
            Localization => "localization/",
            MapMode => "gfx/map/map_modes/",
            MemoryCategory => "common/character_memory_types/",
            MemoryType => "common/character_memory_types/",
            MenAtArms => "common/men_at_arms_types/",
            MenAtArmsBase => "common/men_at_arms_types/",
            Modifier => "common/modifiers/",
            Music => "music/",
            NameList => "common/culture/name_lists/",
            Nickname => "common/nicknames/",
            OnAction => "common/on_action/",
            Perk => "common/lifestyle_perks/",
            PerkTree => "common/lifestyle_perks/",
            PrisonType => "",
            Province => "map_data/definition.csv",
            Region => "map_data/geographical_regions/",
            Relation => "common/scripted_relations/",
            RelationFlag => "common/scripted_relations/",
            Religion => "common/religion/religions/",
            ReligiousFamily => "common/religion/religion_families/",
            RewardItem => "",
            Scheme => "common/schemes/",
            ScriptedEffect => "common/scripted_effects/",
            ScriptedGui => "common/scripted_guis/",
            ScriptedList => "common/scripted_lists/",
            ScriptedModifier => "common/scripted_modifiers/",
            ScriptedRule => "common/scripted_rules/",
            ScriptedTrigger => "common/scripted_triggers/",
            ScriptValue => "common/script_values/",
            Secret => "common/secret_types/",
            Sexuality => "",
            Skill => "",
            SpecialBuilding => "common/buildings/",
            Story => "common/story_cycle/",
            Struggle => "common/struggle/struggles/",
            StrugglePhase => "common/struggle/struggles/",
            StrugglePhaseParameter => "common/struggle/struggles/",
            Suggestion => "common/suggestions/",
            Terrain => "common/terrain_types/",
            Title => "common/landed_titles/",
            TitleHistory => "history/titles/",
            TitleHistoryType => "",
            TitleLaw => "common/laws/",
            TitleLawFlag => "common/laws/",
            Tradition => "common/culture/traditions/",
            Trait => "common/traits/",
            TraitCategory => "",
            TraitFlag => "common/traits/",
            TraitTrack => "common/traits/",
            TravelOption => "common/travel/travel_options/",
            TravelPlanModifier => "",
            TriggerLocalization => "common/trigger_localization/",
            UnitGfx => "common/culture/cultures/",
            VassalContractFlag => "common/vassal_contracts/",
            VassalObligation => "common/vassal_contracts/",
            VassalObligationLevel => "common/vassal_contracts/",
            VassalStance => "common/vassal_stances/",
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let s: &'static str = self.into();
        write!(f, "{}", s.replace('_', " "))
    }
}
