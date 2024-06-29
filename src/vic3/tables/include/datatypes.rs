#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Display, EnumString)]
#[strum(use_phf)]
pub enum Vic3Datatype {
    AIAttitude,
    AIStrategy,
    Achievement,
    AchievementPopup,
    AchievementWindow,
    AddFriendWindow,
    AddWarGoalPanel,
    AgitatorSlot,
    Alert,
    AlertGroup,
    AlertOptionItem,
    AlertSettingsItem,
    AnchorItem,
    Application,
    Attribute,
    AvailabilityEntry,
    Battle,
    BattleBuildingEntry,
    BattleCondition,
    BattleMarker,
    BattleMilitaryFormation,
    BattlePanel,
    BattleParticipant,
    BattleParticipantsPanel,
    BattleUnitType,
    BlurThreshold,
    BrushBool,
    BrushFloat,
    BrushSettings,
    BrushSettingsDropdown,
    BrushSettingsGlobal,
    BudgetPanel,
    Building,
    BuildingBrowserBuildingTypeItem,
    BuildingBrowserPanel,
    BuildingDetailsPanel,
    BuildingMarker,
    BuildingOwnership,
    BuildingPotentialMarker,
    BuildingProductionMethodsEntry,
    BuildingType,
    CPdxEnumValue,
    CPdxFloatRect,
    CPdxInputBindingSetting,
    CPdxIntRect,
    CanalType,
    ChangeLawPanel,
    ChangeProductionMethodMenuItem,
    ChangeProductionMethodsMenuItems,
    Character,
    CharacterInteraction,
    CharacterTrait,
    Chat,
    ChatMessage,
    ChatNotificationMessage,
    ChatTab,
    ChatWindow,
    ChildGenerator,
    ChildItem,
    Cities,
    CityMarker,
    CivilWar,
    CloudSaveData,
    CoatOfArms,
    CoatOfArmsPreviewWindow,
    CohesionLevel,
    ColonyMarker,
    ColorPicker,
    CombatUnit,
    CombatUnitGroup,
    CombatUnitType,
    Command,
    CommanderOrderType,
    CommanderPanel,
    CommanderRank,
    Company,
    CompanyChangeNamePopup,
    CompanyType,
    ConfirmationWindow,
    ConsoleMenuItem,
    ConsoleWindow,
    ConstructionPanel,
    ConstructionQueueElement,
    Container,
    ContextMenuItem,
    ContextualDiplomaticActionType,
    ContextualDiplomaticPact,
    CountriesPanel,
    Country,
    CountryCreation,
    CountryDefinition,
    CountryEntry,
    CountryFormation,
    CountryFormationPanel,
    CountryMarker,
    CountryPanel,
    CountryTrendPair,
    CreateSocialProfile,
    CreateSocialProfileWindow,
    CreditsWindow,
    CulturalCommunity,
    Culture,
    CultureCasualtyStatistics,
    CultureInfoPanel,
    CurveEditor,
    CurvePoint,
    DataColorPicker,
    DataDynamicTrend,
    DataTrend,
    DatatypesExplorer,
    Decision,
    Decree,
    DecreeMarker,
    DecreeType,
    DefineProxyNodeWindow,
    DetailData,
    DiplomaticAction,
    DiplomaticActionType,
    DiplomaticActionWindow,
    DiplomaticCatalyst,
    DiplomaticCatalystCategory,
    DiplomaticCatalystType,
    DiplomaticDemand,
    DiplomaticOverviewPanel,
    DiplomaticPact,
    DiplomaticPlay,
    DiplomaticPlayConfirmation,
    DiplomaticPlayPanel,
    DiplomaticPlayType,
    DiplomaticRelations,
    DiscriminationTrait,
    Dlc,
    DockableLayout,
    DockableLayoutManager,
    DockableWindow,
    DrawCmdsList,
    DrawCmdsViewer,
    EditorSettingCategory,
    EditorSettingsPage,
    EditorSettingsWindow,
    Election,
    ElectionPanel,
    EmitterNodeWindow,
    EmployeeTransfer,
    Encyclopedia,
    EncyclopediaEntry,
    EncyclopediaEntryView,
    EncyclopediaPage,
    EndPrepConfirm,
    EntityEditor,
    EntityEditorAudioEventHandler,
    EntityEditorEventLayer,
    EntityEditorEventLayerPtr,
    EntityEditorKeyframe,
    EntityEditorTimelineState,
    EntityViewerProperties,
    EnumSettingEntry,
    Era,
    ErrorDeer,
    ErrorMessageBox,
    Ethnicity,
    EthnicityItem,
    Event,
    EventIcon,
    EventLayerForEntityEditor,
    EventMarker,
    EventMarkerListItem,
    EventOption,
    EventTargetSetupContext,
    EventWindow,
    ExilePool,
    ExportTool,
    EyeDropper,
    EyeDropperPackedSample,
    FeedMessageItem,
    FilterablePropertyList,
    FormationMarker,
    FormationPanel,
    Friend,
    FriendListWindow,
    FriendRequest,
    FriendSearchResult,
    Friends,
    Front,
    FrontEndCreditsView,
    FrontEndLoadConfirmationWindow,
    FrontEndLoadView,
    FrontEndMainView,
    FrontEndMultiplayerView,
    FrontMarker,
    FrontPanel,
    FrontParticipant,
    GUIAchievement,
    GameConceptType,
    GameMpSetup,
    GameOverScreen,
    GameRule,
    GameRuleSetting,
    GameSetup,
    GeneCategory,
    GeneItem,
    GeneTemplate,
    GenerationItem,
    GfxSkin,
    Goods,
    GoodsExpenseItem,
    GoodsLocalPricesPanel,
    GoodsPanel,
    GoodsPanelValue,
    GoodsProductionInfo,
    GoodsStatePanel,
    GoodsUsagePanel,
    GovernmentType,
    Graph,
    GraphInterfaceNodeWindow,
    GraphPanel,
    Group,
    GuiAnimationCurveEditor,
    GuiAnimationCurveEditorControlPoint,
    GuiAnimationCurveEditorLine,
    GuiAnimationCurveEditorViewport,
    GuiAnimationEditor,
    GuiAnimationEditorAnimSetEntry,
    GuiAnimationEditorAnimationEntry,
    GuiAnimationEditorAvailableTrack,
    GuiAnimationEditorKeyframe,
    GuiAnimationEditorMetadataCtx,
    GuiAnimationEditorPlayer,
    GuiAnimationEditorPlayerSpeedMultiplierEntry,
    GuiAnimationEditorRuler,
    GuiAnimationEditorRulerResolutionEntry,
    GuiAnimationEditorUniversalTrack,
    GuiAnimationEditorViewportBase,
    GuiAnimationEditorViewportUserInput,
    GuiAnimationTimelineViewport,
    GuiContext,
    GuiCurrentDrag,
    GuiEditor,
    GuiEditorCategory,
    GuiEditorDockable,
    GuiEditorOutliner,
    GuiEditorProperties,
    GuiEditorProperty,
    GuiEditorTooltip,
    GuiGameRule,
    GuiGameRulePreset,
    HQMarker,
    Heightmap,
    HeightmapPainter,
    HeightmapPainterMode,
    HeightmapResolution,
    HighlightManager,
    HistoryViewer,
    HomelandMarker,
    Hq,
    Ideology,
    ImportTool,
    Importable,
    ImportableGroup,
    ImportantAlert,
    InfoboxNodeWindow,
    InformationPanel,
    InformationPanelBar,
    IngameHud,
    InputActionBinding,
    InspectorPanel,
    Institution,
    InstitutionInvestmentLevel,
    InstitutionType,
    Interest,
    InterestGroup,
    InterestGroupAndTraitPair,
    InterestGroupMarker,
    InterestGroupPanel,
    InterestGroupTrait,
    InterestingCountryItem,
    InterestingCountrySetting,
    JominiGUISetting,
    JominiGameRules,
    JominiNotification,
    JominiNotificationOverlay,
    JominiPasswordPopup,
    JominiServer,
    JominiServerBrowserGui,
    JominiSettingsWindow,
    JournalEntry,
    JournalEntryGroup,
    JournalEntryPanel,
    JournalEntryType,
    JournalPanel,
    KeyframeEditor,
    KeyframeEventEditor,
    KeyframeWidget,
    Label,
    LabelingHelper,
    LanguageEntry,
    Law,
    LawGroup,
    LawType,
    Layer,
    LayerTreeItem,
    LegitimacyLevel,
    LensOption,
    LensTab,
    LensToolbar,
    LibertyDesireLevel,
    LoadIngameWindow,
    LoadingScreenManager,
    LobbyPlayer,
    LobbyView,
    Location,
    LocationFinder,
    LogEntry,
    LogViewer,
    LogViewerCategory,
    LogViewerEntry,
    LogViewerType,
    LoyaltyType,
    MPConfig,
    MapContentEditorMode,
    MapContentEditorOptions,
    MapContentEditorViewport,
    MapContentEntryDesc,
    MapContentLayerDesc,
    MapContentPanel,
    MapContentPropertyGroup,
    MapContentPropertyGroupsGui,
    MapContentSelector,
    MapContentSelectorGui,
    MapEditor,
    MapEditorGui,
    MapEditorLayerBorder,
    MapEditorLayerBorderDockable,
    MapInteraction,
    MapInteractionManager,
    MapListActivateConscriptionCenterOption,
    MapListActivateConscriptionCenterPanel,
    MapListAdvanceFrontOption,
    MapListAdvanceFrontPanel,
    MapListBuildingOption,
    MapListBuildingPanel,
    MapListColonyOption,
    MapListCountriesPanel,
    MapListCountryOption,
    MapListDecreeOption,
    MapListDecreePanel,
    MapListDeployMilitaryFormationToFrontOption,
    MapListDeployMilitaryFormationToFrontPanel,
    MapListDiploActionOption,
    MapListInterestGroupOption,
    MapListInterestGroupsPanel,
    MapListMarketOption,
    MapListMarketsPanel,
    MapListMilitaryFormationOption,
    MapListMilitaryFormationPanel,
    MapListOption,
    MapListPanel,
    MapListPanelManager,
    MapListPowerBlocInvitationPanel,
    MapListPowerBlocOption,
    MapListPowerBlocsPanel,
    MapListStateOption,
    MapListStatePanel,
    MapListStrategicRegionOption,
    MapListStrategicRegionsPanel,
    MapListTradeRouteOption,
    MapNotification,
    MapObjectMask,
    MapObjectPainter,
    MapObjectPainterMode,
    MapObjectPainterOptions,
    MapObjectTool,
    Market,
    MarketGoods,
    MarketPanel,
    MarketsMarker,
    MarketsMarkerListItem,
    MaskEntry,
    MaskManagerEntry,
    MaskPainterDynamicTerrain,
    MaskPainterManager,
    MaskPainterMapContentPanel,
    MaskPainterMode,
    MaskPainterTool,
    MaskPainterViewport,
    Material,
    MaterialBrowser,
    MaterialEntry,
    MaterialMix,
    MaterialMixBrush,
    MaterialMixEntry,
    MaterialPaintingMode,
    Materials,
    MaterialsSample,
    MessageFeedHandler,
    MessageSettingsWindow,
    MilitaryFormation,
    MilitaryFormationChangeNamePopup,
    MilitaryFormationColor,
    MilitaryFormationFlag,
    MilitaryFormationUnitEntry,
    MilitaryPanelBuildingEntry,
    MixBrushMode,
    MobilizationOption,
    MobilizationOptionGroup,
    Modifier,
    ModifierBreakdown,
    ModifierEntry,
    ModifierNodeData,
    ModifierNodeDebuggerView,
    ModifierNodeDetailsView,
    ModifierNodeGraphItem,
    ModifierNodeGraphLine,
    ModifierNodeListView,
    ModifiersPanel,
    MoveTool,
    MultiplayerSetupWindow,
    MultiplayerSynchronizationInfo,
    MusicPlayer,
    MusicPlayerCategory,
    MusicTrack,
    NationalizeBuildingWindow,
    NavalInvasion,
    NavalInvasionMarker,
    NavalInvasionPanel,
    NavalInvasionPlannerPopup,
    Node,
    NodeEditorSearch,
    NodeError,
    NodeLine,
    NodePin,
    NodeWindow,
    NonRegisteredDockable,
    NotificationDummyContext,
    NotificationOptionItem,
    NotificationSettingsItem,
    Nudger,
    NudgerLayerEntryMapObjectDesc,
    NudgerMapContentGui,
    NudgerMapObjectPropertyListDockable,
    NudgerMode,
    ObjectBrowser,
    ObjectBrowserView,
    ObjectInspector,
    ObjectInspectorDockable,
    ObjectInspectorPlugin,
    ObjectPreset,
    ObjectProvider,
    Objective,
    ObjectiveType,
    OccupationPiechartData,
    OosData,
    OosWindow,
    OpinionMarker,
    OutgoingFriendRequest,
    Outliner,
    OutlinerEntry,
    OutputEntry,
    POPSCreateAccount,
    POPSLoginView,
    POPSStatusWidget,
    PanelMilitary,
    ParametricSelect,
    ParticleUserData,
    Party,
    PartyPanel,
    PastBattleMarker,
    PauseMenu,
    PdxCoreSetting,
    PdxEnumSetting,
    PdxGuiFoldOut,
    PdxGuiGfxVideoControl,
    PdxGuiTableRow,
    PdxGuiTreeTable,
    PdxGuiWidget,
    PdxSetting,
    PdxSettingsWindow,
    PdxSettingsWindowCategory,
    PdxSettingsWindowScopedCategory,
    PdxValueSetting,
    PieTimer,
    PieTimerSlice,
    Playable,
    Player,
    PlayerJoinRequest,
    PlayerMessageItem,
    PlotLine,
    PoliticalLobby,
    PoliticalLobbyAppeasementFactor,
    PoliticalLobbyMarker,
    PoliticalLobbyPanel,
    PoliticalLobbyType,
    PoliticalMovement,
    PoliticalMovementPanel,
    PoliticsPanel,
    Pop,
    PopBrowserPanel,
    PopConsumptionGoods,
    PopDetailsPanel,
    PopList,
    PopListItem,
    PopNeed,
    PopType,
    PopWithIG,
    PopsOverviewPanel,
    PopulationGrouping,
    PopupManager,
    Portrait3dView,
    PortraitDataContext,
    PortraitEditorAnimationItem,
    PortraitEditorWindow,
    PortraitTooltip,
    PowerBloc,
    PowerBlocCoaPiece,
    PowerBlocCoaPieceSelector,
    PowerBlocCustomizationPopup,
    PowerBlocFormation,
    PowerBlocFormationPanel,
    PowerBlocFormationPrincipleSlot,
    PowerBlocIdentity,
    PowerBlocMapTexture,
    PowerBlocMapTextureSelector,
    PowerBlocPanel,
    PowerBlocPrinciple,
    PowerBlocPrincipleData,
    PowerBlocPrincipleGroup,
    PowerBlocPrincipleSelectionWindow,
    PowerBlocPrincipleSlot,
    PowerBlocStatue,
    PowerBlocStatueAccessoryType,
    PowerBlocStatueAccessoryTypeSelector,
    PowerBlocStatueHeroType,
    PowerBlocStatuePedestalType,
    PreviewMaskTexture,
    ProductionMarker,
    ProductionMethod,
    ProductionMethodConfirmationPopup,
    ProductionMethodGroup,
    ProgressInterface,
    Proposal,
    Province,
    RandomizableValueFloat,
    RandomizableValueInt,
    ReformGovernment,
    ReleaseCountryWindow,
    Religion,
    ReligionInfoPanel,
    RemoveFriendConfirmWindow,
    RepackWindow,
    ResignConfirmationWindow,
    RevolutionaryMovementMarker,
    RightClickMenuManager,
    Savable,
    SavableGroup,
    SaveDialog,
    SaveGame,
    SaveGameAnalysisView,
    SaveGameAnalyzer,
    SaveGameBlockData,
    SaveGameConfigView,
    SaveGameItem,
    SaveGameListView,
    SaveGameWindow,
    ScopeDebugData,
    ScopeDebugInspectorPlugin,
    ScopeObjectEditor,
    ScopeObjectProvider,
    ScopeObjectType,
    ScopedEditorSettingsCategory,
    ScopedEditorSettingsPage,
    ScopedJominiSettingsCategory,
    ScopedJominiSettingsPage,
    ScriptProfilerEntry,
    ScriptProfilerFileLine,
    ScriptProfilerGui,
    ScriptRunnerInspector,
    ScriptRunnerResult,
    ScriptedButton,
    ScriptedGui,
    ScriptedProgressBar,
    SeaNodeMarker,
    SeaRegionInfraMarker,
    SeaRegionPanel,
    SeaRegionWar,
    SearchListNodeWindow,
    SelectFromInputNodeWindow,
    SelectParticleUserDataDialog,
    SelectTool,
    SelectionHistory,
    SelectionLine,
    ServerInformation,
    SettingCategory,
    SettingsPage,
    ShippingLane,
    SkinEditor,
    SmartBrushHeightRange,
    SmartBrushPattern,
    SmartBrushPresetManager,
    SmartMaterialPaintingMode,
    Social,
    SocialNotificationWindow,
    SocialUI,
    SocialWidget,
    SplineAdjustmentTool,
    SplineAdjustmentToolMode,
    SplineRiverInteractionMode,
    SplineRiverTool,
    SplineStripTool,
    SplineStripToolMode,
    SplineToolsMapContentPanel,
    SplineTypeCreateSelectionDropdown,
    SplineTypeItem,
    SplineTypeSwitchSelectionDropdown,
    SplineVisibilityDropdown,
    StagedCommanderTransfer,
    StagedFormation,
    StagedUnitsWithType,
    State,
    StateGoods,
    StateInfraMarker,
    StateMarker,
    StatePopulationPanel,
    StateRegion,
    StateTrait,
    StatesPanel,
    StrategicRegion,
    StrategicRegionMarker,
    SubjectType,
    SupplyNetworkEntry,
    Sway,
    SwayCountryPanel,
    SwayOffer,
    SwayOfferType,
    TaxBurdenItem,
    TechTreeItem,
    TechTreeLine,
    TechTreePanel,
    Technology,
    TechnologyUnlock,
    TerrainToolButton,
    TextureEntry,
    TextureImporter,
    TextureList,
    TextureListDirectory,
    TextureListTexture,
    TextureNodeWindow,
    TextureViewer,
    Theater,
    Theme,
    ThemeCategory,
    ThemeWindow,
    TickTaskData,
    TickTaskDebuggerView,
    TickTaskDetailsView,
    TickTaskGraphItem,
    TickTaskGraphLine,
    TickTaskListView,
    TimeKeeper,
    TimedModifier,
    TimelineKeyframe,
    ToastMessageHandler,
    ToolDialog,
    ToolDialogButton,
    ToolMessageDialog,
    ToolProperty,
    ToolPropertyBool,
    ToolPropertyCColor,
    ToolPropertyCString,
    ToolPropertyColor,
    ToolPropertyCurve,
    ToolPropertyFloat,
    ToolPropertyInt,
    ToolPropertyInt16,
    ToolPropertyInt8,
    ToolPropertyList,
    ToolPropertySearchList,
    ToolPropertyString,
    ToolPropertyUint,
    ToolPropertyUint16,
    ToolPropertyUint8,
    ToolPropertyUndoableSearchList,
    ToolPropertyVec1fPercent,
    ToolPropertyVec2f,
    ToolPropertyVec2fPercent,
    ToolPropertyVec2i,
    ToolPropertyVec3f,
    ToolPropertyVec3i,
    ToolPropertyVec4i,
    ToolsPropertyDraggableValueFloat,
    ToolsPropertyDraggableValueInt,
    ToolsPropertyDraggableValueVector2f,
    ToolsPropertyDraggableValueVector2i,
    ToolsPropertyDraggableValueVector3f,
    ToolsPropertyDraggableValueVector3i,
    ToolsPropertyDraggableValueVector4i,
    ToolsPropertyPath,
    ToolsPropertyRangedValueFloat,
    ToolsPropertyRangedValueInt,
    ToolsPropertyTextureValue,
    ToolsSearch,
    ToolsSearchResult,
    ToolsUndoableValueBundleBool,
    ToolsUndoableValueBundleCColor,
    ToolsUndoableValueBundleCString,
    ToolsUndoableValueBundleColor,
    ToolsUndoableValueBundleFloat,
    ToolsUndoableValueBundleInt,
    ToolsUndoableValueBundleString,
    ToolsUndoableValueBundleUint,
    ToolsUndoableValueBundleUint16,
    ToolsUndoableValueBundleVec2f,
    ToolsUndoableValueBundleVec2i,
    ToolsUndoableValueBundleVec3f,
    ToolsUndoableValueBundleVec3i,
    ToolsUndoableValueBundleVec4i,
    TooltipInfo,
    TopFrontend,
    TradeRoute,
    TransferFormationPopup,
    Truce,
    Tutorial,
    Tweakable,
    TweakableCategory,
    TweakableUiEntry,
    TweakablesSnapshot,
    Tweaker,
    Type,
    UndoHistoryViewerClient,
    UndoStack,
    UserDataNode,
    VariableEntry,
    VariableInspectorEntry,
    VariableInspectorPlugin,
    VariableInspectorVariable,
    VariableList,
    VariableListEntry,
    VariableListInspectorPlugin,
    VariableListStore,
    VariableStore,
    VariableSystem,
    ViewerEntity,
    ViewerEntityLodInfo,
    ViewerEntityState,
    ViewerEntityStatePtr,
    War,
    WarGoal,
    WarGoalMarker,
    WarGoalPanelPair,
    WarGoalType,
    WarManager,
    WarPanel,
    WarParticipant,
}
