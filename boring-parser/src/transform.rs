//! 測地系変換モジュール
//!
//! 日本測地系(Tokyo Datum)およびJGD2000からJGD2011への座標変換を提供する。
//! 国土地理院のパラメータファイル(TKY2JGD, PatchJGD)を使用。
//!
//! # 地震補正の適用ルール
//! 地震による地殻変動の補正は、**調査日が地震発生日より前**の場合のみ適用される。
//! 調査日が地震後の場合、測量時点で既に地殻変動後の位置が記録されているため、
//! 補正を適用すると誤った位置になる。

use encoding_rs::SHIFT_JIS;
use jgdtrans::{Format, ParData, Parameter, Point, Transformer};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// 変換エラー
#[derive(Error, Debug)]
pub enum TransformError {
    #[error("パラメータファイル読み込みエラー: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("パラメータパースエラー: {0}")]
    Parse(String),

    #[error("変換エラー: {0}")]
    Transform(String),

    #[error("パラメータディレクトリが存在しません: {0}")]
    DirectoryNotFound(String),
}

/// 調査開始日（年月日）
///
/// 地震補正の適用判定に使用。調査開始日が地震発生日より前の場合のみ補正を適用する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurveyStartDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl SurveyStartDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// YYYYMMDD形式の文字列からパース
    pub fn from_yyyymmdd(s: &str) -> Option<Self> {
        if s.len() < 8 {
            return None;
        }
        let year = s[0..4].parse().ok()?;
        let month = s[4..6].parse().ok()?;
        let day = s[6..8].parse().ok()?;
        Some(Self { year, month, day })
    }

    /// 年月日を個別にパース
    pub fn from_parts(year: Option<&str>, month: Option<&str>, day: Option<&str>) -> Option<Self> {
        Some(Self {
            year: year?.parse().ok()?,
            month: month?.parse().ok()?,
            day: day?.parse().ok()?,
        })
    }
}

/// 地震情報
struct EarthquakeInfo {
    /// ファイル名に含まれるパターン
    file_pattern: &'static str,
    /// 地震発生日
    date: SurveyStartDate,
}

/// 地震補正パラメータの一覧（発生日順）
/// 調査日がこの日付より前の場合のみ、対応するパラメータを適用する
const EARTHQUAKES: &[EarthquakeInfo] = &[
    EarthquakeInfo {
        file_pattern: "tokachi2003",
        date: SurveyStartDate { year: 2003, month: 9, day: 26 },
    },
    EarthquakeInfo {
        file_pattern: "fukuoka2005",
        date: SurveyStartDate { year: 2005, month: 3, day: 20 },
    },
    EarthquakeInfo {
        file_pattern: "noto2007",
        date: SurveyStartDate { year: 2007, month: 3, day: 25 },
    },
    EarthquakeInfo {
        file_pattern: "chuetsuoki2007",
        date: SurveyStartDate { year: 2007, month: 7, day: 16 },
    },
    EarthquakeInfo {
        file_pattern: "iwatemiyagi2008",
        date: SurveyStartDate { year: 2008, month: 6, day: 14 },
    },
    EarthquakeInfo {
        file_pattern: "touhokutaiheiyouoki2011",
        date: SurveyStartDate { year: 2011, month: 3, day: 11 },
    },
    EarthquakeInfo {
        file_pattern: "kumamoto2016",
        date: SurveyStartDate { year: 2016, month: 4, day: 14 },
    },
];

/// ファイル名が地震補正パラメータかどうかを判定し、適用すべきかを返す
fn should_apply_earthquake_correction(file_name: &str, survey_date: Option<SurveyStartDate>) -> bool {
    // 地震補正ファイルでない場合は常に適用（ジオイド補正など）
    let earthquake = EARTHQUAKES.iter().find(|eq| file_name.contains(eq.file_pattern));

    match (earthquake, survey_date) {
        // 地震ファイルで、調査日が指定されている場合
        (Some(eq), Some(date)) => {
            // 調査日が地震発生日より前の場合のみ適用
            date < eq.date
        }
        // 地震ファイルだが調査日が不明 → 安全のため適用しない
        (Some(_), None) => false,
        // 地震ファイルでない（ジオイド等）→ 常に適用
        (None, _) => true,
    }
}

