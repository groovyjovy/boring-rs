use serde::{Deserialize, Serialize};

// v2.10の構造体を流用
use crate::boring_structs_210::*;
use crate::types::FreeInfo;

// ============================================================================
// ルート構造体
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ボーリング情報")]
pub struct Boring300 {
    #[serde(rename = "@DTD_version")]
    pub dtd_version: Option<String>,
    #[serde(rename = "標題情報")]
    pub title: Title300,
    #[serde(rename = "コア情報")]
    pub core: Core300,
}

// ============================================================================
// 標題情報 - v2.10と同じ構造だが、BasicSurvey300を使用
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Title300 {
    #[serde(rename = "調査基本情報")]
    pub basic_info: BasicSurvey300,
    #[serde(rename = "経度緯度情報")]
    pub longitude_latitude: LngLat210,
    #[serde(default, rename = "ローカル座標")]
    pub local_coordinates: Vec<LocalCoordinate210>,
    #[serde(rename = "調査位置")]
    pub survey_position: SurveyPosition210,
    #[serde(rename = "発注機関")]
    pub order_institution: OrderInstitution210,
    #[serde(rename = "調査期間")]
    pub survey_period: SurveyPeriod210,
    #[serde(rename = "調査会社")]
    pub survey_company: SurveyCompany210,
    #[serde(rename = "ボーリング基本情報")]
    pub boring_basic_info: BoringBasicInfo210,
    #[serde(rename = "試錐機")]
    pub drilling_machine: DrillingMachine210,
    #[serde(rename = "エンジン")]
    pub engine: Engine210,
    #[serde(default, rename = "ハンマー落下用具")]
    pub hammer_drop_tool: Option<HammerDropTool210>,
    #[serde(default, rename = "N値記録用具")]
    pub n_value_recorder: Option<NValueRecorder210>,
    #[serde(rename = "ポンプ")]
    pub pump: Pump210,
    #[serde(default, rename = "櫓種類")]
    pub tower_type: Option<TowerType210>,
}

// ============================================================================
// 調査基本情報 - v3.00で適用規格が追加
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicSurvey300 {
    #[serde(default, rename = "事業工事名")]
    pub project_name: Option<String>,
    #[serde(rename = "調査名")]
    pub survey_name: Option<String>,
    #[serde(rename = "調査目的")]
    pub survey_purpose: Option<String>,
    #[serde(rename = "調査対象")]
    pub survey_target: Option<String>,
    #[serde(rename = "ボーリング名")]
    pub boring_name: Option<String>,
    #[serde(rename = "ボーリング総数")]
    pub total_boring: Option<String>,
    #[serde(rename = "ボーリング連番")]
    pub boring_serial: Option<String>,
}

