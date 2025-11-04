use serde::{Deserialize, Serialize};

// ============================================================================
// ルート構造体
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ボーリング情報")]
pub struct Boring210 {
    #[serde(rename = "@DTD_version")]
    pub dtd_version: Option<String>,
    #[serde(rename = "標題情報")]
    pub title: Title210,
    #[serde(rename = "コア情報")]
    pub core: Core210,
}

// ============================================================================
// 標題情報
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Title210 {
    #[serde(rename = "調査基本情報")]
    pub basic_info: BasicSurvey210,
    #[serde(rename = "経度緯度情報")]
    pub longitude_latitude: LngLat210,
    #[serde(default, rename = "ローカル座標")] // * (0回以上)
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
    #[serde(default, rename = "ハンマー落下用具")] // ? (オプション)
    pub hammer_drop_tool: Option<HammerDropTool210>,
    #[serde(default, rename = "N値記録用具")] // ? (オプション)
    pub n_value_recorder: Option<NValueRecorder210>,
    #[serde(rename = "ポンプ")]
    pub pump: Pump210,
    #[serde(default, rename = "櫓種類")] // ? (オプション)
    pub tower_type: Option<TowerType210>,
}

// ============================================================================
// 調査基本情報
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicSurvey210 {
    #[serde(default, rename = "事業工事名")] // オプション
    pub project_name: Option<String>,
    #[serde(rename = "調査名")] // 必須
    pub survey_name: Option<String>,
    #[serde(rename = "調査目的")] // 必須
    pub survey_purpose: Option<String>,
    #[serde(rename = "調査対象")] // 必須
    pub survey_target: Option<String>,
    #[serde(rename = "ボーリング名")] // 必須
    pub boring_name: Option<String>,
    #[serde(rename = "ボーリング総数")] // 必須
    pub total_boring: Option<String>,
    #[serde(rename = "ボーリング連番")] // 必須
    pub boring_serial: Option<String>,
}

// ============================================================================
// 経度緯度情報
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LngLat210 {
    #[serde(rename = "経度_度")] // 必須
    pub longitude_degree: Option<String>,
    #[serde(rename = "経度_分")] // 必須
    pub longitude_minute: Option<String>,
    #[serde(rename = "経度_秒")] // 必須
    pub longitude_second: Option<String>,
    #[serde(rename = "緯度_度")] // 必須
    pub latitude_degree: Option<String>,
    #[serde(rename = "緯度_分")] // 必須
    pub latitude_minute: Option<String>,
    #[serde(rename = "緯度_秒")] // 必須
    pub latitude_second: Option<String>,
    #[serde(rename = "取得方法コード")] // 必須
    pub acquisition_method_code: Option<String>,
    #[serde(default, rename = "取得方法説明")] // オプション（新規）
    pub acquisition_method_description: Option<String>,
    #[serde(rename = "読取精度コード")] // 必須
    pub reading_precision_code: Option<String>,
    #[serde(rename = "測地系")] // 必須
    pub geodetic_system: Option<String>,
}

// ============================================================================
// ローカル座標（構造が変更）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalCoordinate210 {
    #[serde(rename = "座標定義")] // 必須
    pub definition: Option<String>,
    #[serde(rename = "座標")] // 必須
    pub coordinate: Option<String>,
}

// ============================================================================
// 調査位置（v1.10と同じ）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyPosition210 {
    #[serde(rename = "調査位置住所")] // 必須
    pub address: Option<String>,
    #[serde(rename = "コード1次")] // 必須
    pub code1: Option<String>,
    #[serde(rename = "コード2次")] // 必須
    pub code2: Option<String>,
    #[serde(rename = "コード3次")] // 必須
    pub code3: Option<String>,
}

// ============================================================================
// 発注機関（v1.10と同じ）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInstitution210 {
    #[serde(rename = "発注機関名称")] // 必須
    pub name: Option<String>,
    #[serde(rename = "テクリスコード")] // 必須
    pub code: Option<String>,
}

// ============================================================================
// 調査期間（構造が変更：年月日統合）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyPeriod210 {
    #[serde(rename = "調査期間_開始年月日")] // 必須
    pub start_date: Option<String>,
    #[serde(rename = "調査期間_終了年月日")] // 必須
    pub end_date: Option<String>,
}

// ============================================================================
// 調査会社（v1.10と同じ）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyCompany210 {
    #[serde(rename = "調査会社_名称")] // 必須
    pub name: Option<String>,
    #[serde(rename = "調査会社_TEL")] // 必須
    pub tel: Option<String>,
    #[serde(rename = "調査会社_主任技師")] // 必須
    pub chief_engineer: Option<String>,
    #[serde(rename = "調査会社_現場代理人")] // 必須
    pub field_agent: Option<String>,
    #[serde(rename = "調査会社_コア鑑定者")] // 必須
    pub core_appraiser: Option<String>,
    #[serde(rename = "調査会社_ボーリング責任者")] // 必須
    pub boring_manager: Option<String>,
}

// ============================================================================
// ボーリング基本情報
// DTD: <!ELEMENT ボーリング基本情報 (孔口標高, 総掘進長, 柱状図様式,
//      掘進角度?, 掘進方向?, 地盤勾配?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BoringBasicInfo210 {
    #[serde(rename = "孔口標高")] // 必須
    pub elevation: Option<String>,
    #[serde(rename = "総掘進長")] // 必須
    pub total_length: Option<String>,
    #[serde(rename = "柱状図様式")] // 必須（DTDでは「柱状図様式」）
    pub columnar_section_style: Option<String>,
    #[serde(default, rename = "掘進角度")] // オプション
    pub drilling_angle: Option<String>,
    #[serde(default, rename = "掘進方向")] // オプション
    pub drilling_direction: Option<String>,
    #[serde(default, rename = "地盤勾配")] // オプション（v2.10で変更）
    pub ground_slope: Option<String>,
}