/// パラメータファイルを読み込む（Shift_JIS/UTF-8自動判定）
pub fn read_param_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let bytes = fs::read(&path)?;

    // まずUTF-8として試す
    if let Ok(s) = String::from_utf8(bytes.clone()) {
        return Ok(s);
    }

    // UTF-8でなければShift_JISとして変換
    let (decoded, _, _) = SHIFT_JIS.decode(&bytes);
    Ok(decoded.into_owned())
}

/// JGD2011座標変換器
///
/// Tokyo Datum/JGD2000からJGD2011への変換を行う。
/// - Step 1: Tokyo Datum → JGD2000 (TKY2JGD)
/// - Step 2: JGD2000 → JGD2011 (PatchJGD統合)
pub struct JgdTransformer {
    /// Tokyo Datum → JGD2000 変換器
    tky_tf: Transformer<ParData>,
    /// JGD2000 → JGD2011 統合変換器 (水平+垂直)
    unified_tf: Transformer<ParData>,
}

impl JgdTransformer {
    /// パラメータファイルを読み込んで変換器を初期化（調査日指定あり）
    ///
    /// # Arguments
    /// * `tky_path` - TKY2JGD.par のパス
    /// * `patch_dir` - PatchJGD系ファイルが入ったディレクトリのパス
    /// * `survey_date` - 調査日（地震補正の適用判定に使用）
    ///
    /// # 地震補正の適用ルール
    /// - 調査日が地震発生日より**前**の場合のみ、その地震の補正を適用
    /// - 調査日が地震発生日より**後**の場合、補正は適用しない（既に変動後の位置）
    /// - ジオイド補正（hyokorev2014_geoid2011_h.par）は常に適用
    ///
    /// # Example
    /// ```ignore
    /// let survey_date = SurveyStartDate::new(2010, 5, 15);
    /// let transformer = JgdTransformer::with_survey_date(
    ///     "params/TKY2JGD.par",
    ///     "params/patch_files",
    ///     survey_date
    /// )?;
    /// ```
    pub fn with_survey_date<P: AsRef<Path>>(
        tky_path: P,
        patch_dir: P,
        survey_date: SurveyStartDate,
    ) -> Result<Self, TransformError> {
        let tky_content = read_param_file(&tky_path)?;
        let tky_tf = Transformer::from_str(&tky_content, Format::TKY2JGD)
            .map_err(|e| TransformError::Parse(format!("TKY2JGD parse error: {}", e)))?;

        let unified_tf = Self::create_unified_transformer(patch_dir, Some(survey_date))?;

        Ok(Self { tky_tf, unified_tf })
    }

    /// パラメータファイルを読み込んで変換器を初期化（調査日なし）
    ///
    /// **注意**: 調査日が不明な場合、地震補正は適用されません。
    /// ジオイド補正のみが適用されます。
    ///
    /// # Arguments
    /// * `tky_path` - TKY2JGD.par のパス
    /// * `patch_dir` - PatchJGD系ファイルが入ったディレクトリのパス
    pub fn new<P: AsRef<Path>>(tky_path: P, patch_dir: P) -> Result<Self, TransformError> {
        let tky_content = read_param_file(&tky_path)?;
        let tky_tf = Transformer::from_str(&tky_content, Format::TKY2JGD)
            .map_err(|e| TransformError::Parse(format!("TKY2JGD parse error: {}", e)))?;

        // 調査日なしの場合、地震補正は適用しない（ジオイド補正のみ）
        let unified_tf = Self::create_unified_transformer(patch_dir, None)?;

        Ok(Self { tky_tf, unified_tf })
    }

