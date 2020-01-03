#![allow(dead_code)]

extern crate reqwest;
extern crate serde_json;
extern crate serde_yaml;

extern crate obofoundry;

use reqwest::header::CONTENT_LENGTH;
use std::io::Read;
use std::str::FromStr;

#[test]
fn yaml() {
    let url = "http://www.obofoundry.org/registry/ontologies.yml";

    let mut res = reqwest::get(url).unwrap();
    let mut yml = match res.headers().get(CONTENT_LENGTH) {
        Some(s) => String::with_capacity(usize::from_str(s.to_str().unwrap()).unwrap()),
        None => String::new(),
    };
    res.read_to_string(&mut yml).unwrap();

    let _foundry: obofoundry::Foundry = serde_yaml::from_str(&yml).unwrap();
}

#[test]
fn json() {
    let url = "http://www.obofoundry.org/registry/ontologies.jsonld";

    let mut res = reqwest::get(url).unwrap();
    let mut jsn = match res.headers().get(CONTENT_LENGTH) {
        Some(s) => String::with_capacity(usize::from_str(s.to_str().unwrap()).unwrap()),
        None => String::new(),
    };
    res.read_to_string(&mut jsn).unwrap();

    let _foundry: obofoundry::Foundry = serde_json::from_str(&jsn).unwrap();
}