// ============================================================================
// 試錐機
// DTD: <!ELEMENT 試錐機 (試錐機_名称, 試錐機_能力?, 試錐機_方法?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillingMachine210 {
    #[serde(rename = "試錐機_名称")] // 必須
    pub name: Option<String>,
    #[serde(default, rename = "試錐機_能力")] // オプション（DTDで変更）
    pub capacity: Option<String>,
    #[serde(default, rename = "試錐機_方法")] // オプション（DTDで変更）
    pub method: Option<String>,
}

// ============================================================================
// エンジン
// DTD: <!ELEMENT エンジン (エンジン_名称, エンジン_能力?, エンジン_単位?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine210 {
    #[serde(rename = "エンジン_名称")] // 必須
    pub name: Option<String>,
    #[serde(default, rename = "エンジン_能力")] // オプション（DTDで変更）
    pub capacity: Option<String>,
    #[serde(default, rename = "エンジン_単位")] // オプション（DTDで変更）
    pub unit: Option<String>,
}

// ============================================================================
// ハンマー落下用具（v1.10と同じ）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct HammerDropTool210 {
    #[serde(rename = "ハンマー落下用具_コード")] // 必須
    pub code: Option<String>,
    #[serde(default, rename = "ハンマー落下用具_名称")] // オプション（v2.10で変更）
    pub name: Option<String>,
}

// ============================================================================
// N値記録用具（v1.10と同じ）
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct NValueRecorder210 {
    #[serde(rename = "N値記録用具_コード")] // 必須
    pub code: Option<String>,
    #[serde(default, rename = "N値記録用具_名称")] // オプション（v2.10で変更）
    pub name: Option<String>,
}

// ============================================================================
// ポンプ
// DTD: <!ELEMENT ポンプ (ポンプ_名称, ポンプ_能力?, ポンプ_単位?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Pump210 {
    #[serde(rename = "ポンプ_名称")] // 必須
    pub name: Option<String>,
    #[serde(default, rename = "ポンプ_能力")] // オプション（DTDで変更）
    pub capacity: Option<String>,
    #[serde(default, rename = "ポンプ_単位")] // オプション（DTDで変更）
    pub unit: Option<String>,
}

// ============================================================================
// 櫓種類（新規）
// DTD: <!ELEMENT 櫓種類 (櫓種類コード?, 櫓種類名称?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TowerType210 {
    #[serde(default, rename = "櫓種類コード")] // オプション
    pub code: Option<String>,
    #[serde(default, rename = "櫓種類名称")] // オプション
    pub name: Option<String>,
}

