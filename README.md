# plug_

Plug any lexer into any parser.

NOTE: The aim is to create a sensible interface allowing different lexer and parser implementations to work well with
each other, effortlessly. However, this is currently in a proof-of-concept state and may change significantly.

This crate provides a single trait, `Lexer` which can be implemented by lexers in order to allow them being used
effortlessly by any parser library that supports `plug_`.

## Usage

### Lexer-support provided by `plug_`

Support for lexers that don't support `plug_` is provided as optional features.

* [logos](https://crates.io/crates/logos) (feature: `logos_support`)

### Lexers supporting `plug_`

None, currently.

### Parsers supporting `plug_`

* [yalr](https://github.com/timsueberkrueb/yalr) (will support `plug_` in its first release)

### License

`plug_` is licensed under either of the following licenses, at your option:

* [Apache License Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)
