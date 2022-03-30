# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/althonos/obofoundry.rs/compare/v0.8.3...HEAD


## [v0.8.3] - 2022-03-30

[v0.8.3]: https://github.com/althonos/obofoundry.rs/compare/v0.8.2...v0.8.3

### Added
- `tags` field to `Ontology`.


## [v0.8.2] - 2022-01-11

[v0.8.2]: https://github.com/althonos/obofoundry.rs/compare/v0.8.1...v0.8.2

### Added
- `orcid` field to `Contact`.
- `status` field to `Product`.


## [v0.8.1] - 2022-01-11

[v0.8.1]: https://github.com/althonos/obofoundry.rs/compare/v0.8.0...v0.8.1

### Added
- `Mapping` variant to `UsageType`.


## [v0.8.0] - 2021-02-08

[v0.7.0]: https://github.com/althonos/obofoundry.rs/compare/v0.7.0...v0.8.0

### Added
- `publications` field to `Usage`.
- `DataAnnotation` and `DatasetDescription` variant to `UsageType`.

### Changed
- `mireots_from` field of `Product` is now a sequence.


## [v0.7.0] - 2020-11-27

[v0.7.0]: https://github.com/althonos/obofoundry.rs/compare/v0.6.0...v0.7.0

### Added
- `slack` field to `Ontology`.

### Changed
- Replaced `reqwest` with `ureq` in tests and examples.


## [v0.6.0] - 2020-04-10

### Added
- `review` field to `Ontology`.
- `AnnotationQuery` variants to `UsageType`.

[v0.6.0]: https://github.com/althonos/obofoundry.rs/compare/v0.5.0...v0.6.0


## [v0.5.0] - 2020-02-28

### Added
- `aberowl_id` field to `Ontology`.
- `DatabaseArchitecture` and `Analysis` variants to `UsageType`.

[v0.5.0]: https://github.com/althonos/obofoundry.rs/compare/v0.4.0...v0.5.0


## [v0.4.0] - 2019-09-28

### Added
- `name` field of `Product` structure.

[v0.4.0]: https://github.com/althonos/obofoundry.rs/compare/v0.3.0...v0.4.0


## [v0.3.0] - 2019-08-07

### Changed
- `email` field of `Contact` structure is now optional.

[v0.3.0]: https://github.com/althonos/obofoundry.rs/compare/v0.2.0...v0.3.0


## [v0.2.0] - 2019-06-11

### Added
- `source_url` field to `Ontology` structure.

[v0.2.0]: https://github.com/althonos/obofoundry.rs/compare/v0.1.2...v0.2.0


## [v0.1.2] - 2019-03-30

### Changed
- Removed unused `dox` feature.

### Documentation
- Remove changelog section from docs generated with `rustdoc`
- Change target of *Keep a Changelog* badges to refer to the rendered `CHANGELOG.md` file.

[v0.1.2]: https://github.com/althonos/obofoundry.rs/compare/v0.1.1...v0.1.2


## [v0.1.1] - 2019-03-21

### Fixed
- Fixed parsing of `Url` in `examples_vector` private deserializer.

### Documentation
- Add docstring to `src/lib.rs` with embedded changelog.
- Add docstrings to all structures and enums in `src/lib.rs`.
- Add OBO Foundry logo to API doc generated with `rustdoc`.

[v0.1.1]: https://github.com/althonos/obofoundry.rs/compare/v0.1.0...v0.1.1


## [v0.1.0] - 2019-03-19

Initial release.

[v0.1.0]: https://github.com/althonos/obofoundry.rs/compare/95e6d4b...v0.1.0