// ============================================================================
// コア情報
// DTD: <!ELEMENT コア情報 (土質岩種区分+, 色調*, 観察記事*, 観察記事枠線*, 標準貫入試験*,
//      標準貫入試験詳細データ*, ルジオン試験*, ルジオン試験詳細データ*, 相対密度稠度*,
//      硬軟区分判定表*, 硬軟区分*, コア形状区分判定表*, コア形状区分*, 割れ目区分判定表*,
//      割れ目区分*, 風化区分判定表*, 風化区分*, 変質区分判定表*, 変質区分*,
//      孔内水平載荷試験*, 透水試験*, P波試験*, S波試験*, その他原位置試験*, 試料採取*,
//      地盤材料の工学的分類*, 地質時代*, 地層岩体区分*, 孔内水位*, 掘削工程*,
//      孔径孔壁保護*, 掘進速度*, コアチューブビット*, 給圧条件*, 回転数*, 送水条件*,
//      断層破砕帯区分*, コア採取率*, 最大コア長*, RQD*, 岩級区分判定表*, 岩級区分*,
//      保孔管*, 計測機器*, 地下水検層試験*, 地下水検層試験詳細データ*,
//      地下水検層試験判定結果*, 備考*, フリー情報*)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Core210 {
    #[serde(rename = "土質岩種区分")] // + (1回以上)
    pub soil_rock_classifications: Vec<SoilRockClassification210>,
    #[serde(default, rename = "色調")] // * (0回以上)
    pub colors: Vec<Color210>,
    #[serde(default, rename = "観察記事")] // * (0回以上)
    pub observational_articles: Vec<ObservationalArticle210>,
    #[serde(default, rename = "観察記事枠線")] // * (0回以上)
    pub observational_article_frames: Vec<ObservationalArticleFrame210>,
    #[serde(default, rename = "標準貫入試験")] // * (0回以上)
    pub standard_penetration_tests: Vec<StandardPenetrationTest210>,
    #[serde(default, rename = "標準貫入試験詳細データ")] // * (0回以上)
    pub standard_penetration_test_details: Vec<StandardPenetrationTestDetail210>,
    #[serde(default, rename = "ルジオン試験")] // * (0回以上)
    pub lugeon_tests: Vec<LugeonTest210>,
    #[serde(default, rename = "ルジオン試験詳細データ")] // * (0回以上)
    pub lugeon_test_details: Vec<LugeonTestDetail210>,
    #[serde(default, rename = "相対密度稠度")] // * (0回以上)
    pub relative_density_consistency: Vec<RelativeDensityConsistency210>,
    #[serde(default, rename = "硬軟区分判定表")] // * (0回以上)
    pub hardness_classification_table: Vec<HardnessClassificationTable210>,
    #[serde(default, rename = "硬軟区分")] // * (0回以上)
    pub hardness_classifications: Vec<HardnessClassification210>,
    #[serde(default, rename = "コア形状区分判定表")] // * (0回以上)
    pub core_shape_classification_table: Vec<CoreShapeClassificationTable210>,
    #[serde(default, rename = "コア形状区分")] // * (0回以上)
    pub core_shape_classifications: Vec<CoreShapeClassification210>,
    #[serde(default, rename = "割れ目区分判定表")] // * (0回以上)
    pub fracture_classification_table: Vec<FractureClassificationTable210>,
    #[serde(default, rename = "割れ目区分")] // * (0回以上)
    pub fracture_classifications: Vec<FractureClassification210>,
    #[serde(default, rename = "風化区分判定表")] // * (0回以上)
    pub weathering_classification_table: Vec<WeatheringClassificationTable210>,
    #[serde(default, rename = "風化区分")] // * (0回以上)
    pub weathering_classifications: Vec<WeatheringClassification210>,
    #[serde(default, rename = "変質区分判定表")] // * (0回以上)
    pub alteration_classification_table: Vec<AlterationClassificationTable210>,
    #[serde(default, rename = "変質区分")] // * (0回以上)
    pub alteration_classifications: Vec<AlterationClassification210>,
    #[serde(default, rename = "孔内水平載荷試験")] // * (0回以上)
    pub borehole_horizontal_load_tests: Vec<BoreholeHorizontalLoadTest210>,
    #[serde(default, rename = "透水試験")] // * (0回以上)
    pub permeability_tests: Vec<PermeabilityTest210>,
    #[serde(default, rename = "P波試験")] // * (0回以上)
    pub p_wave_tests: Vec<PWaveTest210>,
    #[serde(default, rename = "S波試験")] // * (0回以上)
    pub s_wave_tests: Vec<SWaveTest210>,
    #[serde(default, rename = "その他原位置試験")] // * (0回以上)
    pub other_in_situ_tests: Vec<OtherInSituTest210>,
    #[serde(default, rename = "試料採取")] // * (0回以上)
    pub sample_collections: Vec<SampleCollection210>,
    #[serde(default, rename = "地盤材料の工学的分類")] // * (0回以上)
    pub ground_material_classifications: Vec<GroundMaterialClassification210>,
    #[serde(default, rename = "地質時代")] // * (0回以上)
    pub geological_ages: Vec<GeologicalAge210>,
    #[serde(default, rename = "地層岩体区分")] // * (0回以上)
    pub stratum_rock_classifications: Vec<StratumRockClassification210>,
    #[serde(default, rename = "孔内水位")] // * (0回以上)
    pub borehole_water_levels: Vec<BoreholeWaterLevel210>,
    #[serde(default, rename = "掘削工程")] // * (0回以上)
    pub drilling_processes: Vec<DrillingProcess210>,
    #[serde(default, rename = "孔径孔壁保護")] // * (0回以上)
    pub borehole_diameter_protections: Vec<BoreholeDiameterProtection210>,
    #[serde(default, rename = "掘進速度")] // * (0回以上)
    pub drilling_speeds: Vec<DrillingSpeed210>,
    #[serde(default, rename = "コアチューブビット")] // * (0回以上)
    pub core_tube_bits: Vec<CoreTubeBit210>,
    #[serde(default, rename = "給圧条件")] // * (0回以上)
    pub pressure_conditions: Vec<PressureCondition210>,
    #[serde(default, rename = "回転数")] // * (0回以上)
    pub rotation_speeds: Vec<RotationSpeed210>,
    #[serde(default, rename = "送水条件")] // * (0回以上)
    pub water_supply_conditions: Vec<WaterSupplyCondition210>,
    #[serde(default, rename = "断層破砕帯区分")] // * (0回以上)
    pub fault_fracture_zone_classifications: Vec<FaultFractureZoneClassification210>,
    #[serde(default, rename = "コア採取率")] // * (0回以上)
    pub core_recovery_rates: Vec<CoreRecoveryRate210>,
    #[serde(default, rename = "最大コア長")] // * (0回以上)
    pub maximum_core_lengths: Vec<MaximumCoreLength210>,
    #[serde(default, rename = "RQD")] // * (0回以上)
    pub rqds: Vec<RQD210>,
    #[serde(default, rename = "岩級区分判定表")] // * (0回以上)
    pub rock_class_classification_table: Vec<RockClassClassificationTable210>,
    #[serde(default, rename = "岩級区分")] // * (0回以上)
    pub rock_class_classifications: Vec<RockClassClassification210>,
    #[serde(default, rename = "保孔管")] // * (0回以上)
    pub casing_pipes: Vec<CasingPipe210>,
    #[serde(default, rename = "計測機器")] // * (0回以上)
    pub measuring_instruments: Vec<MeasuringInstrument210>,
    #[serde(default, rename = "地下水検層試験")] // * (0回以上)
    pub groundwater_logging_tests: Vec<GroundwaterLoggingTest210>,
    #[serde(default, rename = "地下水検層試験詳細データ")] // * (0回以上)
    pub groundwater_logging_test_details: Vec<GroundwaterLoggingTestDetail210>,
    #[serde(default, rename = "地下水検層試験判定結果")] // * (0回以上)
    pub groundwater_logging_test_results: Vec<GroundwaterLoggingTestResult210>,
    #[serde(default, rename = "備考")] // * (0回以上)
    pub remarks: Vec<Remark210>,
    #[serde(default, rename = "フリー情報")] // * (0回以上)
    pub free_info: Vec<String>,
}

// ============================================================================
// 土質岩種区分（地質区分から名称変更）
// DTD: <!ELEMENT 土質岩種区分 (土質岩種区分_下端深度, 土質岩種区分_土質岩種区分1,
//      土質岩種区分_土質岩種記号1?, 土質岩種区分_分類コード1,
//      土質岩種区分_土質岩種区分2?, 土質岩種区分_土質岩種記号2?, 土質岩種区分_分類コード2?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilRockClassification210 {
    #[serde(rename = "土質岩種区分_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "土質岩種区分_土質岩種区分1")] // 必須
    pub classification1: Option<String>,
    #[serde(default, rename = "土質岩種区分_土質岩種記号1")] // オプション
    pub code1: Option<String>,
    #[serde(rename = "土質岩種区分_分類コード1")] // 必須
    pub classification_code1: Option<String>,
    #[serde(default, rename = "土質岩種区分_土質岩種区分2")] // オプション
    pub classification2: Option<String>,
    #[serde(default, rename = "土質岩種区分_土質岩種記号2")] // オプション
    pub code2: Option<String>,
    #[serde(default, rename = "土質岩種区分_分類コード2")] // オプション
    pub classification_code2: Option<String>,
}

