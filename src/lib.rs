//! *Structures to deserialize [OBO Foundry](http://www.obofoundry.org) listings into.*
//!
//! [![TravisCI](https://img.shields.io/travis/althonos/obofoundry.rs/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/obofoundry.rs/branches)
//! [![Codecov](https://img.shields.io/codecov/c/gh/althonos/obofoundry.rs/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/obofoundry.rs)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/obofoundry.rs)
//! [![Crate](https://img.shields.io/crates/v/obofoundry.svg?maxAge=600&style=flat-square)](https://crates.io/crates/obofoundry)
//! [![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/obofoundry)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/obofoundry.rs/blob/master/CHANGELOG.md)
//!
//! This library provides structs that can make use of `serde_yaml` and
//! `serde_json` to deserialize the table of ontologies provided by the
//! OBO Foundry. It provides a safe and efficient alternative to manually
//! parsing the obtained JSON/YAML, which is actually quite tricky since
//! there is no actual scheme available. Use the [`Foundry`]("struct.Foundry.html")
//! type as an entry point for deserialization.
//!
//! # Example
//!
//! Download the `ontologies.yml` table from the OBO Foundry and use it to
//! extract all PURLs to ontologies in the OBO format:
//!
//! ```rust
//! extern crate obofoundry;
//! extern crate ureq;
//! extern crate serde_yaml;
//! use std::io::Read;
//!
//! const URL: &'static str = "http://www.obofoundry.org/registry/ontologies.yml";
//!
//! fn main() {
//!
//!     let res = ureq::get(URL).call();
//!     let reader = res.unwrap().into_reader();
//!
//!     let foundry: obofoundry::Foundry = serde_yaml::from_reader(reader).unwrap();
//!     for ontology in &foundry.ontologies {
//!         for product in &ontology.products {
//!             if product.id.ends_with(".obo") {
//!                 println!("{} - {}", product.id, product.ontology_purl)
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Changelog
//!
//! This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
//! and provides a [changelog](https://github.com/althonos/obofoundry.rs/blob/master/CHANGELOG.md)
//! in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.

#![doc(
    html_logo_url = "https://github.com/OBOFoundry/OBOFoundry.github.io/raw/master/images/foundrylogo.png"
)]

extern crate serde;
extern crate url;

use serde::de::Deserializer;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

/// Deserialize an optional `bool` encoded as a 0 or a 1.
fn optional_bool01<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    u8::deserialize(deserializer).map(|n| Some(n != 0))
}

/// Deserialize a possibly missing vector into an empty one.
fn optional_vector<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::deserialize(deserializer).map(|opt| opt.unwrap_or_else(Vec::new))
}

/// Deserialize a vector of `Example`.
fn examples_vector<'de, D>(deserializer: D) -> Result<Vec<Example>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum MaybeExample {
        String(Url),
        Example(Example),
    }

    Vec::<MaybeExample>::deserialize(deserializer).map(|v| {
        v.into_iter()
            .map(|mex| match mex {
                MaybeExample::Example(e) => e,
                MaybeExample::String(url) => Example {
                    url: url,
                    description: None,
                },
            })
            .collect()
    })
}

/// Deserialize the `mireots_from` field of a product
fn mireots_vector<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Mireots {
        Text(String),
        Seq(Vec<String>),
    }

    impl From<Mireots> for Vec<String> {
        fn from(m: Mireots) -> Vec<String> {
            match m {
                Mireots::Text(t) => vec![t],
                Mireots::Seq(s) => s,
            }
        }
    }

    Option::<Mireots>::deserialize(deserializer)
        .map(|opt| opt.map(Vec::<String>::from).unwrap_or_else(Vec::new))
}

/// Returns `true`.
const fn bool_true() -> bool {
    true
}

/// Returns `false`.
const fn bool_false() -> bool {
    false
}

/// An index of ontologies following the OBO Foundry principles.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Foundry {
    pub ontologies: Vec<Ontology>,
}

/// A comprehensive table of informations about an ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ontology {
    pub aberowl_id: Option<String>,
    pub activity_status: ActivityStatus,
    #[serde(rename = "alternativePrefix", alias = "alternatePrefix")]
    pub alternative_prefix: Option<String>,
    pub biosharing: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub browsers: Vec<Browser>,
    pub build: Option<Build>,
    pub canonical: Option<String>,
    pub contact: Option<Contact>,
    #[serde(rename = "createdWith")]
    pub created_with: Option<String>,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub dependencies: Vec<Dependency>,
    pub development: Option<Development>,
    pub depicted_by: Option<String>,
    #[serde(default)]
    pub documentation: Option<Url>,
    pub domain: Option<String>,
    #[serde(default, rename = "DO wiki")]
    pub do_wiki: Option<Url>,
    #[serde(rename = "exampleClass")]
    pub example_class: Option<String>,
    #[serde(default)]
    pub facebook: Option<Url>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub funded_by: Vec<Funding>,
    pub google_plus: Option<String>,
    pub homepage: Option<String>,
    pub id: String,
    #[serde(default = "bool_true")]
    pub in_foundry: bool,
    pub in_foundry_order: Option<usize>,
    pub integration_server: Option<String>,
    #[serde(default = "bool_false")]
    pub is_obsolete: bool,
    #[serde(default, deserialize_with = "optional_vector")]
    pub jobs: Vec<Job>,
    pub label: Option<String>,
    pub layout: String,
    pub license: Option<License>,
    pub mailing_list: Option<String>,
    #[serde(default)]
    pub ontology_purl: Option<Url>,
    #[serde(default)]
    pub page: Option<Url>,
    #[serde(rename = "preferredPrefix")]
    pub preferred_prefix: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub products: Vec<Product>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub publications: Vec<Publication>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub redirects: Vec<Redirect>,
    pub releases: Option<String>,
    pub replaced_by: Option<String>,
    #[serde(default)]
    pub repository: Option<Url>,
    pub review: Option<Review>,
    #[serde(default)]
    pub slack: Option<Url>,
    pub source: Option<String>,
    #[serde(default)]
    pub source_url: Option<Url>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub tags: Vec<String>,
    pub taxon: Option<Taxon>,
    pub termgenie: Option<String>,
    pub title: String,
    #[serde(default, alias = "issue")]
    pub tracker: Option<Url>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub twitter: Option<String>,
    #[serde(default, alias = "used_by", deserialize_with = "optional_vector")]
    pub usages: Vec<Usage>,
    pub validate: Option<bool>,
    #[serde(rename = "wasDerivedFrom")]
    pub was_derived_from: Option<String>,
    pub wikidata_template: Option<String>,
}

