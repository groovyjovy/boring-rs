//! 共通型定義
//!
//! 複数のDTDバージョンで共通して使用される型

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// フリー情報
// DTD: <!ELEMENT フリー情報 (#PCDATA)>
// 注: DTDではテキストのみだが、実際のデータでは子要素を含む場合がある
// ============================================================================

/// フリー情報: テキストのみならSome(String)、子要素があればNone
#[derive(Debug, Serialize, Default, Clone)]
pub struct FreeInfo(pub Option<String>);

impl<'de> Deserialize<'de> for FreeInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FreeInfoVisitor)
    }
}

struct FreeInfoVisitor;

impl<'de> Visitor<'de> for FreeInfoVisitor {
    type Value = FreeInfo;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or any element (which will be skipped)")
    }

    // テキストの場合
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let trimmed = value.trim();
        Ok(FreeInfo((!trimmed.is_empty()).then(|| trimmed.to_string())))
    }

    // 子要素がある場合はmapとして来る - スキップしてNoneを返す
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while map.next_entry::<de::IgnoredAny, de::IgnoredAny>()?.is_some() {}
        Ok(FreeInfo(None))
    }
}