// ============================================================================
// 色調
// DTD: <!ELEMENT 色調 (色調_下端深度, 色調_色調名)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Color210 {
    #[serde(rename = "色調_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "色調_色調名")] // 必須（DTDでは色調_状態→色調_色調名に変更）
    pub color_name: Option<String>,
}

// ============================================================================
// 観察記事
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationalArticle210 {
    #[serde(rename = "観察記事_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "観察記事_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(default, rename = "観察記事_記事")] // オプション（v2.10で変更）
    pub observation: Option<String>,
}

// ============================================================================
// 観察記事枠線
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationalArticleFrame210 {
    #[serde(rename = "観察記事枠線_下端深度")] // 必須
    pub end_depth: Option<String>,
}

// ============================================================================
// 標準貫入試験
// DTD: <!ELEMENT 標準貫入試験 (標準貫入試験_開始深度, 標準貫入試験_0_10打撃回数?,
//      標準貫入試験_0_10貫入量?, 標準貫入試験_10_20打撃回数?, 標準貫入試験_10_20貫入量?,
//      標準貫入試験_20_30打撃回数?, 標準貫入試験_20_30貫入量?, 標準貫入試験_合計打撃回数,
//      標準貫入試験_合計貫入量, 標準貫入試験_備考?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardPenetrationTest210 {
    #[serde(rename = "標準貫入試験_開始深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(default, rename = "標準貫入試験_0_10打撃回数")] // オプション
    pub hits_0_10: Option<String>,
    #[serde(default, rename = "標準貫入試験_0_10貫入量")] // オプション
    pub penetration_0_10: Option<String>,
    #[serde(default, rename = "標準貫入試験_10_20打撃回数")] // オプション
    pub hits_10_20: Option<String>,
    #[serde(default, rename = "標準貫入試験_10_20貫入量")] // オプション
    pub penetration_10_20: Option<String>,
    #[serde(default, rename = "標準貫入試験_20_30打撃回数")] // オプション
    pub hits_20_30: Option<String>,
    #[serde(default, rename = "標準貫入試験_20_30貫入量")] // オプション
    pub penetration_20_30: Option<String>,
    #[serde(rename = "標準貫入試験_合計打撃回数")] // 必須
    pub total_hits: Option<String>,
    #[serde(rename = "標準貫入試験_合計貫入量")] // 必須
    pub total_penetration: Option<String>,
    #[serde(default, rename = "標準貫入試験_備考")] // オプション
    pub remarks: Option<String>,
}

// ============================================================================
// 標準貫入試験詳細データ
// DTD: <!ELEMENT 標準貫入試験詳細データ (標準貫入試験詳細データ_開始深度, 標準貫入試験詳細データ_打撃*)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardPenetrationTestDetail210 {
    #[serde(rename = "標準貫入試験詳細データ_開始深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(default, rename = "標準貫入試験詳細データ_打撃")] // * (0回以上)
    pub blows: Vec<StandardPenetrationTestBlow210>,
}

// ============================================================================
// 標準貫入試験詳細データ_打撃
// DTD: <!ELEMENT 標準貫入試験詳細データ_打撃 (標準貫入試験詳細データ_打撃回数,
//      標準貫入試験詳細データ_貫入量, 標準貫入試験詳細データ_累積貫入量, 標準貫入試験詳細データ_備考?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardPenetrationTestBlow210 {
    #[serde(rename = "標準貫入試験詳細データ_打撃回数")] // 必須
    pub blow_count: Option<String>,
    #[serde(rename = "標準貫入試験詳細データ_貫入量")] // 必須
    pub penetration: Option<String>,
    #[serde(rename = "標準貫入試験詳細データ_累積貫入量")] // 必須
    pub cumulative_penetration: Option<String>,
    #[serde(default, rename = "標準貫入試験詳細データ_備考")] // オプション
    pub remarks: Option<String>,
}

// ============================================================================
// ルジオン試験
// DTD: <!ELEMENT ルジオン試験 (ルジオン試験_試験番号, ルジオン試験_上端深度, ルジオン試験_下端深度,
//      ルジオン試験_圧力管理方法コード, ルジオン試験_圧力管理方法?, ルジオン試験_圧力最大スケール,
//      ルジオン試験_注入量最大スケール, ルジオン試験_圧力開始点, ルジオン試験_注入量開始点,
//      ルジオン試験_ルジオン値区分, ルジオン試験_ルジオン値, ルジオン試験_限界圧力)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LugeonTest210 {
    #[serde(rename = "ルジオン試験_試験番号")] // 必須
    pub test_number: Option<String>,
    #[serde(rename = "ルジオン試験_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "ルジオン試験_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "ルジオン試験_圧力管理方法コード")] // 必須
    pub pressure_management_code: Option<String>,
    #[serde(default, rename = "ルジオン試験_圧力管理方法")] // オプション
    pub pressure_management: Option<String>,
    #[serde(rename = "ルジオン試験_圧力最大スケール")] // 必須
    pub pressure_max_scale: Option<String>,
    #[serde(rename = "ルジオン試験_注入量最大スケール")] // 必須
    pub injection_max_scale: Option<String>,
    #[serde(rename = "ルジオン試験_圧力開始点")] // 必須
    pub pressure_start_point: Option<String>,
    #[serde(rename = "ルジオン試験_注入量開始点")] // 必須
    pub injection_start_point: Option<String>,
    #[serde(rename = "ルジオン試験_ルジオン値区分")] // 必須
    pub lugeon_value_classification: Option<String>,
    #[serde(rename = "ルジオン試験_ルジオン値")] // 必須
    pub lugeon_value: Option<String>,
    #[serde(rename = "ルジオン試験_限界圧力")] // 必須
    pub limit_pressure: Option<String>,
}

