[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/krossbar-settings-viewer.svg
[crates-url]: https://crates.io/crates/krossbar-settings-viewer
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/krossbar-platform/krossbar-bus/blob/main/LICENSE
[actions-badge]: https://github.com/krossbar-platform/krossbar-settings/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/krossbar-platform/krossbar-settings/actions/workflows/ci.yml

# krossbar-settings-viewer

Krossbar settings viewer

A convenient tool to read Krossbar settings files.
It uses [DEFAULT_SETTINGS_DIR](https://docs.rs/krossbar-settings-common/latest/krossbar_settings_common/constant.DEFAULT_SETTINGS_DIR.html) by default and is able to query individual
fields, which is simpler, than reading corresponding JSON by hands.

## Usage

```sh
Krossbar settings viewer

USAGE:
    krossbar-settings-viewer <TARGET_SERVICE> [KEY]

ARGS:
    <TARGET_SERVICE>    Service to monitor
    <KEY>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```
