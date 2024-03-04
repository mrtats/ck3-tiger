&[
    ("ACCOLADE", Args::Args(&[]), Ck3(Accolade)),
    ("ACTIVE_COUNCIL_TASK", Args::Args(&[]), Ck3(ActiveCouncilTask)),
    ("ACTIVITY", Args::Args(&[]), Ck3(Activity)),
    ("ACTIVITY_INTENT", Args::Args(&[]), Ck3(ActivityIntent)),
    ("ACTIVITY_OPTION", Args::Args(&[]), Ck3(ActivityOption)),
    ("ACTIVITY_OPTION_CATEGORY", Args::Args(&[]), Ck3(ActivityOptionCategory)),
    ("ACTIVITY_PHASE", Args::Args(&[]), Ck3(ActivityPhase)),
    ("ACTIVITY_SPECIAL_GUEST", Args::Args(&[]), Ck3(ActivitySpecialGuestType)),
    ("ACTIVITY_TYPE", Args::Args(&[]), Ck3(ActivityType)),
    ("ACTOR", Args::Args(&[]), Ck3(Character)),
    ("ARMY", Args::Args(&[]), Ck3(Army)),
    ("ARMY_REGIMENT", Args::Args(&[]), Ck3(ArmyRegiment)),
    ("ARTIFACT", Args::Args(&[]), Ck3(Artifact)),
    ("ATTACKER", Args::Args(&[]), Ck3(Character)),
    ("AccessCouncilWindow", Args::Args(&[]), Ck3(CouncilWindow)),
    ("AccessCourtWindow", Args::Args(&[]), Ck3(CourtWindow)),
    ("AccessGameRules", Args::Args(&[]), Ck3(JominiGameRules)),
    ("AccessLocalPlayerCachedData", Args::Args(&[]), Ck3(LocalPlayerCachedData)),
    ("AccessLogViewer", Args::Args(&[]), Ck3(LogViewer)),
    ("AccessMessageFeedHandler", Args::Args(&[]), Ck3(MessageFeedHandler)),
    ("AccessMyRealmWindow", Args::Args(&[]), Ck3(MyRealmWindow)),
    ("AccessTopBar", Args::Args(&[]), Ck3(InGameTopbar)),
    ("Application", Args::Args(&[]), Ck3(Application)),
    ("BOOKMARK_CHARACTER", Args::Args(&[]), Ck3(BookmarkCharacter)),
    ("BUILDING", Args::Args(&[]), Ck3(Building)),
    ("CHARACTER", Args::Args(&[]), Ck3(Character)),
    ("CHARACTER_MEMORY", Args::Args(&[]), Ck3(CharacterMemory)),
    ("CHECK_DATE", Args::Args(&[]), Date),
    ("CLAIMANT", Args::Args(&[]), Ck3(Character)),
    ("COUNCIL_POSITION_TYPE", Args::Args(&[]), Ck3(CouncilPositionType)),
    ("COUNCIL_TASK", Args::Args(&[]), Ck3(ActiveCouncilTask)),
    ("COUNCIL_TASK_TYPE", Args::Args(&[]), Ck3(CouncilTaskType)),
    ("COUNTY", Args::Args(&[]), Ck3(County)),
    ("COURT_POSITION", Args::Args(&[]), Ck3(CourtPosition)),
    ("COURT_POSITION_TASK_TYPE", Args::Args(&[]), Ck3(CourtPositionTaskType)),
    ("COURT_POSITION_TYPE", Args::Args(&[]), Ck3(CourtPositionType)),
    ("CULTURE", Args::Args(&[]), Ck3(Culture)),
    ("CULTURE_PILLAR", Args::Args(&[]), Ck3(CulturePillar)),
    ("CULTURE_TRADITION", Args::Args(&[]), Ck3(CultureTradition)),
    ("DATE", Args::Args(&[]), Date),
    ("DATE_MAX", Args::Args(&[]), Date),
    ("DATE_MIN", Args::Args(&[]), Date),
    ("DECISION_TYPE", Args::Args(&[]), Ck3(Decision)),
    ("DEFENDER", Args::Args(&[]), Ck3(Character)),
    ("DYNASTY", Args::Args(&[]), Ck3(Dynasty)),
    ("DYNASTY_HOUSE", Args::Args(&[]), Ck3(DynastyHouse)),
    ("DYNASTY_LEGACY", Args::Args(&[]), Ck3(DynastyLegacy)),
    ("DYNASTY_PERK", Args::Args(&[]), Ck3(DynastyPerk)),
    ("EPIDEMIC", Args::Args(&[]), Ck3(Epidemic)),
    ("EPIDEMIC_TYPE", Args::Args(&[]), Ck3(EpidemicType)),
    ("ERA", Args::Args(&[]), Ck3(CultureEra)),
    ("EmptyScope", Args::Args(&[]), TopScope),
    ("FACTION", Args::Args(&[]), Ck3(Faction)),
    ("FAITH", Args::Args(&[]), Ck3(Faith)),
    ("FAITH_DOCTRINE", Args::Args(&[]), Ck3(FaithDoctrine)),
    ("FOCUS", Args::Args(&[]), Ck3(FocusType)),
    ("GAME_RULE", Args::Args(&[]), Ck3(GameRule)),
    ("GAME_RULE_SETTING", Args::Args(&[]), Ck3(GameRuleSetting)),
    ("GEOGRAPHICAL_REGION", Args::Args(&[]), Ck3(GeographicalRegion)),
    ("GOVERNMENT_TYPE", Args::Args(&[]), Ck3(GovernmentType)),
    ("GREAT_HOLY_WAR", Args::Args(&[]), Ck3(GreatHolyWar)),
    ("GetAccoladeType", Args::Args(&[IType(Item::AccoladeType)]), Ck3(AccoladeType)),
    ("GetActivityGuestInviteRule", Args::Args(&[IType(Item::GuestInviteRule)]), Ck3(ActivityGuestInviteRule)),
    ("GetActivityIntent", Args::Args(&[IType(Item::ActivityIntent)]), Ck3(ActivityIntent)),
    ("GetActivityLocale", Args::Args(&[IType(Item::ActivityLocale)]), Ck3(ActivityLocale)),
    ("GetActivityPulseAction", Args::Args(&[IType(Item::PulseAction)]), Ck3(ActivityPulseAction)),
    ("GetActivityType", Args::Args(&[IType(Item::ActivityType)]), Ck3(ActivityType)),
    ("GetArtifactType", Args::Args(&[IType(Item::ArtifactType)]), Ck3(ArtifactType)),
    ("GetArtifactVisualType", Args::Args(&[IType(Item::ArtifactVisual)]), Ck3(ArtifactVisualType)),
    ("GetBookmark", Args::Args(&[IType(Item::Bookmark)]), Ck3(Bookmark)),
    ("GetBookmarkGroup", Args::Args(&[IType(Item::BookmarkGroup)]), Ck3(BookmarkGroup)),
    ("GetBookmarkPortrait", Args::Args(&[DType(Unknown)]), Ck3(BookmarkPortrait)),
    ("GetBuilding", Args::Args(&[IType(Item::Building)]), Ck3(Building)),
    ("GetCasusBelliType", Args::Args(&[IType(Item::CasusBelli)]), Ck3(CasusBelliType)),
    ("GetCatalystType", Args::Args(&[IType(Item::Catalyst)]), Ck3(CatalystType)),
    ("GetCharacterInteraction", Args::Args(&[IType(Item::CharacterInteraction)]), Ck3(CharacterInteraction)),
    ("GetCharacterInteractionCategory", Args::Args(&[IType(Item::CharacterInteractionCategory)]), Ck3(CharacterInteractionCategory)),
    ("GetCharacterInteractionCategoryByIndex", Args::Args(&[DType(Unknown)]), Ck3(CharacterInteractionCategory)),
    ("GetCharacterMemoryType", Args::Args(&[DType(Unknown)]), Ck3(CharacterMemoryType)),
    ("GetCombatEffect", Args::Args(&[DType(Unknown)]), Ck3(CombatEffect)),
    ("GetCouncilPositionType", Args::Args(&[DType(Unknown)]), Ck3(CouncilPositionType)),
    ("GetCouncilTaskType", Args::Args(&[DType(Unknown)]), Ck3(CouncilTaskType)),
    ("GetCourtAmenitiesSetting", Args::Args(&[DType(Unknown)]), Ck3(CourtAmenitiesSetting)),
    ("GetCourtGrandeurDiffFromExpectedLevelModifier", Args::Args(&[DType(Unknown)]), Ck3(StaticModifier)),
    ("GetCourtGrandeurLevelModifier", Args::Args(&[DType(Unknown)]), Ck3(StaticModifier)),
    ("GetCourtPositionCategory", Args::Args(&[DType(Unknown)]), Ck3(CourtPositionCategory)),
    ("GetCourtPositionTaskType", Args::Args(&[DType(Unknown)]), Ck3(CourtPositionTaskType)),
    ("GetCourtPositionType", Args::Args(&[DType(Unknown)]), Ck3(CourtPositionType)),
    ("GetCourtType", Args::Args(&[DType(Unknown)]), Ck3(CourtType)),
    ("GetCultureEraType", Args::Args(&[DType(Unknown)]), Ck3(CultureEraType)),
    ("GetCultureInnovationType", Args::Args(&[DType(Unknown)]), Ck3(CultureInnovationType)),
    ("GetCulturePillar", Args::Args(&[DType(Unknown)]), Ck3(CulturePillar)),
    ("GetCultureTemplate", Args::Args(&[IType(Item::Culture)]), Ck3(CultureTemplate)),
    ("GetCultureTradition", Args::Args(&[DType(Unknown)]), Ck3(CultureTradition)),
    ("GetCurrentDate", Args::Args(&[]), Date),
    ("GetCurrentMapMode", Args::Args(&[]), Ck3(MapMode)),
    ("GetDecision", Args::Args(&[DType(Unknown)]), Ck3(Decision)),
    ("GetDecisionWithKey", Args::Args(&[IType(Item::Decision)]), Ck3(Decision)),
    ("GetDeepestTooltipInfo", Args::Args(&[]), Ck3(TooltipInfo)),
    ("GetDlc", Args::Args(&[DType(Unknown)]), Ck3(Dlc)),
    ("GetDoctrine", Args::Args(&[IType(Item::Doctrine)]), Ck3(FaithDoctrine)),
    ("GetDynastyByID", Args::Args(&[DType(Unknown)]), Ck3(Dynasty)),
    ("GetDynastyHouseByID", Args::Args(&[DType(Unknown)]), Ck3(DynastyHouse)),
    ("GetDynastyHouseTemplate", Args::Args(&[DType(Unknown)]), Ck3(DynastyHouseTemplate)),
    ("GetDynastyLegacy", Args::Args(&[DType(Unknown)]), Ck3(DynastyLegacy)),
    ("GetDynastyPerk", Args::Args(&[DType(Unknown)]), Ck3(DynastyPerk)),
    ("GetDynastyTemplate", Args::Args(&[DType(Unknown)]), Ck3(DynastyTemplate)),
    ("GetEpidemicType", Args::Args(&[DType(Unknown)]), Ck3(EpidemicType)),
    ("GetFaithByKey", Args::Args(&[IType(Item::Faith)]), Ck3(Faith)),
    ("GetFaithDoctrine", Args::Args(&[IType(Item::Doctrine)]), Ck3(FaithDoctrine)),
    ("GetFaithDoctrineGroup", Args::Args(&[DType(Unknown)]), Ck3(FaithDoctrineGroup)),
    ("GetFocus", Args::Args(&[IType(Item::Focus)]), Ck3(FocusType)),
    ("GetFocusType", Args::Args(&[DType(Unknown)]), Ck3(FocusType)),
    ("GetGameRules", Args::Args(&[]), Ck3(JominiGameRules)),
    ("GetGeographicalRegion", Args::Args(&[IType(Item::Region)]), Ck3(GeographicalRegion)),
    ("GetGlobalVariable", Args::Args(&[DType(Unknown)]), Scope),
    ("GetGovernment", Args::Args(&[DType(Unknown)]), Ck3(GovernmentType)),
    ("GetGovernmentType", Args::Args(&[DType(Unknown)]), Ck3(GovernmentType)),
    ("GetHoldingType", Args::Args(&[DType(Unknown)]), Ck3(HoldingType)),
    ("GetHolySite", Args::Args(&[IType(Item::HolySite)]), Ck3(HolySite)),
    ("GetHouseUnityStage", Args::Args(&[DType(Unknown)]), Ck3(HouseUnityStage)),
    ("GetIllustration", Args::Args(&[IType(Item::ScriptedIllustration)]), Ck3(Illustration)),
    ("GetImportantActionType", Args::Args(&[DType(Unknown)]), Ck3(ImportantActionType)),
    ("GetInspirationType", Args::Args(&[DType(Unknown)]), Ck3(InspirationType)),
    ("GetInventorySlotType", Args::Args(&[DType(Unknown)]), Ck3(InventorySlotType)),
    ("GetLandedTitpleTemplate", Args::Args(&[DType(Unknown)]), Ck3(LandedTitpleTemplate)),
    ("GetLaw", Args::Args(&[DType(Unknown)]), Ck3(Law)),
    ("GetLawGroup", Args::Args(&[DType(Unknown)]), Ck3(LawGroup)),
    ("GetLegendType", Args::Args(&[DType(Unknown)]), Ck3(LegendType)),
    ("GetLegitimacyType", Args::Args(&[DType(Unknown)]), Ck3(LegitimacyType)),
    ("GetLifestyle", Args::Args(&[IType(Item::Lifestyle)]), Ck3(Lifestyle)),
    ("GetMaA", Args::Args(&[IType(Item::MenAtArms)]), Ck3(MenAtArmsType)),
    ("GetMandate", Args::Args(&[DType(Unknown)]), Ck3(Mandate)),
    ("GetMapMode", Args::Args(&[DType(Unknown)]), Ck3(MapMode)),
    ("GetMenAtArmsType", Args::Args(&[DType(Unknown)]), Ck3(MenAtArmsType)),
    ("GetMessageFeedHandler", Args::Args(&[]), Ck3(MessageFeedHandler)),
    ("GetMessageType", Args::Args(&[DType(Unknown)]), Ck3(MessageType)),
    ("GetModifier", Args::Args(&[IType(Item::Modifier)]), Ck3(StaticModifier)),
    ("GetNickname", Args::Args(&[IType(Item::Nickname)]), Ck3(Nickname)),
    ("GetNullCharacter", Args::Args(&[]), Ck3(Character)),
    ("GetNullProvince", Args::Args(&[]), Ck3(Province)),
    ("GetPerk", Args::Args(&[IType(Item::Perk)]), Ck3(Perk)),
    ("GetPlayer", Args::Args(&[]), Ck3(Character)),
    ("GetPlayerArmyComposition", Args::Args(&[]), Ck3(ArmyComposition)),
    ("GetRelation", Args::Args(&[IType(Item::Relation)]), Ck3(ScriptedRelation)),
    ("GetReligionByKey", Args::Args(&[IType(Item::Religion)]), Ck3(Religion)),
    ("GetReligionFamily", Args::Args(&[DType(Unknown)]), Ck3(ReligionFamily)),
    ("GetRoyalCourtHoveredArtifact", Args::Args(&[]), Ck3(Artifact)),
    ("GetRoyalCourtHoveredCharacter", Args::Args(&[]), Ck3(Character)),
    ("GetRoyalCourtSelectedArtifact", Args::Args(&[]), Ck3(Artifact)),
    ("GetRoyalCourtSelectedCharacter", Args::Args(&[]), Ck3(Character)),
    ("GetScheme", Args::Args(&[IType(Item::Scheme)]), Ck3(SchemeType)),
    ("GetSchemeType", Args::Args(&[IType(Item::Scheme)]), Ck3(SchemeType)),
    ("GetScriptedCharacterByHistoryID", Args::Args(&[DType(Unknown)]), Ck3(Character)),
    ("GetScriptedGui", Args::Args(&[IType(Item::ScriptedGui)]), Ck3(ScriptedGui)),
    ("GetScriptedRelation", Args::Args(&[IType(Item::Relation)]), Ck3(ScriptedRelation)),
    ("GetSecretType", Args::Args(&[IType(Item::Secret)]), Ck3(SecretType)),
    ("GetServerInfo", Args::Args(&[]), Ck3(ServerInformation)),
    ("GetStaticModifier", Args::Args(&[IType(Item::Modifier)]), Ck3(StaticModifier)),
    ("GetStruggle", Args::Args(&[DType(Unknown)]), Ck3(Struggle)),
    ("GetStrugglePhase", Args::Args(&[DType(Unknown)]), Ck3(StrugglePhase)),
    ("GetStruggleType", Args::Args(&[DType(Unknown)]), Ck3(StruggleType)),
    ("GetSuggestionType", Args::Args(&[DType(Unknown)]), Ck3(SuggestionType)),
    ("GetTaxSlotObligation", Args::Args(&[DType(Unknown)]), Ck3(TaxSlotObligation)),
    ("GetTaxSlotType", Args::Args(&[DType(Unknown)]), Ck3(TaxSlotType)),
    ("GetTerrain", Args::Args(&[IType(Item::Terrain)]), Ck3(Terrain)),
    ("GetTitleByKey", Args::Args(&[IType(Item::Title)]), Ck3(Title)),
    ("GetTrait", Args::Args(&[IType(Item::Trait)]), Ck3(Trait)),
    ("GetTraitForIndex", Args::Args(&[DType(Unknown)]), Ck3(Trait)),
    ("GetTraitGroup", Args::Args(&[DType(Unknown)]), Ck3(TraitGroup)),
    ("GetTravelOption", Args::Args(&[DType(Unknown)]), Ck3(TravelOption)),
    ("GetTravelPointOfInterestType", Args::Args(&[DType(Unknown)]), Ck3(TravelPointOfInterestType)),
    ("GetTriggeredLegendSeed", Args::Args(&[DType(Unknown)]), Ck3(TriggeredLegendSeed)),
    ("GetTutorial", Args::Args(&[]), Ck3(Tutorial)),
    ("GetVariableSystem", Args::Args(&[]), Ck3(VariableSystem)),
    ("GetVassalContractType", Args::Args(&[DType(Unknown)]), Ck3(VassalContractType)),
    ("GetVassalStance", Args::Args(&[IType(Item::VassalStance)]), Ck3(VassalStance)),
    ("GuiEditor", Args::Args(&[]), Ck3(GuiEditor)),
    ("GuiScope", Args::Args(&[]), TopScope),
    ("HISTORY_ENTRY", Args::Args(&[]), Ck3(HistoryEntry)),
    ("HOLDING", Args::Args(&[]), Ck3(Holding)),
    ("HOLDING_TYPE", Args::Args(&[]), Ck3(HoldingType)),
    ("HOLY_ORDER", Args::Args(&[]), Ck3(HolyOrder)),
    ("HOOK", Args::Args(&[]), Ck3(Hook)),
    ("IMPORTANT_ACTION_ITEM", Args::Args(&[]), Ck3(ImportantActionItem)),
    ("INNOVATION", Args::Args(&[]), Ck3(CultureInnovation)),
    ("INNOVATION_TYPE", Args::Args(&[]), Ck3(CultureInnovationType)),
    ("INSPIRATION", Args::Args(&[]), Ck3(Inspiration)),
    ("INTERMEDIARY", Args::Args(&[]), Ck3(Character)),
    ("JominiPlayer", Args::Args(&[]), Ck3(Playable)),
    ("LAW", Args::Args(&[]), Ck3(Law)),
    ("LEFT_ARTIFACT", Args::Args(&[]), Ck3(Artifact)),
    ("LEFT_CHARACTER", Args::Args(&[]), Ck3(Character)),
    ("LEFT_TITLE", Args::Args(&[]), Ck3(Title)),
    ("LEGEND", Args::Args(&[]), Ck3(Legend)),
    ("LEGEND_SEED", Args::Args(&[]), Ck3(LegendSeed)),
    ("LEGEND_TYPE", Args::Args(&[]), Ck3(LegendType)),
    ("LEGITIMACY_LEVEL", Args::Args(&[]), Ck3(LegitimacyLevel)),
    ("LEGITIMACY_TYPE", Args::Args(&[]), Ck3(LegitimacyType)),
    ("LIFESTYLE", Args::Args(&[]), Ck3(Lifestyle)),
    ("MEN_AT_ARMS_TYPE", Args::Args(&[]), Ck3(MenAtArmsType)),
    ("MEN_AT_ARMS_TYPE_2", Args::Args(&[]), Ck3(MenAtArmsType)),
    ("MERCENARY", Args::Args(&[]), Ck3(MercenaryCompany)),
    ("MODIFIER_ITEM", Args::Args(&[]), Ck3(ModifierItem)),
    ("PERK", Args::Args(&[]), Ck3(Perk)),
    ("PLAYER_MESSAGE_ITEM", Args::Args(&[]), Ck3(PlayerMessageItem)),
    ("PREV", Args::Args(&[]), Scope),
    ("PROVINCE", Args::Args(&[]), Ck3(Province)),
    ("RECIPIENT", Args::Args(&[]), Ck3(Character)),
    ("REGIMENT", Args::Args(&[]), Ck3(Regiment)),
    ("RELATION", Args::Args(&[]), Ck3(ScriptedRelation)),
    ("RELIGION", Args::Args(&[]), Ck3(Religion)),
    ("RIGHT_ARTIFACT", Args::Args(&[]), Ck3(Artifact)),
    ("RIGHT_CHARACTER", Args::Args(&[]), Ck3(Character)),
    ("RIGHT_TITLE", Args::Args(&[]), Ck3(Title)),
    ("ROOT", Args::Args(&[]), Scope),
    ("Root", Args::Args(&[]), Scope),
    ("SCHEME", Args::Args(&[]), Ck3(Scheme)),
    ("SCOPE", Args::Args(&[]), TopScope),
    ("SECONDARY_ACTOR", Args::Args(&[]), Ck3(Character)),
    ("SECONDARY_RECIPIENT", Args::Args(&[]), Ck3(Character)),
    ("SECRET", Args::Args(&[]), Ck3(Secret)),
    ("SIEGE", Args::Args(&[]), Ck3(Siege)),
    ("STRUGGLE", Args::Args(&[]), Ck3(Struggle)),
    ("TARGET_ARMY", Args::Args(&[]), Ck3(Army)),
    ("TARGET_CHARACTER", Args::Args(&[]), Ck3(Character)),
    ("TARGET_CHARACTER_2", Args::Args(&[]), Ck3(Character)),
    ("TARGET_CULTURE", Args::Args(&[]), Ck3(Culture)),
    ("TARGET_FAITH", Args::Args(&[]), Ck3(Faith)),
    ("TARGET_FAITH_2", Args::Args(&[]), Ck3(Faith)),
    ("TARGET_LAW", Args::Args(&[]), Ck3(Law)),
    ("TARGET_TITLE", Args::Args(&[]), Ck3(Title)),
    ("TAX_SLOT_TYPE", Args::Args(&[]), Ck3(TaxSlotType)),
    ("THIS", Args::Args(&[]), Scope),
    ("TITLE", Args::Args(&[]), Ck3(Title)),
    ("TITLE_2", Args::Args(&[]), Ck3(Title)),
    ("TOKEN_PARAMETER", Args::Args(&[]), Ck3(TokenParameter)),
    ("TRAIT", Args::Args(&[]), Ck3(Trait)),
    ("TRAIT_2", Args::Args(&[]), Ck3(Trait)),
    ("TRAIT_GROUP", Args::Args(&[]), Ck3(TraitGroup)),
    ("TRAIT_GROUP_2", Args::Args(&[]), Ck3(TraitGroup)),
    ("TRAIT_LEVEL_TRACK", Args::Args(&[]), Ck3(TraitLevelTrack)),
    ("TRAIT_LEVEL_TRACK_ENTRY", Args::Args(&[]), Ck3(TraitLevelTrackEntry)),
    ("TRAVEL_PLAN", Args::Args(&[]), Ck3(TravelPlan)),
    ("TRAVEL_PLAN_COMPANION", Args::Args(&[]), Ck3(TravelPlanCompanion)),
    ("TRIGGERED_LEGEND_SEED", Args::Args(&[]), Ck3(TriggeredLegendSeed)),
    ("TUTORIAL", Args::Args(&[]), Ck3(Tutorial)),
    ("VASSAL_CONTRACT_OBLIGATION_LEVEL", Args::Args(&[]), Ck3(ObligationLevel)),
    ("VASSAL_CONTRACT_TYPE", Args::Args(&[]), Ck3(VassalContractType)),
    ("VASSAL_STANCE", Args::Args(&[]), Ck3(VassalStance)),
    ("WAR", Args::Args(&[]), Ck3(War)),
]