// ============================================================================
// ルジオン試験詳細データ
// DTD: <!ELEMENT ルジオン試験詳細データ (ルジオン試験詳細データ_試験番号,
//      ルジオン試験詳細データ_有効圧力, ルジオン試験詳細データ_注入量)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LugeonTestDetail210 {
    #[serde(rename = "ルジオン試験詳細データ_試験番号")] // 必須
    pub test_number: Option<String>,
    #[serde(rename = "ルジオン試験詳細データ_有効圧力")] // 必須
    pub effective_pressure: Option<String>,
    #[serde(rename = "ルジオン試験詳細データ_注入量")] // 必須
    pub injection_amount: Option<String>,
}

// ============================================================================
// 相対密度稠度
// DTD: <!ELEMENT 相対密度稠度 (相対密度稠度_下端深度, 相対密度_コード?,
//      相対密度_状態?, 相対稠度_コード?, 相対稠度_状態?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeDensityConsistency210 {
    #[serde(rename = "相対密度稠度_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(default, rename = "相対密度_コード")] // オプション
    pub relative_density_code: Option<String>,
    #[serde(default, rename = "相対密度_状態")] // オプション
    pub relative_density_state: Option<String>,
    #[serde(default, rename = "相対稠度_コード")] // オプション
    pub relative_consistency_code: Option<String>,
    #[serde(default, rename = "相対稠度_状態")] // オプション
    pub relative_consistency_state: Option<String>,
}

// ============================================================================
// 硬軟区分判定表
// DTD: <!ELEMENT 硬軟区分判定表 (硬軟区分判定表_コード, 硬軟区分判定表_記号,
//      硬軟区分判定表_区分?, 硬軟区分判定表_説明?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct HardnessClassificationTable210 {
    #[serde(rename = "硬軟区分判定表_コード")] // 必須
    pub code: Option<String>,
    #[serde(rename = "硬軟区分判定表_記号")] // 必須
    pub symbol: Option<String>,
    #[serde(default, rename = "硬軟区分判定表_区分")] // オプション
    pub classification: Option<String>,
    #[serde(default, rename = "硬軟区分判定表_説明")] // オプション
    pub description: Option<String>,
}

// ============================================================================
// 硬軟区分
// DTD: <!ELEMENT 硬軟区分 (硬軟区分_下端深度, 硬軟区分_硬軟区分)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct HardnessClassification210 {
    #[serde(rename = "硬軟区分_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "硬軟区分_硬軟区分")] // 必須
    pub classification: Option<String>,
}

// ============================================================================
// コア形状区分判定表
// DTD: <!ELEMENT コア形状区分判定表 (コア形状区分判定表_コード, コア形状区分判定表_記号,
//      コア形状区分判定表_区分?, コア形状区分判定表_説明?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreShapeClassificationTable210 {
    #[serde(rename = "コア形状区分判定表_コード")] // 必須
    pub code: Option<String>,
    #[serde(rename = "コア形状区分判定表_記号")] // 必須
    pub symbol: Option<String>,
    #[serde(default, rename = "コア形状区分判定表_区分")] // オプション
    pub classification: Option<String>,
    #[serde(default, rename = "コア形状区分判定表_説明")] // オプション
    pub description: Option<String>,
}

// ============================================================================
// コア形状区分
// DTD: <!ELEMENT コア形状区分 (コア形状区分_下端深度, コア形状区分_コア形状区分)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreShapeClassification210 {
    #[serde(rename = "コア形状区分_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "コア形状区分_コア形状区分")] // 必須
    pub classification: Option<String>,
}

// ============================================================================
// 割れ目区分判定表
// DTD: <!ELEMENT 割れ目区分判定表 (割れ目区分判定表_コード, 割れ目区分判定表_記号,
//      割れ目区分判定表_区分?, 割れ目区分判定表_説明?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FractureClassificationTable210 {
    #[serde(rename = "割れ目区分判定表_コード")] // 必須
    pub code: Option<String>,
    #[serde(rename = "割れ目区分判定表_記号")] // 必須
    pub symbol: Option<String>,
    #[serde(default, rename = "割れ目区分判定表_区分")] // オプション
    pub classification: Option<String>,
    #[serde(default, rename = "割れ目区分判定表_説明")] // オプション
    pub description: Option<String>,
}

// ============================================================================
// 割れ目区分
// DTD: <!ELEMENT 割れ目区分 (割れ目区分_下端深度, 割れ目区分_割れ目区分)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FractureClassification210 {
    #[serde(rename = "割れ目区分_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "割れ目区分_割れ目区分")] // 必須
    pub classification: Option<String>,
}

// ============================================================================
// 風化区分判定表
// DTD: <!ELEMENT 風化区分判定表 (風化区分判定表_コード, 風化区分判定表_記号,
//      風化区分判定表_区分?, 風化区分判定表_説明?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatheringClassificationTable210 {
    #[serde(rename = "風化区分判定表_コード")] // 必須
    pub code: Option<String>,
    #[serde(rename = "風化区分判定表_記号")] // 必須
    pub symbol: Option<String>,
    #[serde(default, rename = "風化区分判定表_区分")] // オプション
    pub classification: Option<String>,
    #[serde(default, rename = "風化区分判定表_説明")] // オプション
    pub description: Option<String>,
}

// ============================================================================
// 風化区分
// DTD: <!ELEMENT 風化区分 (風化区分_下端深度, 風化区分_風化区分)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatheringClassification210 {
    #[serde(rename = "風化区分_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "風化区分_風化区分")] // 必須
    pub classification: Option<String>,
}

// ============================================================================
// 変質区分判定表
// DTD: <!ELEMENT 変質区分判定表 (変質区分判定表_コード, 変質区分判定表_記号,
//      変質区分判定表_区分?, 変質区分判定表_説明?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterationClassificationTable210 {
    #[serde(rename = "変質区分判定表_コード")] // 必須
    pub code: Option<String>,
    #[serde(rename = "変質区分判定表_記号")] // 必須
    pub symbol: Option<String>,
    #[serde(default, rename = "変質区分判定表_区分")] // オプション
    pub classification: Option<String>,
    #[serde(default, rename = "変質区分判定表_説明")] // オプション
    pub description: Option<String>,
}

