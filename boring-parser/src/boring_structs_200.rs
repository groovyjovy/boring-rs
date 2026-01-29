use serde::{Deserialize, Serialize};

// v2.10の構造体を流用
use crate::boring_structs_210::*;
use crate::types::FreeInfo;

// ============================================================================
// ルート構造体
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ボーリング情報")]
pub struct Boring200 {
    #[serde(rename = "@DTD_version")]
    pub dtd_version: Option<String>,
    #[serde(rename = "標題情報")]
    pub title: Title200,
    #[serde(rename = "コア情報")]
    pub core: Core200,
}

// ============================================================================
// 標題情報 - v2.00固有
// DTD: <!ELEMENT 標題情報 (調査基本情報, 経度緯度情報, ローカル座標?,
//      調査位置, 発注機関, 調査期間, 調査会社, ボーリング基本情報,
//      試錐機, エンジン, ハンマー落下用具, N値記録用具, ポンプ, 港湾局指定コード?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Title200 {
    #[serde(rename = "調査基本情報")]
    pub basic_info: BasicSurvey210,
    #[serde(rename = "経度緯度情報")]
    pub longitude_latitude: LngLat210,
    #[serde(default, rename = "ローカル座標")] // ? (オプション、単数、X/Y/Z形式)
    pub local_coordinate: Option<LocalCoordinate200>,
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
    #[serde(rename = "ハンマー落下用具")] // v2.00/v2.01では必須要素
    pub hammer_drop_tool: HammerDropTool210,
    #[serde(rename = "N値記録用具")] // v2.00/v2.01では必須要素
    pub n_value_recorder: NValueRecorder210,
    #[serde(rename = "ポンプ")]
    pub pump: Pump210,
    #[serde(default, rename = "港湾局指定コード")] // ? (オプション) - v2.00固有
    pub port_authority_code: Option<PortAuthorityCode200>,
}

// ============================================================================
// ローカル座標 - v2.00固有 (X/Y/Z座標形式)
// DTD: <!ELEMENT ローカル座標 (X座標定義, X座標, Y座標定義, Y座標, Z座標定義, Z座標)>
// v2.01で各フィールドがオプションに変更
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalCoordinate200 {
    #[serde(default, rename = "X座標定義")] // v2.00: 必須, v2.01: オプション
    pub x_definition: Option<String>,
    #[serde(default, rename = "X座標")] // v2.00: 必須, v2.01: オプション
    pub x: Option<String>,
    #[serde(default, rename = "Y座標定義")] // v2.00: 必須, v2.01: オプション
    pub y_definition: Option<String>,
    #[serde(default, rename = "Y座標")] // v2.00: 必須, v2.01: オプション
    pub y: Option<String>,
    #[serde(default, rename = "Z座標定義")] // v2.00: 必須, v2.01: オプション
    pub z_definition: Option<String>,
    #[serde(default, rename = "Z座標")] // v2.00: 必須, v2.01: オプション
    pub z: Option<String>,
}

// ============================================================================
// 港湾局指定コード - v2.00固有 (v2.10で櫓種類に置換)
// DTD: <!ELEMENT 港湾局指定コード (櫓種類コード, 建設局, 都道府県, 港名, 調査者)>
// v2.01で各フィールドがオプションに変更
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct PortAuthorityCode200 {
    #[serde(default, rename = "櫓種類コード")] // v2.00: 必須, v2.01: オプション
    pub tower_type_code: Option<String>,
    #[serde(default, rename = "建設局")] // v2.00: 必須, v2.01: オプション
    pub construction_bureau: Option<String>,
    #[serde(default, rename = "都道府県")] // v2.00: 必須, v2.01: オプション
    pub prefecture: Option<String>,
    #[serde(default, rename = "港名")] // v2.00: 必須, v2.01: オプション
    pub port_name: Option<String>,
    #[serde(default, rename = "調査者")] // v2.00: 必須, v2.01: オプション
    pub investigator: Option<String>,
}