/// A review for a particular ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Review {
    #[serde(rename = "date")]
    year: u16,
    document: Option<Document>,
}

/// A review document.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Document {
    label: String,
    link: Url,
}

/// A redirection to another location.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Redirect {
    #[serde(rename = "match")]
    pub path: String,
    pub url: Url,
}

/// Metadata concerning the development of the ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Development {
    pub id_policy: String,
}

/// Reference to a particular dependency.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    pub id: String,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub subset: Option<String>,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub connects: Vec<Dependency>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub publications: Vec<Publication>,
}

/// Information about the way an ontology is built.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Build {
    pub checkout: Option<String>,
    #[serde(deserialize_with = "optional_bool01", default = "Default::default")]
    pub infallible: Option<bool>,
    pub insert_ontology_id: Option<bool>,
    pub method: Option<BuildMethod>,
    pub notes: Option<String>,
    pub oort_args: Option<String>,
    pub path: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub publications: Vec<Publication>,
    #[serde(default)]
    pub source_url: Option<Url>,
    pub system: Option<BuildSystem>,
    pub email_cc: Option<String>,
}

/// The build method for an ontology build.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildMethod {
    Archive,
    Obo2Owl,
    Owl2Obo,
    Vcs,
}

/// The build system for an ontology build.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildSystem {
    Git,
    Svn,
}

/// The legal information about an ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub label: String,
    pub logo: Option<String>,
    pub url: Url,
}

/// The corresponding editor of an ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Contact {
    pub email: Option<String>,
    #[serde(alias = "contact")]
    pub github: Option<String>,
    pub label: String,
    #[serde(default)]
    pub orcid: Option<String>,
}

/// A CI/CD job pipeline running for the ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: JobType,
}

/// The type of a job pipeline.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum JobType {
    #[serde(rename = "travis-ci")]
    TravisCi,
    #[serde(rename = "github-action")]
    GithubAction,
    DryRunBuild,
    ReleaseBuild,
}

/// A released product of an ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Product {
    pub id: String,
    pub name: Option<String>,
    pub is_canonical: Option<bool>,
    pub contact: Option<Contact>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub connects: Vec<Dependency>,
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    #[serde(default)]
    pub homepage: Option<Url>,
    pub license: Option<String>,
    #[serde(default, deserialize_with = "mireots_vector")]
    pub mireots_from: Vec<String>,
    pub ontology_purl: Url,
    pub page: Option<String>,
    pub title: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub uses: Vec<String>,
    pub taxon: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub status: Option<String>,
}

/// A publication relevant to the ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub id: String,
    pub title: Option<String>,
    #[serde(default = "bool_false")]
    pub preferred: bool
}

/// A taxon specifically relevant to the ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Taxon {
    pub id: String,
    pub label: Option<String>,
}

/// A relevant project an ontology is used in.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Usage {
    pub description: Option<String>,
    #[serde(default, deserialize_with = "examples_vector", alias = "example")]
    pub examples: Vec<Example>,
    #[serde(alias = "url")]
    pub user: Url,
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<UsageType>,
    #[serde(rename = "seeAlso")]
    pub see_also: Option<String>,
    pub reference: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub publications: Vec<Publication>,
}

/// The way an ontology can be used in a project.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageType {
    Annotation,
    OwlImport,
    Query,
    #[serde(rename = "Database")]
    Database,
    Application,
    #[serde(rename = "database architecture")]
    DatabaseArchitecture,
    Analysis,
    #[serde(rename = "annotation and query")]
    AnnotationQuery,
    #[serde(rename = "data-annotation")]
    DataAnnotation,
    #[serde(rename = "dataset-description")]
    DatasetDescription,
    Mapping,
}

/// A reference to an example usage of the ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub description: Option<String>,
    pub url: Url,
}

/// The current development status of the ontology development.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum ActivityStatus {
    Active,
    Inactive,
    Orphaned,
}

/// A reference to a browser for the ontology.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Browser {
    pub label: String,
    pub title: String,
    pub url: Url,
}


/// A funding reference.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Funding {
    pub id: Url,
    pub title: String,
}