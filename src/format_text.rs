use crate::configuration::Configuration;
use anyhow::Result;
use serde_json::{Map, Value};
use std::option;

/// https://docs.npmjs.com/cli/v10/configuring-npm/package-json
///
/// ```js
/// JSON.stringify([...$('nav.bORCyy ul').querySelectorAll('a.ffgAKS')].map(a => a.innerText))
/// ```
const KEYS: [&str; 32] = [
    "name",
    "version",
    "description",
    "keywords",
    "homepage",
    "bugs",
    "license",
    "author",
    "contributors",
    "funding",
    "files",
    "main",
    "browser",
    "bin",
    "man",
    "directories",
    "repository",
    "scripts",
    "config",
    "dependencies",
    "devDependencies",
    "peerDependencies",
    "peerDependenciesMeta",
    "bundleDependencies",
    "optionalDependencies",
    "overrides",
    "engines",
    "os",
    "cpu",
    "private",
    "publishConfig",
    "workspaces",
];

pub fn format_text(file_text: String, config: &Configuration) -> Result<Option<String>> {
    let value = serde_json::from_str(file_text.as_str()).unwrap();
    let sorted_value = match value {
        Value::Object(map) => {
            let mut sorted_keys = vec![];
            let mut remaining_keys = vec![];

            for key in map.keys() {
                if KEYS.contains(&key.as_str()) {
                    sorted_keys.push(key.clone());
                } else {
                    remaining_keys.push(key.clone());
                }
            }

            let mut sorted_map = Map::<String, Value>::new();

            for key in sorted_keys {
                sorted_map.insert(key.clone(), map.get(&key).unwrap().clone());
            }
            for key in remaining_keys {
                sorted_map.insert(key.clone(), map.get(&key).unwrap().clone());
            }

            Value::Object(sorted_map.into_iter().collect())
        }
        _ => value,
    };
    serde_json::to_string_pretty(&sorted_value)
        .map(option::Option::Some)
        .map_err(Into::into)
}