// ============================================================================
// コア情報 - v2.00固有
// DTD: <!ELEMENT コア情報 (土質岩種区分+, 色調*, 観察記事*, 観察記事枠線*,
//      標準貫入試験*, ルジオン試験*, ルジオン試験詳細データ*, 相対密度稠度*,
//      硬軟区分判定表*, 硬軟区分*, コア形状区分判定表*, コア形状区分*,
//      割れ目区分判定表*, 割れ目区分*, 風化区分判定表*, 風化区分*,
//      変質区分判定表*, 変質区分*, 孔内水平載荷試験*, 透水試験*,
//      P波試験*, S波試験*, その他原位置試験*, 試料採取*,
//      地盤材料の工学的分類*, 地質時代*, 地層岩体区分*, 孔内水位*,
//      掘削工程*, 孔径孔壁保護*, 掘進速度*, コアチューブビット*,
//      給圧条件*, 回転数*, 送水条件*, 断層破砕帯区分*,
//      コア採取率*, 最大コア長*, RQD*, 岩級区分判定表*, 岩級区分*,
//      保孔管*, 計測機器*, 地下水検層試験*, 地下水検層試験詳細データ*,
//      地下水検層試験判定結果*, 備考*, フリー情報*)>
//
// v2.10との差分:
// - 標準貫入試験詳細データ が存在しない (v2.10で追加)
// - LugeonTest200 を使用 (損失水頭補正値フィールドあり)
// - LugeonTestDetail200 を使用 (注入圧力、v2.10では有効圧力)
// - GroundwaterLoggingTest200 を使用 (測定時間が単一String)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Core200 {
    #[serde(rename = "土質岩種区分")] // + (1回以上)
    pub soil_rock_classifications: Vec<SoilRockClassification210>,
    #[serde(default, rename = "色調")]
    pub colors: Vec<Color210>,
    #[serde(default, rename = "観察記事")]
    pub observational_articles: Vec<ObservationalArticle210>,
    #[serde(default, rename = "観察記事枠線")]
    pub observational_article_frames: Vec<ObservationalArticleFrame210>,
    #[serde(default, rename = "標準貫入試験")]
    pub standard_penetration_tests: Vec<StandardPenetrationTest210>,
    // NOTE: 標準貫入試験詳細データ は v2.00には存在しない (v2.10で追加)
    #[serde(default, rename = "ルジオン試験")]
    pub lugeon_tests: Vec<LugeonTest200>, // v2.00固有
    #[serde(default, rename = "ルジオン試験詳細データ")]
    pub lugeon_test_details: Vec<LugeonTestDetail200>, // v2.00固有
    #[serde(default, rename = "相対密度稠度")]
    pub relative_density_consistency: Vec<RelativeDensityConsistency210>,
    #[serde(default, rename = "硬軟区分判定表")]
    pub hardness_classification_table: Vec<HardnessClassificationTable210>,
    #[serde(default, rename = "硬軟区分")]
    pub hardness_classifications: Vec<HardnessClassification210>,
    #[serde(default, rename = "コア形状区分判定表")]
    pub core_shape_classification_table: Vec<CoreShapeClassificationTable210>,
    #[serde(default, rename = "コア形状区分")]
    pub core_shape_classifications: Vec<CoreShapeClassification210>,
    #[serde(default, rename = "割れ目区分判定表")]
    pub fracture_classification_table: Vec<FractureClassificationTable210>,
    #[serde(default, rename = "割れ目区分")]
    pub fracture_classifications: Vec<FractureClassification210>,
    #[serde(default, rename = "風化区分判定表")]
    pub weathering_classification_table: Vec<WeatheringClassificationTable210>,
    #[serde(default, rename = "風化区分")]
    pub weathering_classifications: Vec<WeatheringClassification210>,
    #[serde(default, rename = "変質区分判定表")]
    pub alteration_classification_table: Vec<AlterationClassificationTable210>,
    #[serde(default, rename = "変質区分")]
    pub alteration_classifications: Vec<AlterationClassification210>,
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
    pub geological_ages: Vec<GeologicalAge210>,
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
    pub core_recovery_rates: Vec<CoreRecoveryRate210>,
    #[serde(default, rename = "最大コア長")]
    pub maximum_core_lengths: Vec<MaximumCoreLength210>,
    #[serde(default, rename = "RQD")]
    pub rqds: Vec<RQD210>,
    #[serde(default, rename = "岩級区分判定表")]
    pub rock_class_classification_table: Vec<RockClassClassificationTable210>,
    #[serde(default, rename = "岩級区分")]
    pub rock_class_classifications: Vec<RockClassClassification210>,
    #[serde(default, rename = "保孔管")]
    pub casing_pipes: Vec<CasingPipe210>,
    #[serde(default, rename = "計測機器")]
    pub measuring_instruments: Vec<MeasuringInstrument210>,
    #[serde(default, rename = "地下水検層試験")]
    pub groundwater_logging_tests: Vec<GroundwaterLoggingTest200>, // v2.00固有
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
// ルジオン試験 - v2.00固有 (損失水頭補正値フィールドが存在)
// DTD: <!ELEMENT ルジオン試験 (ルジオン試験_試験番号, ルジオン試験_上端深度,
//      ルジオン試験_下端深度, ルジオン試験_圧力管理方法コード, ルジオン試験_圧力管理方法?,
//      ルジオン試験_損失水頭補正値_注水管?, ルジオン試験_損失水頭補正値_パッカー?,
//      ルジオン試験_圧力最大スケール, ルジオン試験_注入量最大スケール,
//      ルジオン試験_圧力開始点, ルジオン試験_注入量開始点,
//      ルジオン試験_ルジオン値区分, ルジオン試験_ルジオン値, ルジオン試験_限界圧力)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LugeonTest200 {
    #[serde(rename = "ルジオン試験_試験番号")]
    pub test_number: Option<String>,
    #[serde(rename = "ルジオン試験_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "ルジオン試験_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "ルジオン試験_圧力管理方法コード")]
    pub pressure_management_code: Option<String>,
    #[serde(default, rename = "ルジオン試験_圧力管理方法")] // オプション
    pub pressure_management: Option<String>,
    // v2.00固有フィールド (v2.10で削除) - DTD順序で圧力最大スケールの前
    #[serde(default, rename = "ルジオン試験_損失水頭補正値_注水管")] // オプション
    pub head_loss_correction_injection_pipe: Option<String>,
    #[serde(default, rename = "ルジオン試験_損失水頭補正値_パッカー")] // オプション
    pub head_loss_correction_packer: Option<String>,
    #[serde(rename = "ルジオン試験_圧力最大スケール")]
    pub pressure_max_scale: Option<String>,
    #[serde(rename = "ルジオン試験_注入量最大スケール")]
    pub injection_max_scale: Option<String>,
    #[serde(rename = "ルジオン試験_圧力開始点")]
    pub pressure_start_point: Option<String>,
    #[serde(rename = "ルジオン試験_注入量開始点")]
    pub injection_start_point: Option<String>,
    #[serde(rename = "ルジオン試験_ルジオン値区分")]
    pub lugeon_value_classification: Option<String>,
    #[serde(rename = "ルジオン試験_ルジオン値")]
    pub lugeon_value: Option<String>,
    #[serde(rename = "ルジオン試験_限界圧力")]
    pub limit_pressure: Option<String>,
}

