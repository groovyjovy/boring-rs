//! 緯度経度取得用のモジュール
//!
//! 各バージョンのボーリングデータ構造体に実装され、
//! 統一的なインターフェースで緯度経度情報を取得できます。

use crate::transform::JgdTransformer;

/// 度分秒形式の座標
#[derive(Debug, Clone, PartialEq)]
pub struct DmsCoordinate {
    /// 度
    pub degree: Option<String>,
    /// 分
    pub minute: Option<String>,
    /// 秒
    pub second: Option<String>,
}

impl DmsCoordinate {
    /// 度分秒を10進数（度）に変換
    ///
    /// 度、分、秒のいずれかがNoneまたはパースできない場合はNoneを返す
    pub fn to_decimal(&self) -> Option<f64> {
        let deg = self.degree.as_ref()?.parse::<f64>().ok()?;
        let min = self.minute.as_ref()?.parse::<f64>().ok()?;
        let sec = self.second.as_ref()?.parse::<f64>().ok()?;
        Some(deg + min / 60.0 + sec / 3600.0)
    }
}

/// 測地系の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeodeticDatum {
    /// 日本測地系（Tokyo Datum） - EPSG:4301
    Tokyo,
    /// 世界測地系2000（JGD2000） - EPSG:4612
    Jgd2000,
    /// 世界測地系2011（JGD2011） - EPSG:6668
    Jgd2011,
    /// WGS84 - EPSG:4326
    Wgs84,
    /// 不明な測地系
    Unknown,
}

impl GeodeticDatum {
    /// 測地系コードからGeodeticDatumに変換
    ///
    /// # コード対応表（DTD v4.00準拠）
    /// - "00" → 日本測地系
    /// - "01" → 世界測地系(JGD2000)
    /// - "02" → 世界測地系(JGD2011)
    pub fn from_code(s: &str) -> Self {
        match s.trim() {
            "00" => GeodeticDatum::Tokyo,
            "01" => GeodeticDatum::Jgd2000,
            "02" => GeodeticDatum::Jgd2011,
            _ => GeodeticDatum::Unknown,
        }
    }

    /// EPSGコードを取得
    pub fn epsg_code(&self) -> Option<&'static str> {
        match self {
            GeodeticDatum::Tokyo => Some("EPSG:4301"),
            GeodeticDatum::Jgd2000 => Some("EPSG:4612"),
            GeodeticDatum::Jgd2011 => Some("EPSG:6668"),
            GeodeticDatum::Wgs84 => Some("EPSG:4326"),
            GeodeticDatum::Unknown => None,
        }
    }
}

/// 緯度経度情報
#[derive(Debug, Clone, PartialEq)]
pub struct GeoLocation {
    /// 経度（度分秒）
    pub longitude: DmsCoordinate,
    /// 緯度（度分秒）
    pub latitude: DmsCoordinate,
    /// 測地系コード
    pub geodetic_system: Option<String>,
}

impl GeoLocation {
    /// WGS84座標系に変換した座標を取得
    ///
    /// 国土地理院のTKY2JGD/PatchJGDパラメータを使用して
    /// Tokyo Datum/JGD2000 → JGD2011 → WGS84 の変換を行う。
    ///
    /// # Arguments
    /// * `transformer` - JgdTransformer インスタンス
    ///
    /// # Returns
    /// `Some((経度, 緯度))` - WGS84座標系での10進数座標
    /// `None` - 座標が取得できない場合
    ///
    /// # Example
    /// ```ignore
    /// let transformer = JgdTransformer::new("TKY2JGD.par", "patch_files")?;
    /// let (lng, lat) = location.to_wgs84(&transformer)?;
    /// ```
    pub fn to_wgs84(&self, transformer: &JgdTransformer) -> Option<(f64, f64)> {
        let lng = self.longitude.to_decimal()?;
        let lat = self.latitude.to_decimal()?;
        let system_code = self.geodetic_system.as_deref().unwrap_or("02");

        // Tokyo/JGD2000 → JGD2011 → WGS84 (キャッシュ済みprojを使用)
        transformer.to_wgs84(lat, lng, system_code)
    }

    /// 10進数座標を取得 (変換なし)
    ///
    /// # Returns
    /// `Some((経度, 緯度))` - 10進数座標
    /// `None` - 座標が取得できない場合
    pub fn to_decimal(&self) -> Option<(f64, f64)> {
        let lng = self.longitude.to_decimal()?;
        let lat = self.latitude.to_decimal()?;
        Some((lng, lat))
    }

    /// 測地系を取得
    pub fn datum(&self) -> GeodeticDatum {
        self.geodetic_system
            .as_ref()
            .map(|s| GeodeticDatum::from_code(s))
            .unwrap_or(GeodeticDatum::Unknown)
    }
}

/// 緯度経度情報を取得するためのトレイト
pub trait GeoCoordinate {
    /// GeoLocationを取得
    ///
    /// # Example
    /// ```ignore
    /// let loc = boring.geo_location();
    /// let (lng, lat) = loc.to_jgd2011(&transformer)?;
    /// let (lng, lat) = loc.to_wgs84()?;
    /// ```
    fn geo_location(&self) -> GeoLocation;
}

