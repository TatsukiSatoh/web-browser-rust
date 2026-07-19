# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

書籍『［作って学ぶ］ブラウザのしくみ』に沿って実装しているブラウザ **SaBA**。
[WasabiOS](https://github.com/hikalium/wasabi)（`for_saba` ブランチ）上のアプリケーションとして QEMU 内で動作する。コミット履歴は書籍の章単位で進んでいる（現在 ch3 まで完了、HTML トークナイザーを実装中）。

## よく使うコマンド

```bash
# テスト（テストは saba_core 側にある。ルートの `make test` は saba バイナリのビルド+テストのみ）
cd saba_core && cargo test

# 単一テストの実行
cd saba_core && cargo test test_name

# Lint（-D warnings 付き）
make clippy

# WasabiOS ターゲット向けリリースビルド（x86_64-unknown-none）
make build

# WasabiOS のダウンロード〜ビルド〜QEMU での実行まで一括
./rust_on_wasabi.sh
```

### QEMU での動作確認

`src/main.rs` は `host.test:8001` の `test.html` を取得する。実行前にホスト側でリポジトリルートから HTTP サーバーを起動しておくこと：

```bash
python3 -m http.server 8001
```

ポート 8000 はこのマシンでは taskdog-server が 127.0.0.1:8000 を占有しており、QEMU ゲストからの通信がそちらに届くため 8001 を使う。

`rust_on_wasabi.sh` が失敗する場合は `rm -rf build/wasabi` してから再実行する。

## アーキテクチャ

Cargo workspace ではなく、path 依存で繋がった 3 つのクレート構成：

- **`saba`**（ルート、`src/main.rs`）— `#![no_std]` / `#![no_main]` のエントリポイント。`noli` の `entry_point!` マクロで WasabiOS アプリとして起動する。`wasabi` feature（デフォルト有効）が `net_wasabi` と `noli` を引き込む。
- **`saba_core`** — OS 非依存のブラウザエンジン本体（`#![no_std]`、依存ゼロ）。テストはここに書く。
  - `url.rs` — URL パーサー
  - `http.rs` — HTTP レスポンスのパース
  - `renderer/html/` — HTML トークナイザー。WHATWG HTML 仕様のステートマシン（`token.rs` の `State` enum、各バリアントに仕様への doc リンクあり）に従って実装する
- **`net/wasabi`** — WasabiOS 依存のネットワーク層。`noli` の API を使った `HttpClient`

`build/wasabi/` は `rust_on_wasabi.sh` がクローンした WasabiOS 本体（gitignore 済み）。編集対象ではない。

## 制約

- ツールチェインは `nightly-2024-01-01` に固定（`rust-toolchain.toml`）。
- 全クレート `no_std`。`std` は使えないので、`String` / `Vec` などは `alloc` クレートから import する（`extern crate alloc`）。
