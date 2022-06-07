use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    repos: Vec<Repo>,
    #[serde(default = "default_true")]
    fail_fast: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Repo {
    repo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
    hooks: Vec<Hook>,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_stages() -> Vec<Stage> {
    vec![Stage::Commit]
}

fn default_language() -> String {
    "system".to_string()
}

fn default_files() -> String {
    "".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Hook {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    always_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verbose: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pass_filenames: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stages: Option<Vec<Stage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Stage {
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "merge-commit")]
    MergeCommit,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "prepare-commit-msg")]
    PrepareCommitMsg,
    #[serde(rename = "commit-msg")]
    CommitMsg,
    #[serde(rename = "post-checkout")]
    PostCheckout,
    #[serde(rename = "post-commit")]
    PostCommit,
    #[serde(rename = "post-merge")]
    PostMerge,
    #[serde(rename = "post-rewrite")]
    PostRewrite,
    #[serde(rename = "manual")]
    Manual,
}

impl Config {
    pub(crate) fn for_language(language: crate::Language) -> Self {
        match language {
            crate::Language::Rust => Config::rust(),
            crate::Language::Python => Config::python(),
            crate::Language::Go => Config::go(),
        }
    }

    fn rust() -> Self {
        let raw_text = include_str!("templates/rust.yml");
        serde_yaml::from_str(raw_text).expect("error parsing template")
    }

    fn python() -> Self {
        let raw_text = include_str!("templates/python.yml");
        serde_yaml::from_str(raw_text).expect("error parsing template")
    }

    fn go() -> Self {
        let raw_text = include_str!("templates/go.yml");
        serde_yaml::from_str(raw_text).expect("error parsing template")
    }
}
