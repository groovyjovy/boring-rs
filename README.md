# boring-rs

日本のボーリング柱状図XMLファイルをパースして、JSON形式に変換するRustライブラリ＆CLIツール

## 概要

国土交通省が定義するボーリング柱状図XML形式（DTD v1.10）に対応したパーサーです。Shift_JISエンコーディングのXMLファイルを読み込み、構造化されたJSONデータとして出力します。

## 特徴

- 複数バージョン対応: DTD v1.10に対応

## 使い方

### CLI

```bash
cargo install boring-rs

boring-rs --input-file ./docs/dtd/BED0110.XML --output-file output.json
```

### Library

``` rs
use boring_parser::{boring_structs_110::Boring110, parser::Parse};
let boring_110 = Boring110::parse_from_str(&utf8_str); // Result<Boring110, Box<dyn std::error::Error>>
```

## 出力仕様

実装仕様は`spec.md`を参照してください。