    /// TKY2JGDのみで変換器を初期化 (PatchJGDファイルなし)
    ///
    /// JGD2000→JGD2011変換が不要な場合に使用
    pub fn new_tky_only<P: AsRef<Path>>(tky_path: P) -> Result<Self, TransformError> {
        let tky_content = read_param_file(&tky_path)?;
        let tky_tf = Transformer::from_str(&tky_content, Format::TKY2JGD)
            .map_err(|e| TransformError::Parse(format!("TKY2JGD parse error: {}", e)))?;

        // 空の統合変換器を作成
        let empty_params: HashMap<u32, Parameter> = HashMap::new();
        let empty_data =
            ParData::with_description(Format::PatchJGD_HV, empty_params, "Empty".to_string());
        let unified_tf = Transformer::new(empty_data);

        Ok(Self { tky_tf, unified_tf })
    }

    /// ディレクトリ内のパラメータファイルを加算マージして1つのTransformerを作る
    ///
    /// # Directory Structure
    /// ```text
    /// patch_dir/
    /// ├── horizontal/    # 水平補正ファイル (PatchJGD形式)
    /// │   ├── touhokutaiheiyouoki2011.par
    /// │   └── ...
    /// └── elevation/     # 標高補正ファイル (PatchJGD_H/HyokoRev形式)
    ///     ├── touhokutaiheiyouoki2011_h.par
    ///     ├── hyokorev2014_geoid2011_h.par
    ///     └── ...
    /// ```
    ///
    /// # Arguments
    /// * `dir_path` - パラメータファイルのルートディレクトリ
    /// * `survey_date` - 調査日（Noneの場合、地震補正は適用しない）
    fn create_unified_transformer<P: AsRef<Path>>(
        dir_path: P,
        survey_date: Option<SurveyStartDate>,
    ) -> Result<Transformer<ParData>, TransformError> {
        let dir = dir_path.as_ref();

        // ディレクトリが存在しない場合は空のTransformerを返す
        if !dir.exists() {
            let empty_params: HashMap<u32, Parameter> = HashMap::new();
            let empty_data = ParData::with_description(
                Format::PatchJGD_HV,
                empty_params,
                "No patch files".to_string(),
            );
            return Ok(Transformer::new(empty_data));
        }

        let mut unified_params: HashMap<u32, Parameter> = HashMap::new();
        let mut description = String::from("Unified Parameter Set:\n");
        if let Some(date) = survey_date {
            description.push_str(&format!(
                "Survey date: {}/{}/{}\n",
                date.year, date.month, date.day
            ));
        } else {
            description.push_str("Survey date: unknown (earthquake corrections disabled)\n");
        }
        let mut loaded_count = 0;

        // horizontal/ と elevation/ サブディレクトリをスキャン
        let horizontal_dir = dir.join("horizontal");
        let elevation_dir = dir.join("elevation");

        // 水平補正ファイルを読み込み
        if horizontal_dir.exists() {
            description.push_str("\n[Horizontal corrections]\n");
            loaded_count += Self::load_param_files(
                &horizontal_dir,
                Format::PatchJGD,
                survey_date,
                &mut unified_params,
                &mut description,
            )?;
        }

        // 標高補正ファイルを読み込み
        if elevation_dir.exists() {
            description.push_str("\n[Elevation corrections]\n");
            loaded_count += Self::load_param_files_elevation(
                &elevation_dir,
                survey_date,
                &mut unified_params,
                &mut description,
            )?;
        }

        if loaded_count == 0 {
            description = "No patch files loaded".to_string();
        }

        let unified_data =
            ParData::with_description(Format::PatchJGD_HV, unified_params, description);
        Ok(Transformer::new(unified_data))
    }