// ============================================================================
// ルジオン試験詳細データ - v2.00固有 (注入圧力、v2.10では有効圧力)
// DTD: <!ELEMENT ルジオン試験詳細データ (ルジオン試験詳細データ_試験番号,
//      ルジオン試験詳細データ_注入圧力, ルジオン試験詳細データ_注入量)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LugeonTestDetail200 {
    #[serde(rename = "ルジオン試験詳細データ_試験番号")]
    pub test_number: Option<String>,
    #[serde(rename = "ルジオン試験詳細データ_注入圧力")] // v2.10では「有効圧力」
    pub injection_pressure: Option<String>,
    #[serde(rename = "ルジオン試験詳細データ_注入量")]
    pub injection_amount: Option<String>,
}

// ============================================================================
// 地下水検層試験 - v2.00固有 (測定時間が単一#PCDATA)
// DTD: <!ELEMENT 地下水検層試験 (地下水検層試験_試験番号, 地下水検層試験_上端深度,
//      地下水検層試験_下端深度, 地下水検層試験_掘削深度, 地下水検層試験_孔内水位,
//      地下水検層試験_試験方法コード, 地下水検層試験_電解質溶液濃度, 地下水検層試験_測定時間)>
// v2.10では 測定時間* (繰り返し) に変更
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundwaterLoggingTest200 {
    #[serde(rename = "地下水検層試験_試験番号")]
    pub test_number: Option<String>,
    #[serde(rename = "地下水検層試験_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "地下水検層試験_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "地下水検層試験_掘削深度")]
    pub drilling_depth: Option<String>,
    #[serde(rename = "地下水検層試験_孔内水位")]
    pub water_level: Option<String>,
    #[serde(rename = "地下水検層試験_試験方法コード")]
    pub test_method_code: Option<String>,
    #[serde(rename = "地下水検層試験_電解質溶液濃度")]
    pub electrolyte_concentration: Option<String>,
    #[serde(default, rename = "地下水検層試験_測定時間")] // 単一文字列 (v2.10ではVec<String>)
    pub measurement_time: Option<String>,
}
