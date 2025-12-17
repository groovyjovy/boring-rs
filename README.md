# boring-rs

日本のボーリング柱状図XMLファイルをパースするRustライブラリ

## 概要

国土交通省が定義するボーリング柱状図XML形式に対応したパーサーです。Shift_JISエンコーディングのXMLファイルを読み込み、構造化されたデータとして扱えます。

## 特徴

- **複数バージョン対応**: DTD v1.10, v2.10, v3.00, v4.00
- **座標変換**: Tokyo Datum / JGD2000 → JGD2011 → WGS84
- **地震補正**: 調査開始日に基づく地震補正の自動適用

## 使い方

### パース

```rust
use boring_parser::{boring_structs_400::Boring400, parser::Parse};

let boring = Boring400::parse_from_str(&xml_str)?;
```

### 座標変換

```rust
use boring_parser::coordinate::GeoCoordinate;
use boring_parser::transform::{JgdTransformer, SurveyStartDate};

// 1. 変換器を初期化（パラメータファイルが必要）
let transformer = JgdTransformer::with_survey_date(
    "path/to/TKY2JGD.par",
    "path/to/patch_files",  // horizontal/, elevation/ を含むディレクトリ
    SurveyStartDate::new(2010, 5, 15),
)?;

// 2. 座標を取得してWGS84に変換
let loc = boring.geo_location();
let (lng, lat) = loc.to_wgs84(&transformer)?;

// 3. 標高も含む場合
let (lat, lng, alt) = transformer.transform_full(lat, lng, alt, "01")?;
```

### パラメータファイル

座標変換には国土地理院のパラメータファイルが必要です：

- `TKY2JGD.par` - Tokyo Datum → JGD2000
- `patch_files/horizontal/` - 水平補正（地震補正）
- `patch_files/elevation/` - 標高補正（ジオイド＋地震補正）

ダウンロード: https://www.gsi.go.jp/sokuchikijun/sokuchikijun41012.html

## 測地系コード

| コード | 測地系 |
|--------|--------|
| 00 | 日本測地系（Tokyo Datum） |
| 01 | 世界測地系2000（JGD2000） |
| 02 | 世界測地系2011（JGD2011） |
