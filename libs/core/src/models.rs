use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub attribute: String,
    pub operator: String, // "eq", "percent"
    pub value: serde_json::Value,
    pub enabled: bool,
    pub variant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flag {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

/// Persistence-friendly flag definition (name is the map key in YAML).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlagDef {
    pub description: String,
    pub enabled: bool,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

impl From<(String, FlagDef)> for Flag {
    fn from((name, def): (String, FlagDef)) -> Self {
        Flag {
            name,
            description: def.description,
            enabled: def.enabled,
            rules: def.rules,
        }
    }
}

impl From<&Flag> for (String, FlagDef) {
    fn from(flag: &Flag) -> Self {
        (
            flag.name.clone(),
            FlagDef {
                description: flag.description.clone(),
                enabled: flag.enabled,
                rules: flag.rules.clone(),
            },
        )
    }
}