// ============================================================================
// 変質区分
// DTD: <!ELEMENT 変質区分 (変質区分_下端深度, 変質区分_変質区分)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterationClassification210 {
    #[serde(rename = "変質区分_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "変質区分_変質区分")] // 必須
    pub classification: Option<String>,
}

// ============================================================================
// 孔内水平載荷試験
// DTD: <!ELEMENT 孔内水平載荷試験 (孔内水平載荷試験_試験深度, 孔内水平載荷試験_試験方法コード,
//      孔内水平載荷試験_試験方法?, 孔内水平載荷試験_載荷パターン?, 孔内水平載荷試験_初期圧,
//      孔内水平載荷試験_降伏圧, 孔内水平載荷試験_変形係数, 孔内水平載荷試験_割線弾性係数?,
//      孔内水平載荷試験_接線弾性係数?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BoreholeHorizontalLoadTest210 {
    #[serde(rename = "孔内水平載荷試験_試験深度")] // 必須
    pub test_depth: Option<String>,
    #[serde(rename = "孔内水平載荷試験_試験方法コード")] // 必須
    pub test_method_code: Option<String>,
    #[serde(default, rename = "孔内水平載荷試験_試験方法")] // オプション
    pub test_method: Option<String>,
    #[serde(default, rename = "孔内水平載荷試験_載荷パターン")] // オプション
    pub load_pattern: Option<String>,
    #[serde(rename = "孔内水平載荷試験_初期圧")] // 必須
    pub initial_pressure: Option<String>,
    #[serde(rename = "孔内水平載荷試験_降伏圧")] // 必須
    pub yield_pressure: Option<String>,
    #[serde(rename = "孔内水平載荷試験_変形係数")] // 必須
    pub deformation_modulus: Option<String>,
    #[serde(default, rename = "孔内水平載荷試験_割線弾性係数")] // オプション
    pub secant_modulus: Option<String>,
    #[serde(default, rename = "孔内水平載荷試験_接線弾性係数")] // オプション
    pub tangent_modulus: Option<String>,
}

// ============================================================================
// 透水試験
// DTD: <!ELEMENT 透水試験 (透水試験_上端深度, 透水試験_下端深度, 透水試験_試験コード,
//      透水試験_試験方法?, 透水試験_透水係数)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct PermeabilityTest210 {
    #[serde(rename = "透水試験_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "透水試験_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "透水試験_試験コード")] // 必須
    pub test_code: Option<String>,
    #[serde(default, rename = "透水試験_試験方法")] // オプション
    pub test_method: Option<String>,
    #[serde(rename = "透水試験_透水係数")] // 必須
    pub permeability_coefficient: Option<String>,
}

// ============================================================================
// P波試験
// DTD: <!ELEMENT P波試験 (P波試験_上端深度, P波試験_下端深度, P波試験_起振方式?, P波試験_速度)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct PWaveTest210 {
    #[serde(rename = "P波試験_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "P波試験_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(default, rename = "P波試験_起振方式")] // オプション
    pub excitation_method: Option<String>,
    #[serde(rename = "P波試験_速度")] // 必須
    pub velocity: Option<String>,
}

// ============================================================================
// S波試験
// DTD: <!ELEMENT S波試験 (S波試験_上端深度, S波試験_下端深度, S波試験_起振方式?, S波試験_速度)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SWaveTest210 {
    #[serde(rename = "S波試験_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "S波試験_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(default, rename = "S波試験_起振方式")] // オプション
    pub excitation_method: Option<String>,
    #[serde(rename = "S波試験_速度")] // 必須
    pub velocity: Option<String>,
}

// ============================================================================
// その他原位置試験
// DTD: <!ELEMENT その他原位置試験 (その他原位置試験_試験名, その他原位置試験_上端深度,
//      その他原位置試験_下端深度, その他原位置試験_試験結果)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherInSituTest210 {
    #[serde(rename = "その他原位置試験_試験名")] // 必須
    pub test_name: Option<String>,
    #[serde(rename = "その他原位置試験_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "その他原位置試験_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "その他原位置試験_試験結果")] // 必須
    pub test_result: Option<String>,
}

// ============================================================================
// 試料採取
// DTD: <!ELEMENT 試料採取 (試料採取_上端深度, 試料採取_下端深度, 試料採取_試料番号,
//      試料採取_採取方法コード, 試料採取_採取方法?, 試料採取_試験名*)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleCollection210 {
    #[serde(rename = "試料採取_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "試料採取_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "試料採取_試料番号")] // 必須
    pub sample_number: Option<String>,
    #[serde(rename = "試料採取_採取方法コード")] // 必須
    pub collection_method_code: Option<String>,
    #[serde(default, rename = "試料採取_採取方法")] // オプション
    pub collection_method: Option<String>,
    #[serde(default, rename = "試料採取_試験名")] // * (0回以上)
    pub test_names: Vec<String>,
}

// ============================================================================
// 地盤材料の工学的分類
// DTD: <!ELEMENT 地盤材料の工学的分類 (地盤分類_下端深度, 地盤分類_工学的分類記号?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundMaterialClassification210 {
    #[serde(rename = "地盤分類_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(default, rename = "地盤分類_工学的分類記号")] // オプション
    pub engineering_classification_symbol: Option<String>,
}

// ============================================================================
// 地質時代
// DTD: <!ELEMENT 地質時代 (地質時代_上端深度, 地質時代_下端深度, 地質時代_コード, 地質時代_時代名?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GeologicalAge210 {
    #[serde(rename = "地質時代_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "地質時代_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "地質時代_コード")] // 必須
    pub code: Option<String>,
    #[serde(default, rename = "地質時代_時代名")] // オプション
    pub age_name: Option<String>,
}

