use strum_macros::IntoStaticStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoStaticStr, Hash, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
#[cfg(feature = "vic3")]
pub enum Item {
    Attitude,
    BattleCondition,
    BuildingGroup,
    BuildingType,
    CanalType,
    Country,
    Culture,
    CustomLocalization,
    Define,
    Dlc,
    DlcFeature,
    EffectLocalization,
    Ethnicity,
    Event,
    EventNamespace,
    File,
    GameConcept,
    Goods,
    Ideology,
    Institution,
    InterestGroup,
    Law,
    Localization,
    MediaAlias,
    Modifier,
    NamedColor,
    PopType,
    ProductionMethod,
    ProductionMethodGroup,
    Province,
    Religion,
    ScriptedEffect,
    ScriptedGui,
    ScriptedList,
    ScriptedModifier,
    ScriptedTrigger,
    ScriptValue,
    Sound,
    StateRegion,
    StrategicRegion,
    Technology,
    TechnologyEra,
    TerrainManipulator,
    TriggerLocalization,
}

impl Item {
    #[cfg(feature = "vic3")]
    pub fn path(self) -> &'static str {
        #[allow(clippy::match_same_arms)]
        match self {
            Item::Attitude => "",
            Item::BattleCondition => "common/battle_conditions/",
            Item::BuildingGroup => "common/building_groups/",
            Item::BuildingType => "common/buildings/",
            Item::CanalType => "common/canals/",
            Item::Country => "common/country_definitions/",
            Item::Culture => "common/cultures/",
            Item::CustomLocalization => "common/customizable_localization/",
            Item::Define => "common/defines/",
            Item::Dlc => "",
            Item::DlcFeature => "",
            Item::EffectLocalization => "common/effect_localization/",
            Item::Ethnicity => "common/ethnicities/",
            Item::Event => "events/",
            Item::EventNamespace => "events/",
            Item::File => "",
            Item::GameConcept => "common/game_concepts/",
            Item::Goods => "common/goods/",
            Item::Ideology => "common/ideologies/",
            Item::Institution => "common/institutions/",
            Item::InterestGroup => "common/interest_groups/",
            Item::Law => "common/laws/",
            Item::Localization => "localization/",
            Item::MediaAlias => "gfx/media_aliases",
            Item::Modifier => "common/modifiers/",
            Item::NamedColor => "common/named_colors/",
            Item::PopType => "common/pop_types/",
            Item::ProductionMethod => "common/production_methods/",
            Item::ProductionMethodGroup => "common/production_method_groups/",
            Item::Province => "map_data/provinces.png",
            Item::Religion => "common/religions/",
            Item::ScriptedEffect => "common/scripted_effects/",
            Item::ScriptedGui => "common/scripted_guis/",
            Item::ScriptedList => "common/scripted_lists/",
            Item::ScriptedModifier => "common/scripted_modifiers/",
            Item::ScriptedTrigger => "common/scripted_triggers/",
            Item::ScriptValue => "common/script_values/",
            Item::Sound => "",
            Item::StateRegion => "map_data/state_regions/",
            Item::StrategicRegion => "common/strategic_regions/",
            Item::Technology => "common/technology/",
            Item::TechnologyEra => "common/technology/era/",
            Item::TerrainManipulator => "common/terrain_manipulators/",
            Item::TriggerLocalization => "common/trigger_localization/",
        }
    }
}