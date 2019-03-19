extern crate serde;
extern crate url;

use serde::de::Deserializer;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

fn optional_bool01<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum MaybeBool {
        Opt(Option<u8>),
        Bool(u8),
    }

    match MaybeBool::deserialize(deserializer) {
        Ok(MaybeBool::Opt(opt)) => Ok(opt.map(|n| n != 0)),
        Ok(MaybeBool::Bool(n)) => Ok(Some(n != 0)),
        Err(e) => Err(e),
    }
}

fn optional_vector<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match Option::deserialize(deserializer) {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

fn examples_vector<'de, D>(deserializer: D) -> Result<Vec<Example>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum MaybeExample {
        String(String),
        Example(Example),
    }

    Vec::<MaybeExample>::deserialize(deserializer).map(|v| {
        v.into_iter()
            .map(|mex| match mex {
                MaybeExample::Example(e) => e,
                MaybeExample::String(s) => Example {
                    url: Url::parse(&s).unwrap(),
                    description: None,
                },
            })
            .collect()
    })
}

const fn bool_true() -> bool {
    true
}

const fn bool_false() -> bool {
    false
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Foundry {
    pub ontologies: Vec<Ontology>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ontology {
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
    #[serde(default, with = "url_serde")]
    pub documentation: Option<Url>,
    pub domain: Option<String>,
    #[serde(default, rename = "DO wiki", with = "url_serde")]
    pub do_wiki: Option<Url>,
    #[serde(rename = "exampleClass")]
    pub example_class: Option<String>,
    #[serde(default, with = "url_serde")]
    pub facebook: Option<Url>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub funded_by: Vec<String>,
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
    #[serde(default, with = "url_serde")]
    pub ontology_purl: Option<Url>,
    #[serde(default, with = "url_serde")]
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
    #[serde(default, with = "url_serde")]
    pub repository: Option<Url>,
    pub source: Option<String>,
    pub taxon: Option<Taxon>,
    pub termgenie: Option<String>,
    pub title: String,
    #[serde(default, alias = "issue", with = "url_serde")]
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

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Redirect {
    #[serde(rename = "match")]
    pub path: String,
    #[serde(with = "url_serde")]
    pub url: Url,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Development {
    pub id_policy: String,
}

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
    #[serde(default, with = "url_serde")]
    pub source_url: Option<Url>,
    pub system: Option<BuildSystem>,
    pub email_cc: Option<String>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildMethod {
    Archive,
    Obo2Owl,
    Owl2Obo,
    Vcs,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildSystem {
    Git,
    Svn,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub label: String,
    pub logo: Option<String>,
    #[serde(with = "url_serde")]
    pub url: Url,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Contact {
    pub email: String,
    #[serde(alias = "contact")]
    pub github: Option<String>,
    pub label: String,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: JobType,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum JobType {
    #[serde(rename = "travis-ci")]
    TravisCi,
    DryRunBuild,
    ReleaseBuild,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Product {
    pub id: String,
    pub is_canonical: Option<bool>,
    pub contact: Option<Contact>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub connects: Vec<Dependency>,
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    #[serde(default, with = "url_serde")]
    pub homepage: Option<Url>,
    pub license: Option<String>,
    pub mireots_from: Option<String>,
    #[serde(with = "url_serde")]
    pub ontology_purl: Url,
    pub page: Option<String>,
    pub title: Option<String>,
    #[serde(default, deserialize_with = "optional_vector")]
    pub uses: Vec<String>,
    pub taxon: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub id: String,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Taxon {
    pub id: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Usage {
    pub description: Option<String>,
    #[serde(default, deserialize_with = "examples_vector", alias = "example")]
    pub examples: Vec<Example>, // FIXME?: list or dict
    #[serde(alias = "url", with = "url_serde")]
    pub user: Url,
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<UsageType>, // FIXME: enum
    #[serde(rename = "seeAlso")]
    pub see_also: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageType {
    Annotation,
    OwlImport,
    Query,
    #[serde(rename = "Database")]
    Database,
    Application,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub description: Option<String>,
    #[serde(with = "url_serde")]
    pub url: Url,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum ActivityStatus {
    Active,
    Inactive,
    Orphaned,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Browser {
    pub label: String,
    pub title: String,
    #[serde(with = "url_serde")]
    pub url: Url,
}
