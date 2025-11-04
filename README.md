# boring-rs

日本のボーリング柱状図XMLファイルをパースして、JSON形式に変換するRustライブラリ＆CLIツール

## 概要

国土交通省が定義するボーリング柱状図XML形式（DTD v1.10）に対応したパーサーです。Shift_JISエンコーディングのXMLファイルを読み込み、構造化されたJSONデータとして出力します。

## 特徴

- 複数バージョン対応: DTD v1.10に対応

## 使い方

### ビルド

```bash
cargo build --release
```

### 実行

```bash
./target/release/columnar-section-converter -i <入力XMLファイル> --output-file <出力JSONファイル>

# 例
./target/release/columnar-section-converter -i docs/dtd/BED0110.XML --output-file boring.json
```

## JSON出力形式

### 空要素と要素なしの扱い

- **要素なし**: `null`
- **空要素** (`<tag/>`または`<tag></tag>`): `""`（空文字列）

例：
```xml
<エンジン>
  <エンジン_名称>エンジンA</エンジン_名称>
  <エンジン_能力></エンジン_能力>  <!-- 空要素 -->
</エンジン>
```

↓ JSON

```json
{
  "エンジン_名称": "エンジンA",
  "エンジン_能力": ""
}
```

## 開発

実装仕様は`spec.md`を参照してください。
