# `obofoundry.rs` [![Star me](https://img.shields.io/github/stars/althonos/obofoundry.rs.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/obofoundry.rs/stargazers)

*Structures to deserialize [OBO Foundry] listings into.*

[OBO Foundry]: http://www.obofoundry.org/


[![Actions](https://img.shields.io/github/actions/workflow/status/althonos/obofoundry.rs/test.yml?style=flat-square&maxAge=600)](https://github.com/althonos/obofoundry.rs/actions)
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
obofoundry = "0.11"
serde_yaml = "0.8"
```

Then use the `serde` framework to load the listings:

```rust,ignore
extern crate obofoundry;
extern crate serde_yaml;

let yaml_data = include_str!("...");
let foundry: obofoundry::Foundry = serde_yaml::from_str(&yml).unwrap();
```

It's also possible to use an HTTP library to load the listings from the OBO Foundry
website directly, for instance using [`ureq`](https://crates.io/crates/ureq):

```rust
extern crate obofoundry;
extern crate ureq;
extern crate serde_yaml;

let url = "http://www.obofoundry.org/registry/ontologies.yml";

let res = ureq::get(url).call();
let reader = res.unwrap().into_reader();
let foundry: obofoundry::Foundry = serde_yaml::from_reader(reader).unwrap();
```

## Example

Download the `ontologies.yml` table from the OBO Foundry and use it to
extract all PURLs to ontologies in the OBO format:

```rust
extern crate obofoundry;
extern crate ureq;
extern crate serde_yaml;
use std::io::Read;

const URL: &'static str = "http://www.obofoundry.org/registry/ontologies.yml";

fn main() {
    let res = ureq::get(URL).call();
    let reader = res.unwrap().into_reader();

    let foundry: obofoundry::Foundry = serde_yaml::from_reader(reader).unwrap();
    for ontology in &foundry.ontologies {
        for product in &ontology.products {
            if product.id.ends_with(".obo") {
                println!("{} - {}", product.id, product.ontology_purl)
            }
        }
    }
}
```


See the online documentation at [`docs.rs`](https://docs.rs/obofoundry) for more examples.

## Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
and provides a [changelog](https://github.com/althonos/obofoundry.rs/blob/master/CHANGELOG.md)
in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.

## License

This library is provided under the open-source
[MIT license](https://choosealicense.com/licenses/mit/).
