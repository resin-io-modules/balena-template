# Change Log

All notable changes to this project will be documented in this file
automatically by Versionist. DO NOT EDIT THIS FILE MANUALLY!
This project adheres to [Semantic Versioning](http://semver.org/).

# v0.5.7
## (2019-06-11)

* Fix cargo lint warnings [Giovanni Garufi]

# v0.5.6
## (2019-03-21)

* Update to newest version of webpack [Cyryl Płotnicki]

# v0.5.5
## (2019-03-21)

* Fix CI build. [Cyryl Płotnicki]
* Fix cargo publish [Giovanni Garufi]
* Test CI publish [Giovanni Garufi]

# v0.5.4
## (2019-03-14)

* Delete .travis.yml [Giovanni Garufi]
* Change repo type to rust-public-crate [Giovanni Garufi]

# v0.5.3
## (2019-02-08)

* Fix stack overflow [Robert Vojta]

# v0.5.2
## (2019-02-08)

* Add Fuzzer support [Cyryl Płotnicki]

# v0.5.1
## (2019-02-05)

* Compile wasm bindings conditionally [Robert Vojta]

# v0.5.0
## (2019-02-01)

* Rename the js files in the resulting npm package. [Cyryl Płotnicki]

# v0.4.0
## (2019-01-30)

* Add ternary operator support [Robert Vojta]

# v0.3.1
## (2019-01-28)

* Add MIN/MAX functions [Robert Vojta]

# v0.3.0
## (2019-01-28)

* NOW() accepts one boolean argument only [Robert Vojta]
* Add POW & LOG10 functions [Robert Vojta]
* Replace kwargs with positional arguments [Robert Vojta]

# v0.2.0
## (2019-01-25)

* Function / filters must be in UPPER case [Robert Vojta]

# v0.1.0
## (2019-01-09)

* Rename $$eval to $$formula [Robert Vojta]

# v0.0.26
## (2019-01-07)

* Fix WASM build [Cyryl Płotnicki]

# v0.0.25
## (2018-12-06)

* Rust toolchain stable [Robert Vojta]

# v0.0.24
## (2018-12-06)

* Do not require jest to be installed globally [Cyryl Płotnicki]

# v0.0.23
## (2018-12-06)

* Remove -preview suffix for clippy, rustfmt [Robert Vojta]
* Add changelog link to readme [Robert Vojta]
* Replace wasm32 triplet with target_arch [Robert Vojta]

# v0.0.22
## (2018-11-27)

* Fix now() for wasm [Robert Vojta]
* Fix uuidv4() for wasm [Robert Vojta]

# v0.0.21
## (2018-11-27)

* Cleanup unneeded dependencies in the browser example [Cyryl Płotnicki]

# v0.0.20
## (2018-11-27)

* Add temporary fix for the now() fn (wasm) [Robert Vojta]

# v0.0.19
## (2018-11-26)

* Add note about functions & NPM package [Robert Vojta]

# v0.0.18
## (2018-11-26)

* Make browser example interactive [Lucian]

# v0.0.17
## (2018-11-26)

* Improve documentation for Javascript [Lucian]

# v0.0.16
## (2018-11-26)

* Rename eval to evaluate [Robert Vojta]
* Loosen up requirements for the node package in examples [Cyryl Płotnicki]
* Use `helper::eval` as the function to be exported to WASM [Cyryl Płotnicki]
* Basic wasm support [Cyryl Płotnicki]

# v0.0.15
## (2018-11-21)

* Reformat with new rustfmt [Cyryl Płotnicki]
* Optimize CI scripts [Cyryl Płotnicki]

# v0.0.14
## (2018-11-19)

* Add docs links to README.md [Robert Vojta]
* Add expression language docs [Robert Vojta]
* Update README [Robert Vojta]

# v0.0.13
## (2018-11-19)

* Add eval & eval_with_engine helper functions [Robert Vojta]
* Move engine related structs to engine module [Robert Vojta]

# v0.0.12
## (2018-11-16)

* Add canonicalize to Identifier [Robert Vojta]

# v0.0.11
## (2018-11-16)

* Simplify identifier values [Robert Vojta]

# v0.0.10
## (2018-11-16)

* Replace error-chain [Robert Vojta]

# v0.0.9
## (2018-11-14)

* Internal optimisation [Robert Vojta]

# v0.0.8
## (2018-11-14)

* Add crate documentation [Robert Vojta]

# v0.0.7
## (2018-11-08)

* Refactoring of tests [Robert Vojta]

## v0.0.6 - 2018-11-08

* Add context aware evaluation (this, super) [Robert Vojta]

## v0.0.5 - 2018-11-06

* Add slugify filter [Robert Vojta]
* Add trim filter [Robert Vojta]
* Add time, date, datetime filters [Robert Vojta]
* Add now() function [Robert Vojta]

## v0.0.4 - 2018-11-05

* Add dotted integer index [Robert Vojta]

## v0.0.3 - 2018-11-05

* Allow to register custom function / filter [Robert Vojta]

## v0.0.2 - 2018-11-05

* Change type only [Robert Vojta]
* Update email address [Robert Vojta]
