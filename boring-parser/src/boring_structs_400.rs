use serde::{Deserialize, Serialize};

// v3.00とv2.10の構造体を流用
use crate::boring_structs_210::*;
use crate::boring_structs_300::*;
use crate::types::FreeInfo;

// ============================================================================
// ルート構造体 - v4.00で基礎情報が追加
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ボーリング情報")]
pub struct Boring400 {
    #[serde(rename = "@DTD_version")]
    pub dtd_version: Option<String>,
    #[serde(rename = "基礎情報")]
    pub foundation_info: FoundationInfo400,
    #[serde(rename = "標題情報")]
    pub title: Title400,
    #[serde(rename = "コア情報")]
    pub core: Core400,
}

// ============================================================================
// 基礎情報 - v4.00で新規追加
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FoundationInfo400 {
    #[serde(default, rename = "適用規格")]
    pub applicable_standards: Vec<String>,
    #[serde(rename = "公開フラグ")]
    pub public_flag: PublicFlag400,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicFlag400 {
    #[serde(rename = "公開フラグ_コード")]
    pub code: Option<String>,
    #[serde(default, rename = "公開フラグ_備考")]
    pub remarks: Option<String>,
}

// ============================================================================
// 標題情報 - v4.00でハンマー落下用具とN値記録用具が削除
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Title400 {
    #[serde(rename = "調査基本情報")]
    pub basic_info: BasicSurvey300, // v3.00を流用
    #[serde(rename = "経度緯度情報")]
    pub longitude_latitude: LngLat210, // v2.10を流用
    #[serde(default, rename = "ローカル座標")]
    pub local_coordinates: Vec<LocalCoordinate210>, // v2.10を流用
    #[serde(rename = "調査位置")]
    pub survey_position: SurveyPosition210, // v2.10を流用
    #[serde(rename = "発注機関")]
    pub order_institution: OrderInstitution210, // v2.10を流用
    #[serde(rename = "調査期間")]
    pub survey_period: SurveyPeriod210, // v2.10を流用
    #[serde(rename = "調査会社")]
    pub survey_company: SurveyCompany400, // v4.00で拡張（登録番号追加）
    #[serde(rename = "ボーリング基本情報")]
    pub boring_basic_info: BoringBasicInfo400, // v4.00でフィールド名変更
    #[serde(rename = "試錐機")]
    pub drilling_machine: DrillingMachine210, // v2.10を流用
    #[serde(rename = "エンジン")]
    pub engine: Engine210, // v2.10を流用
    // v4.00で削除: ハンマー落下用具、N値記録用具
    #[serde(rename = "ポンプ")]
    pub pump: Pump210, // v2.10を流用
    #[serde(default, rename = "櫓種類")]
    pub tower_type: Option<TowerType210>, // v2.10を流用
}

// ============================================================================
// 調査会社 - v4.00で地質調査技士登録番号等を追加
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyCompany400 {
    #[serde(rename = "調査会社_名称")]
    pub name: Option<String>,
    #[serde(rename = "調査会社_TEL")]
    pub tel: Option<String>,
    #[serde(rename = "調査会社_主任技師_氏名")]
    pub chief_engineer_name: Option<String>,
    #[serde(default, rename = "調査会社_主任技師_地質調査技士登録番号")]
    pub chief_engineer_registration_number: Option<String>,
    #[serde(rename = "調査会社_現場代理人_氏名")]
    pub site_agent_name: Option<String>,
    #[serde(default, rename = "調査会社_現場代理人_地質調査技士登録番号")]
    pub site_agent_registration_number: Option<String>,
    #[serde(rename = "調査会社_コア鑑定者_氏名")]
    pub core_appraiser_name: Option<String>,
    #[serde(default, rename = "調査会社_コア鑑定者_地質調査技士登録番号")]
    pub core_appraiser_registration_number: Option<String>,
    #[serde(rename = "調査会社_ボーリング責任者_氏名")]
    pub boring_supervisor_name: Option<String>,
    #[serde(default, rename = "調査会社_ボーリング責任者_地質調査技士登録番号")]
    pub boring_supervisor_registration_number: Option<String>,
    #[serde(default, rename = "調査会社_電子納品管理者_氏名")]
    pub electronic_delivery_manager_name: Option<String>,
    #[serde(default, rename = "調査会社_電子納品管理者_地質情報管理士登録番号")]
    pub electronic_delivery_manager_registration_number: Option<String>,
}

// ============================================================================
// ボーリング基本情報 - v4.00でフィールド名変更
// 総掘進長→総削孔長、掘進角度→角度、掘進方向→方位
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BoringBasicInfo400 {
    #[serde(rename = "孔口標高")]
    pub surface_elevation: Option<String>,
    #[serde(rename = "総削孔長")] // v3.00: 総掘進長
    pub total_drilling_length: Option<String>,
    #[serde(rename = "柱状図様式")]
    pub columnar_diagram_format: Option<String>,
    #[serde(default, rename = "角度")] // v3.00: 掘進角度
    pub angle: Option<String>,
    #[serde(default, rename = "方位")] // v3.00: 掘進方向
    pub azimuth: Option<String>,
    #[serde(default, rename = "地盤勾配")]
    pub ground_slope: Option<String>,
}

// ============================================================================
// コア情報 - v4.00で名称変更と新規要素追加
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Core400 {
    // 岩石土区分 → 工学的地質区分名現場土質名
    #[serde(rename = "工学的地質区分名現場土質名")]
    pub engineering_geology_classifications: Vec<EngineeringGeologyClassification400>,
    #[serde(default, rename = "色調")]
    pub colors: Vec<Color210>, // v2.10を流用
    #[serde(default, rename = "観察記事")]
    pub observational_articles: Vec<ObservationalArticle210>, // v2.10を流用
    #[serde(default, rename = "観察記事枠線")]
    pub observational_article_frames: Vec<ObservationalArticleFrame210>, // v2.10を流用
    #[serde(default, rename = "標準貫入試験")]
    pub standard_penetration_tests: Vec<StandardPenetrationTest400>, // v4.00で単位変更
    #[serde(default, rename = "標準貫入試験詳細データ")]
    pub standard_penetration_test_details: Vec<StandardPenetrationTestDetail210>, // v2.10を流用
    #[serde(default, rename = "ルジオン試験")]
    pub lugeon_tests: Vec<LugeonTest210>, // v2.10を流用
    #[serde(default, rename = "ルジオン試験詳細データ")]
    pub lugeon_test_details: Vec<LugeonTestDetail210>, // v2.10を流用
    #[serde(default, rename = "相対密度稠度")]
    pub relative_density_consistency: Vec<RelativeDensityConsistency210>, // v2.10を流用
    #[serde(default, rename = "硬軟区分判定表")]
    pub hardness_classification_table: Vec<HardnessClassificationTable210>, // v2.10を流用
    #[serde(default, rename = "硬軟区分")]
    pub hardness_classifications: Vec<HardnessClassification300>, // v3.00を流用
    // コア形状区分 → ボーリングコアの形状区分
    #[serde(default, rename = "ボーリングコアの形状区分判定表")]
    pub boring_core_shape_classification_table: Vec<CoreShapeClassificationTable210>, // v2.10を流用（名称のみ変更）
    #[serde(default, rename = "ボーリングコアの形状区分")]
    pub boring_core_shape_classifications: Vec<CoreShapeClassification300>, // v3.00を流用（名称のみ変更）
    // 割れ目区分 → 割れ目の状態区分
    #[serde(default, rename = "割れ目の状態区分判定表")]
    pub crack_state_classification_table: Vec<FractureClassificationTable210>, // v2.10を流用（名称のみ変更）
    #[serde(default, rename = "割れ目の状態区分")]
    pub crack_state_classifications: Vec<FractureClassification300>, // v3.00を流用（名称のみ変更）
    // 風化区分 → 風化の程度区分
    #[serde(default, rename = "風化の程度区分判定表")]
    pub weathering_degree_classification_table: Vec<WeatheringClassificationTable210>, // v2.10を流用（名称のみ変更）
    #[serde(default, rename = "風化の程度区分")]
    pub weathering_degree_classifications: Vec<WeatheringClassification300>, // v3.00を流用（名称のみ変更）
    // 変質区分 → 熱水変質の程度区分
    #[serde(default, rename = "熱水変質の程度区分判定表")]
    pub hydrothermal_alteration_degree_classification_table: Vec<AlterationClassificationTable210>, // v2.10を流用（名称のみ変更）
    #[serde(default, rename = "熱水変質の程度区分")]
    pub hydrothermal_alteration_degree_classifications: Vec<AlterationClassification300>, // v3.00を流用（名称のみ変更）
    // 破砕度（v4.00で新規追加）
    #[serde(default, rename = "破砕度判定表")]
    pub crushing_degree_classification_table: Vec<CrushingDegreeClassificationTable400>,
    #[serde(default, rename = "破砕度")]
    pub crushing_degrees: Vec<CrushingDegree400>,
    // 孔内水平載荷試験 → 孔内載荷試験
    #[serde(default, rename = "孔内載荷試験")]
    pub borehole_loading_tests: Vec<BoreholeHorizontalLoadTest210>, // v2.10を流用（名称のみ変更）
    #[serde(default, rename = "透水試験")]
    pub permeability_tests: Vec<PermeabilityTest210>, // v2.10を流用
    #[serde(default, rename = "P波試験")]
    pub p_wave_tests: Vec<PWaveTest210>, // v2.10を流用
    #[serde(default, rename = "S波試験")]
    pub s_wave_tests: Vec<SWaveTest210>, // v2.10を流用
    #[serde(default, rename = "その他原位置試験")]
    pub other_in_situ_tests: Vec<OtherInSituTest210>, // v2.10を流用
    #[serde(default, rename = "試料採取")]
    pub sample_collections: Vec<SampleCollection210>, // v2.10を流用
    #[serde(default, rename = "地盤材料の工学的分類")]
    pub ground_material_classifications: Vec<GroundMaterialClassification210>, // v2.10を流用
    #[serde(default, rename = "地質時代")]
    pub geological_ages: Vec<GeologicalAge300>, // v3.00を流用
    #[serde(default, rename = "地層岩体区分")]
    pub stratum_rock_classifications: Vec<StratumRockClassification210>, // v2.10を流用
    #[serde(default, rename = "孔内水位")]
    pub borehole_water_levels: Vec<BoreholeWaterLevel400>, // v4.00で水位種別コード削除
    // 掘削工程 → 削孔工程
    #[serde(default, rename = "削孔工程")]
    pub drilling_processes: Vec<DrillingProcess400>, // v4.00でフィールド名変更
    #[serde(default, rename = "孔径孔壁保護")]
    pub borehole_diameter_protections: Vec<BoreholeDiameterProtection210>, // v2.10を流用
    // 掘進速度 → 削孔速度
    #[serde(default, rename = "削孔速度")]
    pub drilling_speeds: Vec<DrillingSpeed400>, // v4.00でフィールド名変更
    #[serde(default, rename = "コアチューブビット")]
    pub core_tube_bits: Vec<CoreTubeBit210>, // v2.10を流用
    #[serde(default, rename = "給圧条件")]
    pub pressure_conditions: Vec<PressureCondition210>, // v2.10を流用
    #[serde(default, rename = "回転数")]
    pub rotation_speeds: Vec<RotationSpeed210>, // v2.10を流用
    #[serde(default, rename = "送水条件")]
    pub water_supply_conditions: Vec<WaterSupplyCondition210>, // v2.10を流用
    #[serde(default, rename = "断層破砕帯区分")]
    pub fault_fracture_zone_classifications: Vec<FaultFractureZoneClassification210>, // v2.10を流用
    #[serde(default, rename = "コア採取率")]
    pub core_recovery_rates: Vec<CoreRecoveryRate300>, // v3.00を流用
    #[serde(default, rename = "最大コア長")]
    pub maximum_core_lengths: Vec<MaximumCoreLength300>, // v3.00を流用
    #[serde(default, rename = "RQD")]
    pub rqds: Vec<RQD210>, // v2.10を流用
    // コア質量（v4.00で新規追加）
    #[serde(default, rename = "コア質量")]
    pub core_masses: Vec<CoreMass400>,
    #[serde(default, rename = "岩級区分判定表")]
    pub rock_class_classification_table: Vec<RockClassClassificationTable210>, // v2.10を流用
    #[serde(default, rename = "岩級区分")]
    pub rock_class_classifications: Vec<RockClassClassification300>, // v3.00を流用
    #[serde(default, rename = "保孔管")]
    pub casing_pipes: Vec<CasingPipe210>, // v2.10を流用
    #[serde(default, rename = "計測機器")]
    pub measuring_instruments: Vec<MeasuringInstrument210>, // v2.10を流用
    // 地下水検層試験 → トレーサーによる地下水流動層検層
    #[serde(default, rename = "トレーサーによる地下水流動層検層")]
    pub tracer_groundwater_flow_layer_loggings: Vec<TracerGroundwaterFlowLayerLogging400>, // v4.00で名称変更
    #[serde(default, rename = "トレーサーによる地下水流動層検層詳細データ")]
    pub tracer_groundwater_flow_layer_logging_details:
        Vec<TracerGroundwaterFlowLayerLoggingDetail400>, // v4.00で名称変更
    #[serde(default, rename = "トレーサーによる地下水流動層検層判定結果")]
    pub tracer_groundwater_flow_layer_logging_results:
        Vec<TracerGroundwaterFlowLayerLoggingResult400>, // v4.00で名称変更
    #[serde(default, rename = "備考")]
    pub remarks: Vec<Remark210>, // v2.10を流用
    #[serde(default, rename = "フリー情報")]
    pub free_info: Vec<FreeInfo>,
}

// ============================================================================
// 工学的地質区分名現場土質名 - v4.00で岩石土区分から名称変更
// 構造はv3.00の岩石土区分と同じ
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineeringGeologyClassification400 {
    #[serde(rename = "工学的地質区分名現場土質名_下端深度")]
    pub depth: Option<String>,
    #[serde(rename = "工学的地質区分名現場土質名_工学的地質区分名現場土質名")]
    pub name: Option<String>,
    #[serde(
        default,
        rename = "工学的地質区分名現場土質名_工学的地質区分名現場土質名記号"
    )]
    pub symbol: Option<String>,
    #[serde(rename = "工学的地質区分名現場土質名_岩石群")]
    pub rock_groups: Vec<RockGroup400>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockGroup400 {
    #[serde(rename = "工学的地質区分名現場土質名_岩石群コード")]
    pub rock_group_code: Option<String>,
    #[serde(rename = "工学的地質区分名現場土質名_岩石土コード")]
    pub rock_soil_codes: Vec<RockSoilCode400>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RockSoilCode400 {
    #[serde(default, rename = "工学的地質区分名現場土質名_岩相")]
    pub lithofacies: Option<String>,
    #[serde(default, rename = "工学的地質区分名現場土質名_岩石")]
    pub rock: Option<String>,
    #[serde(default, rename = "工学的地質区分名現場土質名_変成岩岩相")]
    pub metamorphic_rock_lithofacies: Option<String>,
    #[serde(default, rename = "工学的地質区分名現場土質名_変成岩岩石")]
    pub metamorphic_rock: Option<String>,
}

// ============================================================================
// 標準貫入試験 - v4.00で単位変更（cm → mm）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardPenetrationTest400 {
    #[serde(rename = "標準貫入試験_開始深度")]
    pub start_depth: Option<String>,
    // 単位変更: 0-10cm → 0-100mm
    #[serde(default, rename = "標準貫入試験_0_100打撃回数")]
    pub hit_count_0_100: Option<String>,
    #[serde(default, rename = "標準貫入試験_0_100貫入量")]
    pub penetration_0_100: Option<String>,
    // 単位変更: 10-20cm → 100-200mm
    #[serde(default, rename = "標準貫入試験_100_200打撃回数")]
    pub hit_count_100_200: Option<String>,
    #[serde(default, rename = "標準貫入試験_100_200貫入量")]
    pub penetration_100_200: Option<String>,
    // 単位変更: 20-30cm → 200-300mm
    #[serde(default, rename = "標準貫入試験_200_300打撃回数")]
    pub hit_count_200_300: Option<String>,
    #[serde(default, rename = "標準貫入試験_200_300貫入量")]
    pub penetration_200_300: Option<String>,
    #[serde(rename = "標準貫入試験_合計打撃回数")]
    pub total_hit_count: Option<String>,
    #[serde(rename = "標準貫入試験_合計貫入量")]
    pub total_penetration: Option<String>,
    #[serde(default, rename = "標準貫入試験_備考")]
    pub remarks: Option<String>,
}

