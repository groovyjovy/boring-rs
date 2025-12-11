/// 緯度経度取得用のトレイト
///
/// 各バージョンのボーリングデータ構造体に実装され、
/// 統一的なインターフェースで緯度経度情報を取得できます。

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
    /// # Returns
    /// `Some((経度, 緯度))` - WGS84座標系での10進数座標
    /// `None` - 座標が取得できない、測地系が不明、または変換に失敗した場合
    pub fn to_wgs84(&self) -> Option<(f64, f64)> {
        let lng = self.longitude.to_decimal()?;
        let lat = self.latitude.to_decimal()?;
        let datum = self
            .geodetic_system
            .as_ref()
            .map(|s| GeodeticDatum::from_code(s))
            .unwrap_or(GeodeticDatum::Unknown);
        let from_crs = datum.epsg_code()?;
        let proj = proj::Proj::new_known_crs(from_crs, "EPSG:4326", None).ok()?;
        proj.convert((lng, lat)).ok()
    }
}

/// WGS84座標を取得するためのトレイト
pub trait GeoCoordinate {
    /// WGS84座標系での (経度, 緯度) を取得
    fn to_wgs84(&self) -> Option<(f64, f64)>;
}

impl GeoCoordinate for crate::boring_structs_400::Boring400 {
    fn to_wgs84(&self) -> Option<(f64, f64)> {
        let lng_lat = &self.title.longitude_latitude;
        let loc = GeoLocation {
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
        };
        loc.to_wgs84()
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
    fn test_to_wgs84_from_tokyo() {
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

        let (lng, lat) = loc.to_wgs84().unwrap();
        // 日本測地系からWGS84への変換で、約10秒（約300m）程度のずれがある
        assert!((lng - 139.76).abs() < 0.02);
        assert!((lat - 35.67).abs() < 0.02);
    }

    #[test]
    fn test_to_wgs84_from_jgd2000() {
        // JGD2000（コード: 01）での座標はWGS84とほぼ同一
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
            geodetic_system: Some("01".to_string()),
        };

        let (lng, lat) = loc.to_wgs84().unwrap();
        // JGD2000はWGS84とほぼ同一なので、変換後も同じ値
        assert!((lng - 139.76666666).abs() < 0.0001);
        assert!((lat - 35.66666666).abs() < 0.0001);
    }
}