// ============================================================================
// 地層岩体区分
// DTD: <!ELEMENT 地層岩体区分 (地層岩体区分_上端深度, 地層岩体区分_下端深度, 地層岩体区分_地層岩体名?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct StratumRockClassification210 {
    #[serde(rename = "地層岩体区分_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "地層岩体区分_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(default, rename = "地層岩体区分_地層岩体名")] // オプション
    pub stratum_rock_name: Option<String>,
}

// ============================================================================
// 孔内水位
// DTD: <!ELEMENT 孔内水位 (孔内水位_測定年月日, 孔内水位_掘削状況コード, 孔内水位_掘削状況?,
//      孔内水位_孔内水位, 孔内水位_水位種別コード?, 孔内水位_水位種別備考?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BoreholeWaterLevel210 {
    #[serde(rename = "孔内水位_測定年月日")] // 必須
    pub measurement_date: Option<String>,
    #[serde(rename = "孔内水位_掘削状況コード")] // 必須
    pub drilling_status_code: Option<String>,
    #[serde(default, rename = "孔内水位_掘削状況")] // オプション
    pub drilling_status: Option<String>,
    #[serde(rename = "孔内水位_孔内水位")] // 必須
    pub water_level: Option<String>,
    #[serde(default, rename = "孔内水位_水位種別コード")] // オプション
    pub water_level_type_code: Option<String>,
    #[serde(default, rename = "孔内水位_水位種別備考")] // オプション
    pub water_level_type_remarks: Option<String>,
}

// ============================================================================
// 掘削工程
// DTD: <!ELEMENT 掘削工程 (掘削工程_測定年月日, 掘削工程_掘進深度, 掘削工程_ケーシング下端深度?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillingProcess210 {
    #[serde(rename = "掘削工程_測定年月日")] // 必須
    pub measurement_date: Option<String>,
    #[serde(rename = "掘削工程_掘進深度")] // 必須
    pub drilling_depth: Option<String>,
    #[serde(default, rename = "掘削工程_ケーシング下端深度")] // オプション
    pub casing_bottom_depth: Option<String>,
}

// ============================================================================
// 孔径孔壁保護
// DTD: <!ELEMENT 孔径孔壁保護 (孔径孔壁保護_下端深度, 孔径孔壁保護_孔径,
//      孔径孔壁保護_孔壁保護コード?, 孔径孔壁保護_孔壁保護方法?, 孔径孔壁保護_孔壁保護実施理由?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BoreholeDiameterProtection210 {
    #[serde(rename = "孔径孔壁保護_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "孔径孔壁保護_孔径")] // 必須
    pub diameter: Option<String>,
    #[serde(default, rename = "孔径孔壁保護_孔壁保護コード")] // オプション
    pub wall_protection_code: Option<String>,
    #[serde(default, rename = "孔径孔壁保護_孔壁保護方法")] // オプション
    pub wall_protection_method: Option<String>,
    #[serde(default, rename = "孔径孔壁保護_孔壁保護実施理由")] // オプション
    pub wall_protection_reason: Option<String>,
}

// ============================================================================
// 掘進速度
// DTD: <!ELEMENT 掘進速度 (掘進速度_下端深度, 掘進速度_掘進速度)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillingSpeed210 {
    #[serde(rename = "掘進速度_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "掘進速度_掘進速度")] // 必須
    pub speed: Option<String>,
}

// ============================================================================
// コアチューブビット
// DTD: <!ELEMENT コアチューブビット (コアチューブビット_下端深度,
//      コアチューブビット_コアチューブ名?, コアチューブビット_ビット名?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreTubeBit210 {
    #[serde(rename = "コアチューブビット_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(default, rename = "コアチューブビット_コアチューブ名")] // オプション
    pub core_tube_name: Option<String>,
    #[serde(default, rename = "コアチューブビット_ビット名")] // オプション
    pub bit_name: Option<String>,
}

// ============================================================================
// 給圧条件
// DTD: <!ELEMENT 給圧条件 (給圧条件_下端深度, 給圧条件_給圧)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct PressureCondition210 {
    #[serde(rename = "給圧条件_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "給圧条件_給圧")] // 必須
    pub pressure: Option<String>,
}

// ============================================================================
// 回転数
// DTD: <!ELEMENT 回転数 (回転数_下端深度, 回転数_回転数)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RotationSpeed210 {
    #[serde(rename = "回転数_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "回転数_回転数")] // 必須
    pub rotation_speed: Option<String>,
}

// ============================================================================
// 送水条件
// DTD: <!ELEMENT 送水条件 (送水条件_下端深度, 送水条件_送水圧, 送水条件_送水量,
//      送水条件_排水量, 送水条件_送水種類コード, 送水条件_送水種類?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterSupplyCondition210 {
    #[serde(rename = "送水条件_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "送水条件_送水圧")] // 必須
    pub water_pressure: Option<String>,
    #[serde(rename = "送水条件_送水量")] // 必須
    pub water_supply_volume: Option<String>,
    #[serde(rename = "送水条件_排水量")] // 必須
    pub drainage_volume: Option<String>,
    #[serde(rename = "送水条件_送水種類コード")] // 必須
    pub water_type_code: Option<String>,
    #[serde(default, rename = "送水条件_送水種類")] // オプション
    pub water_type: Option<String>,
}

// ============================================================================
// 断層破砕帯区分
// DTD: <!ELEMENT 断層破砕帯区分 (断層破砕帯区分_上端深度, 断層破砕帯区分_下端深度,
//      断層破砕帯区分_性状コード, 断層破砕帯区分_性状?, 断層破砕帯区分_備考?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FaultFractureZoneClassification210 {
    #[serde(rename = "断層破砕帯区分_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "断層破砕帯区分_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "断層破砕帯区分_性状コード")] // 必須
    pub property_code: Option<String>,
    #[serde(default, rename = "断層破砕帯区分_性状")] // オプション
    pub property: Option<String>,
    #[serde(default, rename = "断層破砕帯区分_備考")] // オプション
    pub remarks: Option<String>,
}

