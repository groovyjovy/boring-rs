use serde::{Deserialize, Serialize};
use std::str;

use crate::types::FreeInfo;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "ボーリング情報")]
pub struct Boring110 {
    #[serde(rename = "@DTD_version")]
    pub dtd_version: Option<String>,
    #[serde(rename = "標題情報")]
    pub title: Title,
    #[serde(rename = "コア情報")]
    pub core: Core,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "調査基本情報")]
    pub basic_info: BasicSurvey,
    #[serde(rename = "経度緯度情報")]
    pub longitude_latitude: LngLat,
    #[serde(default, rename = "ローカル座標")] // オプション
    pub local_coordinate_definition: Option<LocalCoordinateDefinition>,
    #[serde(rename = "調査位置")]
    pub survey_position: SurveyPosition,
    #[serde(rename = "発注機関")]
    pub order_institution: OrderInstitution,
    #[serde(rename = "調査期間")]
    pub survey_period: SurveyPeriod,
    #[serde(rename = "調査会社")]
    pub survey_company: SurveyCompany,
    #[serde(rename = "ボーリング基本情報")]
    pub boring_basic_info: BoringBasicInfo,
    #[serde(rename = "試錐機")]
    pub drilling_machine: DrillingMachine,
    #[serde(rename = "ハンマ落下用具")]
    pub hammer_drop_tool: HammerDropTool,
    #[serde(rename = "N値記録用具")]
    pub n_value_recorder: NValueRecorder,
    #[serde(rename = "エンジン")]
    pub engine: Engine,
    #[serde(rename = "ポンプ")]
    pub pump: Pump,
    #[serde(default, rename = "港湾局指定コード")] // オプション
    pub port_authority_code: Option<PortAuthorityCode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicSurvey {
    #[serde(rename = "事業工事名")] // 必須
    pub project_name: Option<String>,
    #[serde(rename = "調査名")] // 必須
    pub survey_name: Option<String>,
    #[serde(rename = "調査目的")] // 必須
    pub survey_purpose: Option<String>,
    #[serde(rename = "調査対象")] // 必須
    pub survey_target: Option<String>,
    #[serde(rename = "ボーリング名")] // 必須
    pub boring_name: Option<String>,
    #[serde(rename = "ボーリング連番")] // 必須
    pub boring_serial: Option<String>,
    #[serde(rename = "ボーリング総数")] // 必須
    pub total_boring: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LngLat {
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
    #[serde(rename = "読取精度コード")] // 必須
    pub reading_precision_code: Option<String>,
    #[serde(rename = "測地系")] // 必須
    pub geodetic_system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LocalCoordinateDefinition {
    #[serde(rename = "X座標定義")] // 必須
    pub x_definition: Option<String>,
    #[serde(rename = "X座標")] // 必須
    pub x: Option<String>,
    #[serde(rename = "Y座標定義")] // 必須
    pub y_definition: Option<String>,
    #[serde(rename = "Y座標")] // 必須
    pub y: Option<String>,
    #[serde(rename = "Z座標定義")] // 必須
    pub z_definition: Option<String>,
    #[serde(rename = "Z座標")] // 必須
    pub z: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyPosition {
    #[serde(rename = "調査位置住所")] // 必須
    pub address: Option<String>,
    #[serde(rename = "コード1次")] // 必須
    pub code1: Option<String>,
    #[serde(rename = "コード2次")] // 必須
    pub code2: Option<String>,
    #[serde(rename = "コード3次")] // 必須
    pub code3: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInstitution {
    #[serde(rename = "発注機関名称")] // 必須
    pub name: Option<String>,
    #[serde(rename = "テクリスコード")] // 必須
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyPeriod {
    #[serde(rename = "調査期間_開始年")] // 必須
    pub start_year: Option<String>,
    #[serde(rename = "調査期間_開始月")] // 必須
    pub start_month: Option<String>,
    #[serde(rename = "調査期間_開始日")] // 必須
    pub start_day: Option<String>,
    #[serde(rename = "調査期間_終了年")] // 必須
    pub end_year: Option<String>,
    #[serde(rename = "調査期間_終了月")] // 必須
    pub end_month: Option<String>,
    #[serde(rename = "調査期間_終了日")] // 必須
    pub end_day: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyCompany {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BoringBasicInfo {
    #[serde(rename = "孔口標高")] // 必須
    pub elevation: Option<String>,
    #[serde(rename = "総掘進長")] // 必須
    pub total_length: Option<String>,
    #[serde(rename = "地質柱状図様式")] // 必須
    pub columnar_section_style: Option<String>,
    #[serde(default, rename = "掘進角度")] // オプション
    pub drilling_angle: Option<String>,
    #[serde(default, rename = "掘進方位")] // オプション
    pub drilling_direction: Option<String>,
    #[serde(rename = "地盤勾配")] // 必須
    pub ground_slope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillingMachine {
    #[serde(rename = "試錐機_名称")]
    pub name: Option<String>,
    #[serde(rename = "試錐機_能力")]
    pub capacity: Option<String>,
    #[serde(rename = "試錐機_方法")]
    pub method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HammerDropTool {
    #[serde(rename = "ハンマ落下用具_コード")]
    pub code: Option<String>,
    #[serde(rename = "ハンマ落下用具_名称")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NValueRecorder {
    #[serde(rename = "N値記録用具_コード")]
    pub code: Option<String>,
    #[serde(rename = "N値記録用具_名称")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    #[serde(rename = "エンジン_名称")]
    pub name: Option<String>,
    #[serde(rename = "エンジン_能力")]
    pub capacity: Option<String>,
    #[serde(rename = "エンジン_単位")]
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pump {
    #[serde(rename = "ポンプ_名称")]
    pub name: Option<String>,
    #[serde(rename = "ポンプ_能力")]
    pub capacity: Option<String>,
    #[serde(rename = "ポンプ_単位")]
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortAuthorityCode {
    #[serde(rename = "櫓種類コード")]
    pub tower_type_code: Option<String>,
    #[serde(rename = "建設局")]
    pub construction_bureau: Option<String>,
    #[serde(rename = "都道府県")]
    pub prefecture: Option<String>,
    #[serde(rename = "港名")]
    pub port_name: Option<String>,
    #[serde(rename = "調査者")]
    pub investigator: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Core {
    #[serde(rename = "地質区分")]
    pub geological_classifications: Vec<GeologicalClassification>,
    #[serde(default, rename = "色調")]
    pub colors: Vec<Color>,
    #[serde(default, rename = "観察記事")]
    pub observational_articles: Vec<ObservationalArticle>,
    #[serde(default, rename = "観察記事枠線")]
    pub observational_article_frames: Vec<ObservationalArticleFrame>,
    #[serde(default, rename = "標準貫入試験")]
    pub standard_penetration_test: Vec<StandardPenetrationTest>,
    #[serde(default, rename = "相対密度稠度")]
    pub relative_density: Vec<RelativeDensity>,
    #[serde(default, rename = "孔内水平載荷試験")]
    pub borehole_horizontal_load_test: Vec<BoreholeHorizontalLoadTest>,
    #[serde(default, rename = "透水試験")]
    pub borehole_drilled_well_test: Vec<BoreholeDrilledWellTest>,
    #[serde(default, rename = "P波試験")]
    pub p_wave_test: Vec<PWaveTest>,
    #[serde(default, rename = "S波試験")]
    pub s_wave_test: Vec<SWaveTest>,
    #[serde(default, rename = "その他原位置試験")]
    pub other_original_position_tests: Vec<OtherOriginalPositionTests>,
    #[serde(default, rename = "採取試料")]
    pub core_samples: Vec<CoreSample>,
    #[serde(default, rename = "土質試験結果")]
    pub soil_test_result: Vec<SoilTestResult>,
    #[serde(default, rename = "地盤分類")]
    pub ground_classifications: Vec<GeotechnicalClassification>,
    #[serde(default, rename = "地質時代")]
    pub geological_eras: Vec<GeologicalEra>,
    #[serde(default, rename = "地層区分")]
    pub geostratum_classifications: Vec<GeostratumClassification>,
    #[serde(default, rename = "孔内水位")]
    pub borehole_water_levels: Vec<BoreholeWaterLevel>,
    #[serde(default, rename = "掘削工程")]
    pub drilling_process: Vec<DrillingProcess>,
    #[serde(default, rename = "孔径孔壁保護")]
    pub borehole_wall_protection: Vec<BoreholeWallProtection>,
    #[serde(default, rename = "断層区分")]
    pub fault_classification: Vec<FaultClassification>,
    #[serde(default, rename = "フリー情報")]
    pub free_info: Vec<FreeInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeologicalClassification {
    #[serde(rename = "地質区分_深度")]
    pub depth: Option<String>,
    #[serde(rename = "地質区分_地質名称1")]
    pub classification1: Option<String>,
    #[serde(rename = "地質区分_地質コード1")]
    pub code1: Option<String>,
    #[serde(rename = "地質区分_地質名称2")]
    pub classification2: Option<String>,
    #[serde(rename = "地質区分_地質コード2")]
    pub code2: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    #[serde(rename = "色調_下端深度")]
    pub depth: Option<String>,
    #[serde(rename = "色調_状態")]
    pub state: Option<String>,
    #[serde(rename = "色調_コード")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationalArticle {
    #[serde(rename = "観察記事_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "観察記事_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "観察記事_記事")]
    pub observation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationalArticleFrame {
    #[serde(rename = "観察記事枠線_下端深度")]
    pub end_depth: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardPenetrationTest {
    #[serde(default, rename = "標準貫入試験_開始深度")]
    pub start_depth: Option<String>,
    #[serde(default, rename = "標準貫入試験_0_10打撃回数")]
    pub hits_0_10: Option<String>,
    #[serde(default, rename = "標準貫入試験_0_10貫入量")]
    pub penetration_0_10: Option<String>,
    #[serde(default, rename = "標準貫入試験_10_20打撃回数")]
    pub hits_10_20: Option<String>,
    #[serde(default, rename = "標準貫入試験_10_20貫入量")]
    pub penetration_10_20: Option<String>,
    #[serde(default, rename = "標準貫入試験_20_30打撃回数")]
    pub hits_20_30: Option<String>,
    #[serde(default, rename = "標準貫入試験_20_30貫入量")]
    pub penetration_20_30: Option<String>,
    #[serde(default, rename = "標準貫入試験_合計打撃回数")]
    pub total_hits: Option<String>,
    #[serde(default, rename = "標準貫入試験_合計貫入量")]
    pub total_penetration: Option<String>,
    #[serde(default, rename = "標準貫入試験_備考")]
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeDensity {
    #[serde(rename = "相対密度稠度_下端深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "相対密度_コード")]
    pub code: Option<String>,
    #[serde(default, rename = "相対密度_状態")]
    pub state: Option<String>,
    #[serde(default, rename = "相対稠度_コード")]
    pub relative_compaction_code: Option<String>,
    #[serde(default, rename = "相対稠度_状態")]
    pub relative_compaction_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoreholeHorizontalLoadTest {
    #[serde(rename = "孔内水平載荷試験_試験深度")] // 必須
    pub depth: Option<String>,
    #[serde(rename = "孔内水平載荷試験_試験方法コード")] // 必須
    pub method_code: Option<String>,
    #[serde(default, rename = "孔内水平載荷試験_試験方法")] // オプション
    pub method: Option<String>,
    #[serde(rename = "孔内水平載荷試験_初期圧")] // 必須
    pub initial_pressure: Option<String>,
    #[serde(rename = "孔内水平載荷試験_降伏圧")] // 必須
    pub yield_pressure: Option<String>,
    #[serde(rename = "孔内水平載荷試験_変形係数")] // 必須
    pub deformation_coefficient: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoreholeDrilledWellTest {
    #[serde(rename = "透水試験_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "透水試験_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(rename = "透水試験_試験コード")] // 必須
    pub test_code: Option<String>,
    #[serde(default, rename = "透水試験_試験方法")] // オプション
    pub test_method: Option<String>,
    #[serde(rename = "透水試験_透水係数")] // 必須
    pub permeability: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PWaveTest {
    #[serde(rename = "P波試験_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "P波試験_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "P波試験_速度")]
    pub velocity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SWaveTest {
    #[serde(rename = "S波試験_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "S波試験_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "S波試験_速度")]
    pub velocity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherOriginalPositionTests {
    #[serde(default, rename = "その他原位置試験_試験方法")]
    pub test_method: Option<String>,
    #[serde(default, rename = "その他原位置試験_上端深度")]
    pub start_depth: Option<String>,
    #[serde(default, rename = "その他原位置試験_下端深度")]
    pub end_depth: Option<String>,
    #[serde(default, rename = "その他原位置試験_試験結果")]
    pub test_result: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CoreSample {
    #[serde(default, rename = "採取試料_上端深度")]
    pub start_depth: Option<String>,
    #[serde(default, rename = "採取試料_下端深度")]
    pub end_depth: Option<String>,
    #[serde(default, rename = "採取試料_試料番号")]
    pub sample_number: Option<String>,
    #[serde(default, rename = "採取試料_採取方法コード")]
    pub collection_method_code: Option<String>,
    #[serde(default, rename = "採取試料_採取方法")]
    pub method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoilTestResult {
    #[serde(rename = "土質試験結果_試料番号")] // 必須
    pub sample_number: Option<String>,
    #[serde(rename = "土質試験結果_上端深度")] // 必須
    pub start_depth: Option<String>,
    #[serde(rename = "土質試験結果_下端深度")] // 必須
    pub end_depth: Option<String>,
    #[serde(default, rename = "土質試験結果_湿潤密度")] // オプション
    pub wet_density: Option<String>,
    #[serde(default, rename = "土質試験結果_乾燥密度")] // オプション
    pub dry_density: Option<String>,
    #[serde(default, rename = "土質試験結果_土粒子密度")] // オプション
    pub soil_particle_density: Option<String>,
    #[serde(default, rename = "土質試験結果_自然含水比")] // オプション
    pub natural_water_content: Option<String>,
    #[serde(default, rename = "土質試験結果_間隙比")] // オプション
    pub void_ratio: Option<String>,
    #[serde(default, rename = "土質試験結果_飽和度")] // オプション
    pub saturation: Option<String>,
    #[serde(default, rename = "土質試験結果_粒度_石")] // オプション
    pub stone: Option<String>,
    #[serde(default, rename = "土質試験結果_粒度_礫")] // オプション
    pub gravel: Option<String>,
    #[serde(default, rename = "土質試験結果_粒度_砂")] // オプション
    pub sand: Option<String>,
    #[serde(default, rename = "土質試験結果_粒度_シルト")] // オプション
    pub silt: Option<String>,
    #[serde(default, rename = "土質試験結果_粒度_粘土")] // オプション
    pub clay: Option<String>,
    #[serde(default, rename = "土質試験結果_粒度_最大粒径")] // オプション
    pub max_grain_size: Option<String>,
    #[serde(default, rename = "土質試験結果_粒度_均等係数")] // オプション
    pub equal_coefficient: Option<String>,
    #[serde(default, rename = "土質試験結果_液性限界")] // オプション
    pub liquidity_limit: Option<String>,
    #[serde(default, rename = "土質試験結果_塑性限界")] // オプション
    pub plasticity_limit: Option<String>,
    #[serde(default, rename = "土質試験結果_塑性指数")] // オプション
    pub plasticity_index: Option<String>,
    #[serde(default, rename = "土質試験結果_地盤材料名")] // オプション
    pub soil_name: Option<String>,
    #[serde(default, rename = "土質試験結果_地盤材料記号")] // オプション
    pub soil_symbol: Option<String>,
    #[serde(default, rename = "土質試験結果_圧密試験方法")] // オプション
    pub compression_test_method: Option<String>,
    #[serde(default, rename = "土質試験結果_圧縮指数")] // オプション
    pub compression_index: Option<String>,
    #[serde(default, rename = "土質試験結果_圧密降伏応力")] // オプション
    pub compression_yield_stress: Option<String>,
    #[serde(default, rename = "土質試験結果_体積圧縮係数")] // オプション
    pub volume_compression_coefficient: Option<String>,
    #[serde(default, rename = "土質試験結果_圧密係数")] // オプション
    pub compression_coefficient: Option<String>,
    #[serde(default, rename = "土質試験結果_応力範囲")] // オプション
    pub stress_range: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸圧縮強さ1")] // オプション
    pub compression_strength_1: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸圧縮強さ2")] // オプション
    pub compression_strength_2: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸圧縮強さ3")] // オプション
    pub compression_strength_3: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸圧縮強さ4")] // オプション
    pub compression_strength_4: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸試験破壊ひずみ1")] // オプション
    pub compression_failure_1: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸試験破壊ひずみ2")] // オプション
    pub compression_failure_2: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸試験破壊ひずみ3")] // オプション
    pub compression_failure_3: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸試験破壊ひずみ4")] // オプション
    pub compression_failure_4: Option<String>,
    #[serde(default, rename = "土質試験結果_一軸試験鋭敏比")] // オプション
    pub compression_sharpness_ratio: Option<String>,
    #[serde(default, rename = "土質試験結果_せん断試験条件")] // オプション
    pub tensile_test_condition: Option<String>,
    #[serde(default, rename = "土質試験結果_せん断強さ_全応力")] // オプション
    pub tensile_strength_total: Option<String>,
    #[serde(default, rename = "土質試験結果_せん断抵抗角_全応力")] // オプション
    pub tensile_resistance_angle_total: Option<String>,
    #[serde(default, rename = "土質試験結果_せん断強さ_有効応力")] // オプション
    pub tensile_strength_effective: Option<String>,
    #[serde(default, rename = "土質試験結果_せん断抵抗角_有効応力")] // オプション
    pub tensile_resistance_angle_effective: Option<String>,
    #[serde(default, rename = "土質試験結果_試料状態")] // オプション
    pub sample_condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeotechnicalClassification {
    #[serde(rename = "地盤分類_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "地盤分類_工学的分類記号")]
    pub classification_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeologicalEra {
    #[serde(rename = "地質時代_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "地質時代_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "地質時代_コード")]
    pub code: Option<String>,
    #[serde(rename = "地質時代_時代名")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeostratumClassification {
    #[serde(rename = "地層区分_上端深度")]
    pub start_depth: Option<String>,
    #[serde(rename = "地層区分_下端深度")]
    pub end_depth: Option<String>,
    #[serde(rename = "地層区分_地層名")]
    pub rock_body_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BoreholeWaterLevel {
    #[serde(default, rename = "孔内水位_測定年")]
    pub year: Option<String>,
    #[serde(default, rename = "孔内水位_測定月")]
    pub month: Option<String>,
    #[serde(default, rename = "孔内水位_測定日")]
    pub day: Option<String>,
    #[serde(default, rename = "孔内水位_掘削深度")]
    pub depth: Option<String>,
    #[serde(default, rename = "孔内水位_孔内水位")]
    pub water_level: Option<String>,
    #[serde(default, rename = "孔内水位_水位種別")]
    pub water_level_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillingProcess {
    #[serde(default, rename = "掘削工程_測定年")]
    pub measurement_year: Option<String>,
    #[serde(default, rename = "掘削工程_測定月")]
    pub measurement_month: Option<String>,
    #[serde(default, rename = "掘削工程_測定日")]
    pub measurement_day: Option<String>,
    #[serde(default, rename = "掘削工程_掘進深度")]
    pub drilling_depth: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoreholeWallProtection {
    #[serde(default, rename = "孔径孔壁保護_上端深度")]
    pub upper_depth: Option<String>,
    #[serde(default, rename = "孔径孔壁保護_下端深度")]
    pub lower_depth: Option<String>,
    #[serde(default, rename = "孔径孔壁保護_孔径")]
    pub diameter: Option<String>,
    #[serde(default, rename = "孔径孔壁保護_孔壁保護")]
    pub wall_protection: Option<String>,
    #[serde(default, rename = "孔径孔壁保護_送水条件")]
    pub water_supply_condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaultClassification {
    #[serde(default, rename = "断層区分_上端深度")]
    pub upper_depth: Option<String>,
    #[serde(default, rename = "断層区分_下端深度")]
    pub lower_depth: Option<String>,
    #[serde(default, rename = "断層区分_性状")]
    pub property: Option<String>,
}