impl GeoCoordinate for crate::boring_structs_400::Boring400 {
    fn geo_location(&self) -> GeoLocation {
        let lng_lat = &self.title.longitude_latitude;
        GeoLocation {
            longitude: DmsCoordinate {
                degree: lng_lat.longitude_degree.clone(),
                minute: lng_lat.longitude_minute.clone(),
                second: lng_lat.longitude_second.clone(),
            },
            latitude: DmsCoordinate {
                degree: lng_lat.latitude_degree.clone(),
                minute: lng_lat.latitude_minute.clone(),
                second: lng_lat.latitude_second.clone(),
            },
            geodetic_system: lng_lat.geodetic_system.clone(),
        }
    }
}

impl GeoCoordinate for crate::boring_structs_300::Boring300 {
    fn geo_location(&self) -> GeoLocation {
        let lng_lat = &self.title.longitude_latitude;
        GeoLocation {
            longitude: DmsCoordinate {
                degree: lng_lat.longitude_degree.clone(),
                minute: lng_lat.longitude_minute.clone(),
                second: lng_lat.longitude_second.clone(),
            },
            latitude: DmsCoordinate {
                degree: lng_lat.latitude_degree.clone(),
                minute: lng_lat.latitude_minute.clone(),
                second: lng_lat.latitude_second.clone(),
            },
            geodetic_system: lng_lat.geodetic_system.clone(),
        }
    }
}

impl GeoCoordinate for crate::boring_structs_210::Boring210 {
    fn geo_location(&self) -> GeoLocation {
        let lng_lat = &self.title.longitude_latitude;
        GeoLocation {
            longitude: DmsCoordinate {
                degree: lng_lat.longitude_degree.clone(),
                minute: lng_lat.longitude_minute.clone(),
                second: lng_lat.longitude_second.clone(),
            },
            latitude: DmsCoordinate {
                degree: lng_lat.latitude_degree.clone(),
                minute: lng_lat.latitude_minute.clone(),
                second: lng_lat.latitude_second.clone(),
            },
            geodetic_system: lng_lat.geodetic_system.clone(),
        }
    }
}

impl GeoCoordinate for crate::boring_structs_110::Boring110 {
    fn geo_location(&self) -> GeoLocation {
        let lng_lat = &self.title.longitude_latitude;
        GeoLocation {
            longitude: DmsCoordinate {
                degree: lng_lat.longitude_degree.clone(),
                minute: lng_lat.longitude_minute.clone(),
                second: lng_lat.longitude_second.clone(),
            },
            latitude: DmsCoordinate {
                degree: lng_lat.latitude_degree.clone(),
                minute: lng_lat.latitude_minute.clone(),
                second: lng_lat.latitude_second.clone(),
            },
            geodetic_system: lng_lat.geodetic_system.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dms_to_decimal() {
        let dms = DmsCoordinate {
            degree: Some("139".to_string()),
            minute: Some("45".to_string()),
            second: Some("0".to_string()),
        };
        let decimal = dms.to_decimal().unwrap();
        assert!((decimal - 139.75).abs() < 0.0001);
    }

    #[test]
    fn test_dms_to_decimal_with_seconds() {
        // 東京: 139度41分30秒
        let dms = DmsCoordinate {
            degree: Some("139".to_string()),
            minute: Some("41".to_string()),
            second: Some("30".to_string()),
        };
        let decimal = dms.to_decimal().unwrap();
        // 139 + 41/60 + 30/3600 = 139.69166...
        assert!((decimal - 139.69166666).abs() < 0.0001);
    }

    #[test]
    fn test_dms_to_decimal_none() {
        let dms = DmsCoordinate {
            degree: None,
            minute: Some("45".to_string()),
            second: Some("0".to_string()),
        };
        assert!(dms.to_decimal().is_none());
    }

    #[test]
    fn test_geodetic_datum_from_code() {
        assert_eq!(GeodeticDatum::from_code("00"), GeodeticDatum::Tokyo);
        assert_eq!(GeodeticDatum::from_code("01"), GeodeticDatum::Jgd2000);
        assert_eq!(GeodeticDatum::from_code("02"), GeodeticDatum::Jgd2011);
        assert_eq!(GeodeticDatum::from_code("99"), GeodeticDatum::Unknown);
    }

    #[test]
    fn test_to_wgs84() {
        // パラメータファイルが存在する場合のみテスト
        let workspace_root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let tky_path = workspace_root.join("params/TKY2JGD.par");
        let patch_dir = workspace_root.join("params/patch_files");

        if !tky_path.exists() || !patch_dir.exists() {
            println!("Skipping test: params not found");
            return;
        }

        let transformer = crate::transform::JgdTransformer::new(&tky_path, &patch_dir)
            .expect("Failed to create transformer");

        // 日本測地系（コード: 00）での東京駅付近の座標
        let loc = GeoLocation {
            longitude: DmsCoordinate {
                degree: Some("139".to_string()),
                minute: Some("46".to_string()),
                second: Some("0".to_string()),
            },
            latitude: DmsCoordinate {
                degree: Some("35".to_string()),
                minute: Some("40".to_string()),
                second: Some("0".to_string()),
            },
            geodetic_system: Some("00".to_string()),
        };

        let (lng, lat) = loc.to_wgs84(&transformer).unwrap();
        // 日本測地系からWGS84への変換で、約10秒（約300m）程度のずれがある
        assert!((lng - 139.76).abs() < 0.02);
        assert!((lat - 35.67).abs() < 0.02);

        println!("Tokyo Datum → WGS84: ({}, {})", lng, lat);
    }
}
