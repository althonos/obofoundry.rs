#![allow(dead_code)]

extern crate serde_json;
extern crate serde_yaml;
extern crate ureq;

extern crate obofoundry;

#[test]
fn yaml() {
    let url = "http://www.obofoundry.org/registry/ontologies.yml";

    let res = ureq::get(url).call();
    let reader = res.unwrap().into_reader();

    let _foundry: obofoundry::Foundry = serde_yaml::from_reader(reader).unwrap();
}

#[test]
fn json() {
    let url = "http://www.obofoundry.org/registry/ontologies.jsonld";

    let res = ureq::get(url).call();
    let reader = res.unwrap().into_reader();

    let _foundry: obofoundry::Foundry = serde_json::from_reader(reader).unwrap();
}