    /// 指定ディレクトリからパラメータファイルを読み込む（水平用）
    fn load_param_files(
        dir: &Path,
        format: Format,
        survey_date: Option<SurveyStartDate>,
        unified_params: &mut HashMap<u32, Parameter>,
        description: &mut String,
    ) -> Result<usize, TransformError> {
        let mut loaded_count = 0;

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("par") {
                continue;
            }

            let file_name = match path.file_name().and_then(|s| s.to_str()) {
                Some(name) => name.to_string(),
                None => continue,
            };

            // 地震補正の適用判定
            if !should_apply_earthquake_correction(&file_name, survey_date) {
                description.push_str(&format!("  - {} (skipped: survey after earthquake)\n", file_name));
                continue;
            }

            let content = read_param_file(&path)?;
            let par_data = ParData::from_str(&content, format)
                .map_err(|e| TransformError::Parse(format!("{}: {}", file_name, e)))?;

            for (mesh_code, param) in par_data.parameter {
                unified_params
                    .entry(mesh_code)
                    .and_modify(|existing| {
                        existing.latitude += param.latitude;
                        existing.longitude += param.longitude;
                        existing.altitude += param.altitude;
                    })
                    .or_insert(param);
            }

            description.push_str(&format!("  - {} (applied)\n", file_name));
            loaded_count += 1;
        }

