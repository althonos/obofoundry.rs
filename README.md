# `obofoundry.rs`

*Structures to deserialize [OBO Foundry] listings into.*

[OBO Foundry]: http://www.obofoundry.org/


[![TravisCI](https://img.shields.io/travis/althonos/obofoundry.rs/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/obofoundry.rs/branches)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/obofoundry.rs/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/obofoundry.rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/obofoundry.rs)
[![Crate](https://img.shields.io/crates/v/obofoundry.svg?maxAge=600&style=flat-square)](https://crates.io/crates/obofoundry)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/obofoundry)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/obofoundry.rs/blob/master/CHANGELOG.md)


## Usage

Add the `obofoundry` crate to the `Cargo.toml` manifest, as well as either
`serde_yaml` or `serde_json`:

```toml
[dependencies]
obofoundry = "0.1"
serde_yaml = "0.8"
```

Then use the `serde` framework to load the listings:

```rust
extern crate obofoundry;
extern crate serde_yaml;

let yaml_data = include_str!("...");
let foundry: obofoundry::Foundry = serde_yaml::from_str(&yml).unwrap();
```

It's also possible to use an HTTP library to load the listings from the OBO Foundry
website directly, for instance using `reqwest`:

```rust
extern crate obofoundry;
extern crate reqwest;
extern crate serde_yaml;

let url = "http://www.obofoundry.org/registry/ontologies.yml";

let mut res = reqwest::get(url).unwrap();
let mut yml = String::new();
res.read_to_string(&mut yml);

let foundry: obofoundry::Foundry = serde_yaml::from_str(&yml).unwrap();
```

## Examples

See the online documentation at [`docs.rs`](https://docs.rs/obofoundry) for more examples.