// ============================================================================
// コア採取率
// DTD: <!ELEMENT コア採取率 (コア採取率_下端深度, コア採取率_採取率)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreRecoveryRate210 {
    #[serde(rename = "コア採取率_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "コア採取率_採取率")] // 必須
    pub recovery_rate: Option<String>,
}

// ============================================================================
// 最大コア長
// DTD: <!ELEMENT 最大コア長 (最大コア長_下端深度, 最大コア長_コア長)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct MaximumCoreLength210 {
    #[serde(rename = "最大コア長_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "最大コア長_コア長")] // 必須
    pub core_length: Option<String>,
}

// ============================================================================
// RQD
// DTD: <!ELEMENT RQD (RQD_下端深度, RQD_RQD?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RQD210 {
    #[serde(rename = "RQD_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(default, rename = "RQD_RQD")] // オプション
    pub rqd: Option<String>,
}

// ============================================================================
// 岩級区分判定表
// DTD: <!ELEMENT 岩級区分判定表 (岩級区分判定表_項目名*, 岩級区分判定表_判定*)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RockClassClassificationTable210 {
    #[serde(default, rename = "岩級区分判定表_項目名")] // * (0回以上)
    pub item_names: Vec<String>,
    #[serde(default, rename = "岩級区分判定表_判定")] // * (0回以上)
    pub judgments: Vec<RockClassJudgment210>,
}

// ============================================================================
// 岩級区分判定表_判定
// DTD: <!ELEMENT 岩級区分判定表_判定 (岩級区分判定表_コード, 岩級区分判定表_記号, 岩級区分判定表_説明*)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RockClassJudgment210 {
    #[serde(rename = "岩級区分判定表_コード")] // 必須
    pub code: Option<String>,
    #[serde(rename = "岩級区分判定表_記号")] // 必須
    pub symbol: Option<String>,
    #[serde(default, rename = "岩級区分判定表_説明")] // * (0回以上)
    pub descriptions: Vec<String>,
}

// ============================================================================
// 岩級区分
// DTD: <!ELEMENT 岩級区分 (岩級区分_下端深度, 岩級区分_岩級区分)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RockClassClassification210 {
    #[serde(rename = "岩級区分_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "岩級区分_岩級区分")] // 必須
    pub classification: Option<String>,
}

// ============================================================================
// 保孔管
// DTD: <!ELEMENT 保孔管 (保孔管_下端深度, 保孔管_種別コード, 保孔管_備考?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CasingPipe210 {
    #[serde(rename = "保孔管_下端深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "保孔管_種別コード")] // 必須
    pub type_code: Option<String>,
    #[serde(default, rename = "保孔管_備考")] // オプション
    pub remarks: Option<String>,
}

// ============================================================================
// 計測機器
// DTD: <!ELEMENT 計測機器 (計測機器_上端深度, 計測機器_下端深度, 計測機器_機器種別, 計測機器_備考?)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasuringInstrument210 {
    #[serde(rename = "計測機器_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "計測機器_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "計測機器_機器種別")] // 必須
    pub instrument_type: Option<String>,
    #[serde(default, rename = "計測機器_備考")] // オプション
    pub remarks: Option<String>,
}

// ============================================================================
// 地下水検層試験
// DTD: <!ELEMENT 地下水検層試験 (地下水検層試験_試験番号, 地下水検層試験_上端深度,
//      地下水検層試験_下端深度, 地下水検層試験_掘削深度, 地下水検層試験_孔内水位,
//      地下水検層試験_試験方法コード, 地下水検層試験_電解質溶液濃度, 地下水検層試験_測定時間*)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundwaterLoggingTest210 {
    #[serde(rename = "地下水検層試験_試験番号")] // 必須
    pub test_number: Option<String>,
    #[serde(rename = "地下水検層試験_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "地下水検層試験_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "地下水検層試験_掘削深度")] // 必須
    pub drilling_depth: Option<String>,
    #[serde(rename = "地下水検層試験_孔内水位")] // 必須
    pub water_level: Option<String>,
    #[serde(rename = "地下水検層試験_試験方法コード")] // 必須
    pub test_method_code: Option<String>,
    #[serde(rename = "地下水検層試験_電解質溶液濃度")] // 必須
    pub electrolyte_concentration: Option<String>,
    #[serde(default, rename = "地下水検層試験_測定時間")] // * (0回以上)
    pub measurement_times: Vec<String>,
}

// ============================================================================
// 地下水検層試験詳細データ
// DTD: <!ELEMENT 地下水検層試験詳細データ (地下水検層試験詳細データ_試験番号,
//      地下水検層試験詳細データ_測定深度, 地下水検層試験詳細データ_比抵抗値*)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundwaterLoggingTestDetail210 {
    #[serde(rename = "地下水検層試験詳細データ_試験番号")] // 必須
    pub test_number: Option<String>,
    #[serde(rename = "地下水検層試験詳細データ_測定深度")] // 必須
    pub measurement_depth: Option<String>,
    #[serde(default, rename = "地下水検層試験詳細データ_比抵抗値")] // * (0回以上)
    pub resistivity_values: Vec<String>,
}

// ============================================================================
// 地下水検層試験判定結果
// DTD: <!ELEMENT 地下水検層試験判定結果 (地下水検層試験判定結果_上端深度,
//      地下水検層試験判定結果_下端深度, 地下水検層試験判定結果_地下水検層結果)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundwaterLoggingTestResult210 {
    #[serde(rename = "地下水検層試験判定結果_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "地下水検層試験判定結果_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "地下水検層試験判定結果_地下水検層結果")] // 必須
    pub result: Option<String>,
}

// ============================================================================
// 備考
// DTD: <!ELEMENT 備考 (備考_タイトル, 備考_上端深度, 備考_下端深度, 備考_備考内容)>
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Remark210 {
    #[serde(rename = "備考_タイトル")] // 必須
    pub title: Option<String>,
    #[serde(rename = "備考_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "備考_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "備考_備考内容")] // 必須
    pub content: Option<String>,
}