        Ok(loaded_count)
    }

    /// 指定ディレクトリからパラメータファイルを読み込む（標高用）
    fn load_param_files_elevation(
        dir: &Path,
        survey_date: Option<SurveyStartDate>,
        unified_params: &mut HashMap<u32, Parameter>,
        description: &mut String,
    ) -> Result<usize, TransformError> {
        let mut loaded_count = 0;

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("par") {
                continue;
            }

            let file_name = match path.file_name().and_then(|s| s.to_str()) {
                Some(name) => name.to_string(),
                None => continue,
            };

            // 地震補正の適用判定
            if !should_apply_earthquake_correction(&file_name, survey_date) {
                description.push_str(&format!("  - {} (skipped: survey after earthquake)\n", file_name));
                continue;
            }

            // ファイル名でフォーマットを判別
            let format = if file_name.contains("hyoko") {
                Format::HyokoRev // ジオイドモデル改定
            } else {
                Format::PatchJGD_H // 地震（標高）
            };

            let content = read_param_file(&path)?;
            let par_data = ParData::from_str(&content, format)
                .map_err(|e| TransformError::Parse(format!("{}: {}", file_name, e)))?;

            for (mesh_code, param) in par_data.parameter {
                unified_params
                    .entry(mesh_code)
                    .and_modify(|existing| {
                        existing.latitude += param.latitude;
                        existing.longitude += param.longitude;
                        existing.altitude += param.altitude;
                    })
                    .or_insert(param);
            }

            description.push_str(&format!("  - {} (applied)\n", file_name));
            loaded_count += 1;
        }

        Ok(loaded_count)
    }

    /// 座標をJGD2011に変換 (水平座標のみ)
    ///
    /// # Arguments
    /// * `lat` - 緯度 (度, 10進数)
    /// * `lon` - 経度 (度, 10進数)
    /// * `system_code` - 測地系コード ("00"=Tokyo, "01"=JGD2000, "02"=JGD2011)
    ///
    /// # Returns
    /// 変換後の (緯度, 経度) または変換失敗時にNone
    pub fn transform_horizontal(
        &self,
        lat: f64,
        lon: f64,
        system_code: &str,
    ) -> Option<(f64, f64)> {
        let result = self.transform_point(lat, lon, 0.0, system_code).ok()?;
        Some((result.latitude, result.longitude))
    }

    /// 座標をJGD2011に変換 (水平+垂直)
    ///
    /// # Arguments
    /// * `lat` - 緯度 (度, 10進数)
    /// * `lon` - 経度 (度, 10進数)
    /// * `alt` - 標高 (m)
    /// * `system_code` - 測地系コード ("00"=Tokyo, "01"=JGD2000, "02"=JGD2011)
    ///
    /// # Returns
    /// 変換後の (緯度, 経度, 標高)
    pub fn transform_full(
        &self,
        lat: f64,
        lon: f64,
        alt: f64,
        system_code: &str,
    ) -> Result<(f64, f64, f64), TransformError> {
        let result = self.transform_point(lat, lon, alt, system_code)?;
        Ok((result.latitude, result.longitude, result.altitude))
    }

    /// 内部変換処理
    fn transform_point(
        &self,
        lat: f64,
        lon: f64,
        alt: f64,
        system_code: &str,
    ) -> Result<Point, TransformError> {
        let input_point = Point::new_unchecked(lat, lon, alt);

        // コードの正規化 (先頭のゼロ除去)
        let code = system_code.trim().trim_start_matches('0');

        match code {
            // Tokyo Datum (日本測地系) → Step 1 + Step 2
            // code "00" の場合、trim_start_matches('0') で "" になる
            "" | "0" => {
                // Step 1: Tokyo → JGD2000
                let corr_tky = self
                    .tky_tf
                    .forward_corr(&input_point)
                    .map_err(|e| TransformError::Transform(format!("TKY2JGD: {}", e)))?;
                let p_2000 = &input_point + corr_tky;

                // Step 2: JGD2000 → JGD2011
                match self.unified_tf.forward_corr(&p_2000) {
                    Ok(corr_unified) => Ok(&p_2000 + corr_unified),
                    Err(_) => Ok(p_2000), // パッチ範囲外の場合はJGD2000のまま
                }
            }

            // JGD2000 (世界測地系2000) → Step 2のみ
            "1" => match self.unified_tf.forward_corr(&input_point) {
                Ok(corr_unified) => Ok(&input_point + corr_unified),
                Err(_) => Ok(input_point), // パッチ範囲外の場合はそのまま
            },

            // JGD2011 → 変換不要
            "2" => Ok(input_point),

            // 未知のコード → そのまま返す
            _ => Ok(input_point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_normalization() {
        // "00" → "" (Tokyo Datum)
        let code = "00".trim().trim_start_matches('0');
        assert_eq!(code, "");

        // "01" → "1" (JGD2000)
        let code = "01".trim().trim_start_matches('0');
        assert_eq!(code, "1");

        // "02" → "2" (JGD2011)
        let code = "02".trim().trim_start_matches('0');
        assert_eq!(code, "2");
    }

    #[test]
    fn test_tky2jgd_transform() {
        // TKY2JGD.par が存在する場合のみテスト実行
        // ワークスペースルートからのパスを構築
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let tky_path = workspace_root.join("params/TKY2JGD.par");
        if !tky_path.exists() {
            println!("Skipping test: TKY2JGD.par not found at {:?}", tky_path);
            return;
        }

        let transformer = JgdTransformer::new_tky_only(tky_path)
            .expect("Failed to create transformer");

        // 東京駅付近 (Tokyo Datum)
        // 日本測地系: 35°40'00", 139°46'00"
        let (lat, lon) = transformer
            .transform_horizontal(35.666667, 139.766667, "00")
            .expect("Transform failed");

        // Tokyo→JGD2000では約10秒(約300m)のずれがある
        // JGD2000座標は元より北西にずれる
        assert!((lat - 35.666667).abs() < 0.01, "Latitude shift too large");
        assert!((lon - 139.766667).abs() < 0.01, "Longitude shift too large");

        // 変換後は元と異なる値になるはず
        assert!((lat - 35.666667).abs() > 0.0001, "Latitude should have changed");
        assert!((lon - 139.766667).abs() > 0.0001, "Longitude should have changed");

        println!("Tokyo Datum: (35.666667, 139.766667)");
        println!("JGD2000:     ({}, {})", lat, lon);
    }

    #[test]
    fn test_jgd2011_no_transform() {
        // JGD2011座標は変換不要
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let tky_path = workspace_root.join("params/TKY2JGD.par");
        if !tky_path.exists() {
            return;
        }

        let transformer = JgdTransformer::new_tky_only(&tky_path)
            .expect("Failed to create transformer");

        let (lat, lon) = transformer
            .transform_horizontal(35.666667, 139.766667, "02")
            .expect("Transform failed");

        // JGD2011は変換しないので同じ値
        assert!((lat - 35.666667).abs() < 0.000001);
        assert!((lon - 139.766667).abs() < 0.000001);
    }

    #[test]
    fn test_full_transform_with_survey_date_before_earthquake() {
        // 調査日が東北地震前 → 地震補正が適用される
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let tky_path = workspace_root.join("params/TKY2JGD.par");
        let patch_dir = workspace_root.join("params/patch_files");

        if !tky_path.exists() || !patch_dir.exists() {
            println!("Skipping test: params not found");
            return;
        }

        // 2010年の調査（東北地震2011/3/11より前）
        let survey_date = SurveyStartDate::new(2010, 5, 15);
        let transformer = JgdTransformer::with_survey_date(&tky_path, &patch_dir, survey_date)
            .expect("Failed to create transformer");

        // 仙台駅付近 (JGD2000) - 標高15m
        let result = transformer
            .transform_full(38.2601, 140.8824, 15.0, "01")
            .expect("Transform failed");

        println!("Survey date: 2010/5/15 (before Tohoku earthquake)");
        println!("JGD2000: (38.2601, 140.8824, 15.0m)");
        println!("JGD2011: ({}, {}, {}m)", result.0, result.1, result.2);

        // 東北地震補正が適用されるため、標高が変化するはず
        // (ジオイド補正 + 地震補正)
    }

    #[test]
    fn test_full_transform_with_survey_date_after_earthquake() {
        // 調査日が東北地震後 → 地震補正は適用されない（ジオイド補正のみ）
        let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let tky_path = workspace_root.join("params/TKY2JGD.par");
        let patch_dir = workspace_root.join("params/patch_files");

        if !tky_path.exists() || !patch_dir.exists() {
            println!("Skipping test: params not found");
            return;
        }

        // 2015年の調査（東北地震2011/3/11より後）
        let survey_date = SurveyStartDate::new(2015, 6, 20);
        let transformer = JgdTransformer::with_survey_date(&tky_path, &patch_dir, survey_date)
            .expect("Failed to create transformer");

        // 仙台駅付近 (JGD2000) - 標高15m
        let result = transformer
            .transform_full(38.2601, 140.8824, 15.0, "01")
            .expect("Transform failed");

        println!("Survey date: 2015/6/20 (after Tohoku earthquake)");
        println!("JGD2000: (38.2601, 140.8824, 15.0m)");
        println!("JGD2011: ({}, {}, {}m)", result.0, result.1, result.2);

        // 地震補正は適用されない（ジオイド補正のみ）
    }

    #[test]
    fn test_survey_date_parsing() {
        // YYYYMMDD形式
        let date = SurveyStartDate::from_yyyymmdd("20101015");
        assert!(date.is_some());
        let date = date.unwrap();
        assert_eq!(date.year, 2010);
        assert_eq!(date.month, 10);
        assert_eq!(date.day, 15);

        // 個別パース
        let date = SurveyStartDate::from_parts(Some("2011"), Some("3"), Some("10"));
        assert!(date.is_some());
        let date = date.unwrap();
        assert_eq!(date.year, 2011);
        assert_eq!(date.month, 3);
        assert_eq!(date.day, 10);
    }

    #[test]
    fn test_earthquake_correction_should_apply() {
        // 地震前の調査 → 適用する
        let before = SurveyStartDate::new(2010, 1, 1);
        assert!(should_apply_earthquake_correction("touhokutaiheiyouoki2011.par", Some(before)));

        // 地震後の調査 → 適用しない
        let after = SurveyStartDate::new(2012, 1, 1);
        assert!(!should_apply_earthquake_correction("touhokutaiheiyouoki2011.par", Some(after)));

        // ジオイド補正は常に適用
        assert!(should_apply_earthquake_correction("hyokorev2014_geoid2011_h.par", Some(before)));
        assert!(should_apply_earthquake_correction("hyokorev2014_geoid2011_h.par", Some(after)));
        assert!(should_apply_earthquake_correction("hyokorev2014_geoid2011_h.par", None));

        // 調査日不明の場合、地震補正は適用しない
        assert!(!should_apply_earthquake_correction("touhokutaiheiyouoki2011.par", None));
    }
}