// ============================================================================
// コア情報 - 岩石土区分を使用、その他はv2.10を再利用
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Core300 {
    #[serde(rename = "岩石土区分")] // v3.00で名称変更
    pub rock_soil_classifications: Vec<RockSoilClassification300>,
    #[serde(default, rename = "色調")]
    pub colors: Vec<Color210>,
    #[serde(default, rename = "観察記事")]
    pub observational_articles: Vec<ObservationalArticle210>,
    #[serde(default, rename = "観察記事枠線")]
    pub observational_article_frames: Vec<ObservationalArticleFrame210>,
    #[serde(default, rename = "標準貫入試験")]
    pub standard_penetration_tests: Vec<StandardPenetrationTest210>,
    #[serde(default, rename = "標準貫入試験詳細データ")]
    pub standard_penetration_test_details: Vec<StandardPenetrationTestDetail210>,
    #[serde(default, rename = "ルジオン試験")]
    pub lugeon_tests: Vec<LugeonTest210>,
    #[serde(default, rename = "ルジオン試験詳細データ")]
    pub lugeon_test_details: Vec<LugeonTestDetail210>,
    #[serde(default, rename = "相対密度稠度")]
    pub relative_density_consistency: Vec<RelativeDensityConsistency210>,
    #[serde(default, rename = "硬軟区分判定表")]
    pub hardness_classification_table: Vec<HardnessClassificationTable210>,
    #[serde(default, rename = "硬軟区分")]
    pub hardness_classifications: Vec<HardnessClassification300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "コア形状区分判定表")]
    pub core_shape_classification_table: Vec<CoreShapeClassificationTable210>,
    #[serde(default, rename = "コア形状区分")]
    pub core_shape_classifications: Vec<CoreShapeClassification300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "割れ目区分判定表")]
    pub fracture_classification_table: Vec<FractureClassificationTable210>,
    #[serde(default, rename = "割れ目区分")]
    pub fracture_classifications: Vec<FractureClassification300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "風化区分判定表")]
    pub weathering_classification_table: Vec<WeatheringClassificationTable210>,
    #[serde(default, rename = "風化区分")]
    pub weathering_classifications: Vec<WeatheringClassification300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "変質区分判定表")]
    pub alteration_classification_table: Vec<AlterationClassificationTable210>,
    #[serde(default, rename = "変質区分")]
    pub alteration_classifications: Vec<AlterationClassification300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "孔内水平載荷試験")]
    pub borehole_horizontal_load_tests: Vec<BoreholeHorizontalLoadTest210>,
    #[serde(default, rename = "透水試験")]
    pub permeability_tests: Vec<PermeabilityTest210>,
    #[serde(default, rename = "P波試験")]
    pub p_wave_tests: Vec<PWaveTest210>,
    #[serde(default, rename = "S波試験")]
    pub s_wave_tests: Vec<SWaveTest210>,
    #[serde(default, rename = "その他原位置試験")]
    pub other_in_situ_tests: Vec<OtherInSituTest210>,
    #[serde(default, rename = "試料採取")]
    pub sample_collections: Vec<SampleCollection210>,
    #[serde(default, rename = "地盤材料の工学的分類")]
    pub ground_material_classifications: Vec<GroundMaterialClassification210>,
    #[serde(default, rename = "地質時代")]
    pub geological_ages: Vec<GeologicalAge300>, // v3.00で構造変更
    #[serde(default, rename = "地層岩体区分")]
    pub stratum_rock_classifications: Vec<StratumRockClassification210>,
    #[serde(default, rename = "孔内水位")]
    pub borehole_water_levels: Vec<BoreholeWaterLevel210>,
    #[serde(default, rename = "掘削工程")]
    pub drilling_processes: Vec<DrillingProcess210>,
    #[serde(default, rename = "孔径孔壁保護")]
    pub borehole_diameter_protections: Vec<BoreholeDiameterProtection210>,
    #[serde(default, rename = "掘進速度")]
    pub drilling_speeds: Vec<DrillingSpeed210>,
    #[serde(default, rename = "コアチューブビット")]
    pub core_tube_bits: Vec<CoreTubeBit210>,
    #[serde(default, rename = "給圧条件")]
    pub pressure_conditions: Vec<PressureCondition210>,
    #[serde(default, rename = "回転数")]
    pub rotation_speeds: Vec<RotationSpeed210>,
    #[serde(default, rename = "送水条件")]
    pub water_supply_conditions: Vec<WaterSupplyCondition210>,
    #[serde(default, rename = "断層破砕帯区分")]
    pub fault_fracture_zone_classifications: Vec<FaultFractureZoneClassification210>,
    #[serde(default, rename = "コア採取率")]
    pub core_recovery_rates: Vec<CoreRecoveryRate300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "最大コア長")]
    pub maximum_core_lengths: Vec<MaximumCoreLength300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "RQD")]
    pub rqds: Vec<RQD210>,
    #[serde(default, rename = "岩級区分判定表")]
    pub rock_class_classification_table: Vec<RockClassClassificationTable210>,
    #[serde(default, rename = "岩級区分")]
    pub rock_class_classifications: Vec<RockClassClassification300>, // v3.00で内部フィールドがオプション化
    #[serde(default, rename = "保孔管")]
    pub casing_pipes: Vec<CasingPipe210>,
    #[serde(default, rename = "計測機器")]
    pub measuring_instruments: Vec<MeasuringInstrument210>,
    #[serde(default, rename = "地下水検層試験")]
    pub groundwater_logging_tests: Vec<GroundwaterLoggingTest210>,
    #[serde(default, rename = "地下水検層試験詳細データ")]
    pub groundwater_logging_test_details: Vec<GroundwaterLoggingTestDetail210>,
    #[serde(default, rename = "地下水検層試験判定結果")]
    pub groundwater_logging_test_results: Vec<GroundwaterLoggingTestResult210>,
    #[serde(default, rename = "備考")]
    pub remarks: Vec<Remark210>,
    #[serde(default, rename = "フリー情報")]
    pub free_info: Vec<FreeInfo>,
}

// ============================================================================
// 岩石土区分 - v3.00で大きく構造変更
// DTD: <!ELEMENT 岩石土区分 (岩石土区分_下端深度, 岩石土区分_岩石土名,
//      岩石土区分_岩石土記号?, 岩石土区分_岩石群+)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RockSoilClassification300 {
    #[serde(rename = "岩石土区分_下端深度")]
    pub depth: Option<String>,
    #[serde(rename = "岩石土区分_岩石土名")]
    pub rock_soil_name: Option<String>,
    #[serde(default, rename = "岩石土区分_岩石土記号")]
    pub rock_soil_symbol: Option<String>,
    #[serde(rename = "岩石土区分_岩石群")] // + (1回以上)
    pub rock_groups: Vec<RockGroup300>,
}

// ============================================================================
// 岩石土区分_岩石群
// DTD: <!ELEMENT 岩石土区分_岩石群 (岩石土区分_岩石群コード, 岩石土区分_岩石土コード+)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RockGroup300 {
    #[serde(rename = "岩石土区分_岩石群コード")]
    pub rock_group_code: Option<String>,
    #[serde(rename = "岩石土区分_岩石土コード")] // + (1回以上)
    pub rock_soil_codes: Vec<RockSoilCode300>,
}