// ============================================================================
// 破砕度 - v4.00で新規追加
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CrushingDegreeClassificationTable400 {
    #[serde(default, rename = "破砕度判定表_項目名")]
    pub item_names: Vec<String>,
    #[serde(default, rename = "破砕度判定表_判定")]
    pub judgments: Vec<CrushingDegreeJudgment400>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrushingDegreeJudgment400 {
    #[serde(rename = "破砕度判定表_コード")]
    pub code: Option<String>,
    #[serde(rename = "破砕度判定表_記号")]
    pub symbol: Option<String>,
    #[serde(default, rename = "破砕度判定表_説明")]
    pub descriptions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrushingDegree400 {
    #[serde(rename = "破砕度_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "破砕度_下端深度")]
    pub end_depth: Option<String>,
    #[serde(default, rename = "破砕度_破砕度")]
    pub crushing_degree: Option<String>,
}

// ============================================================================
// コア質量 - v4.00で新規追加
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreMass400 {
    #[serde(rename = "コア質量_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "コア質量_コア質量")]
    pub core_mass: Option<String>,
}

// ============================================================================
// 孔内水位 - v4.00で水位種別コード削除、掘削状況→削孔状況
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BoreholeWaterLevel400 {
    #[serde(rename = "孔内水位_測定年月日")]
    pub measurement_date: Option<String>,
    #[serde(rename = "孔内水位_削孔状況コード")] // v3.00: 掘削状況コード
    pub drilling_status_code: Option<String>,
    #[serde(rename = "孔内水位_削孔状況")] // v3.00: 掘削状況
    pub drilling_status: Option<String>,
    #[serde(rename = "孔内水位_孔内水位")]
    pub water_level: Option<String>,
    // v4.00で削除: 孔内水位_水位種別コード
    #[serde(default, rename = "孔内水位_水位種別備考")]
    pub water_level_type_remarks: Option<String>,
}

// ============================================================================
// 削孔工程 - v4.00で掘削工程から名称変更、掘進深度→削孔深度
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillingProcess400 {
    #[serde(rename = "削孔工程_開始年月日")] // v3.00: 掘削工程_開始年月日
    pub start_date: Option<String>,
    #[serde(rename = "削孔工程_終了年月日")] // v3.00: 掘削工程_終了年月日
    pub end_date: Option<String>,
    #[serde(rename = "削孔工程_削孔深度")] // v3.00: 掘削工程_掘進深度
    pub drilling_depth: Option<String>,
    #[serde(default, rename = "削孔工程_作業内容")] // v3.00: 掘削工程_作業内容
    pub work_content: Option<String>,
}

// ============================================================================
// 削孔速度 - v4.00で掘進速度から名称変更
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillingSpeed400 {
    #[serde(rename = "削孔速度_上端深度")] // v3.00: 掘進速度_上端深度
    pub start_depth: Option<String>,
    #[serde(rename = "削孔速度_下端深度")] // v3.00: 掘進速度_下端深度
    pub end_depth: Option<String>,
    #[serde(default, rename = "削孔速度_削孔速度")] // v3.00: 掘進速度_掘進速度
    pub drilling_speed: Option<String>,
}

// ============================================================================
// トレーサーによる地下水流動層検層 - v4.00で地下水検層試験から名称変更
// 掘削深度→削孔深度
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TracerGroundwaterFlowLayerLogging400 {
    #[serde(rename = "トレーサーによる地下水流動層検層_検層番号")]
    // v3.00: 地下水検層試験_試験番号
    pub test_number: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層_削孔深度")] // v3.00: 掘削深度
    pub drilling_depth: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層_孔内水位")]
    pub borehole_water_level: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層_検層方法コード")] // v3.00: 試験方法コード
    pub test_method_code: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層_電解質溶液濃度")]
    pub electrolyte_concentration: Option<String>,
    #[serde(default, rename = "トレーサーによる地下水流動層検層_測定時間")]
    pub measurement_times: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TracerGroundwaterFlowLayerLoggingDetail400 {
    #[serde(rename = "トレーサーによる地下水流動層検層詳細データ_検層番号")] // v3.00: 試験番号
    pub test_number: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層詳細データ_測定深度")]
    pub measurement_depth: Option<String>,
    #[serde(
        default,
        rename = "トレーサーによる地下水流動層検層詳細データ_比抵抗値"
    )]
    pub resistivity_values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TracerGroundwaterFlowLayerLoggingResult400 {
    #[serde(rename = "トレーサーによる地下水流動層検層判定結果_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層判定結果_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "トレーサーによる地下水流動層検層判定結果_地下水流動層検層結果")]
    pub result: Option<String>,
}