// ============================================================================
// 岩石土区分_岩石土コード
// DTD: <!ELEMENT 岩石土区分_岩石土コード (岩石土区分_岩相?, 岩石土区分_岩石?,
//      岩石土区分_変成岩岩相?, 岩石土区分_変成岩岩石?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RockSoilCode300 {
    #[serde(default, rename = "岩石土区分_岩相")]
    pub lithofacies: Option<String>,
    #[serde(default, rename = "岩石土区分_岩石")]
    pub rock: Option<String>,
    #[serde(default, rename = "岩石土区分_変成岩岩相")]
    pub metamorphic_rock_lithofacies: Option<String>,
    #[serde(default, rename = "岩石土区分_変成岩岩石")]
    pub metamorphic_rock: Option<String>,
}

// ============================================================================
// 地質時代 - v3.00で構造変更
// DTD: <!ELEMENT 地質時代 (地質時代_上端深度, 地質時代_下端深度, 地質時代_地質時代名,
//      地質時代_形成年代上限?, 地質時代_形成年代下限?, 地質時代_変成年代上限?, 地質時代_変成年代下限?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GeologicalAge300 {
    #[serde(rename = "地質時代_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "地質時代_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "地質時代_地質時代名")] // v2.10の「地質時代_時代名」から変更
    pub geological_age_name: Option<String>,
    #[serde(default, rename = "地質時代_形成年代上限")] // v3.00で追加
    pub formation_age_upper_limit: Option<String>,
    #[serde(default, rename = "地質時代_形成年代下限")] // v3.00で追加
    pub formation_age_lower_limit: Option<String>,
    #[serde(default, rename = "地質時代_変成年代上限")] // v3.00で追加
    pub metamorphic_age_upper_limit: Option<String>,
    #[serde(default, rename = "地質時代_変成年代下限")] // v3.00で追加
    pub metamorphic_age_lower_limit: Option<String>,
}

// ============================================================================
// 硬軟区分 - v3.00で「硬軟区分_硬軟区分」がオプション化
// DTD: <!ELEMENT 硬軟区分 (硬軟区分_下端深度, 硬軟区分_硬軟区分?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct HardnessClassification300 {
    #[serde(rename = "硬軟区分_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "硬軟区分_硬軟区分")] // v3.00でオプション化
    pub classification: Option<String>,
}

// ============================================================================
// コア形状区分 - v3.00で「コア形状区分_コア形状区分」がオプション化
// DTD: <!ELEMENT コア形状区分 (コア形状区分_下端深度, コア形状区分_コア形状区分?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreShapeClassification300 {
    #[serde(rename = "コア形状区分_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "コア形状区分_コア形状区分")] // v3.00でオプション化
    pub classification: Option<String>,
}

// ============================================================================
// 割れ目区分 - v3.00で「割れ目区分_割れ目区分」がオプション化
// DTD: <!ELEMENT 割れ目区分 (割れ目区分_下端深度, 割れ目区分_割れ目区分?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FractureClassification300 {
    #[serde(rename = "割れ目区分_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "割れ目区分_割れ目区分")] // v3.00でオプション化
    pub classification: Option<String>,
}

// ============================================================================
// 風化区分 - v3.00で「風化区分_風化区分」がオプション化
// DTD: <!ELEMENT 風化区分 (風化区分_下端深度, 風化区分_風化区分?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatheringClassification300 {
    #[serde(rename = "風化区分_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "風化区分_風化区分")] // v3.00でオプション化
    pub classification: Option<String>,
}

// ============================================================================
// 変質区分 - v3.00で「変質区分_変質区分」がオプション化
// DTD: <!ELEMENT 変質区分 (変質区分_下端深度, 変質区分_変質区分?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterationClassification300 {
    #[serde(rename = "変質区分_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "変質区分_変質区分")] // v3.00でオプション化
    pub classification: Option<String>,
}

// ============================================================================
// コア採取率 - v3.00で「コア採取率_採取率」がオプション化
// DTD: <!ELEMENT コア採取率 (コア採取率_下端深度, コア採取率_採取率?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreRecoveryRate300 {
    #[serde(rename = "コア採取率_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "コア採取率_採取率")] // v3.00でオプション化
    pub recovery_rate: Option<String>,
}

// ============================================================================
// 最大コア長 - v3.00で「最大コア長_コア長」がオプション化
// DTD: <!ELEMENT 最大コア長 (最大コア長_下端深度, 最大コア長_コア長?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct MaximumCoreLength300 {
    #[serde(rename = "最大コア長_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "最大コア長_コア長")] // v3.00でオプション化
    pub core_length: Option<String>,
}

// ============================================================================
// 岩級区分 - v3.00で「岩級区分_岩級区分」がオプション化
// DTD: <!ELEMENT 岩級区分 (岩級区分_下端深度, 岩級区分_岩級区分?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RockClassClassification300 {
    #[serde(rename = "岩級区分_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "岩級区分_岩級区分")] // v3.00でオプション化
    pub classification: Option<String>,
}
