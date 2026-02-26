use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::models::{Flag, Rule};

/// Evaluates a flag for a user. Returns true if the flag is enabled for the given context.
///
/// Rules are evaluated in order. First matching rule wins. If no rule matches, falls back to `flag.enabled`.
/// - Global on/off: flag.enabled
/// - Rule "eq": attribute value equals rule value → rule.enabled
/// - Rule "percent": stable bucket hash(user_id + flag_name) % 100 < value → rule.enabled
pub fn evaluate(
    flag: &Flag,
    user_id: &str,
    attributes: &HashMap<String, String>,
) -> bool {
    if !flag.enabled {
        return false;
    }
    let key = format!("{}:{}", flag.name, user_id);
    for rule in &flag.rules {
        if let Some(result) = rule_matches(rule, user_id, &key, attributes) {
            return result;
        }
    }
    flag.enabled
}

fn rule_matches(
    rule: &Rule,
    _user_id: &str,
    bucket_key: &str,
    attributes: &HashMap<String, String>,
) -> Option<bool> {
    match rule.operator.as_str() {
        "eq" => {
            let attr_value = attributes.get(&rule.attribute)?;
            let matches = match &rule.value {
                serde_json::Value::String(s) => attr_value.as_str() == s.as_str(),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        attr_value == &i.to_string()
                    } else if let Some(f) = n.as_f64() {
                        attr_value == &f.to_string()
                    } else {
                        false
                    }
                }
                _ => false,
            };
            if matches {
                Some(rule.enabled)
            } else {
                None
            }
        }
        "percent" => {
            let percent = rule.value.as_u64().or_else(|| rule.value.as_i64().map(|i| i as u64))?;
            let bucket = stable_bucket(bucket_key) % 100;
            Some(bucket < percent && rule.enabled)
        }
        _ if rule.attribute.is_empty() || rule.attribute == "*" => Some(rule.enabled),
        _ => None,
    }
}

/// Stable bucket 0..100 for percent rollout (deterministic from key).
fn stable_bucket(key: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() % 101
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Flag;

    fn flag_with_rules(rules: Vec<Rule>) -> Flag {
        Flag {
            name: "test".into(),
            description: "".into(),
            enabled: true,
            rules,
        }
    }

    #[test]
    fn disabled_flag_returns_false() {
        let flag = Flag {
            name: "x".into(),
            description: "".into(),
            enabled: false,
            rules: vec![],
        };
        assert!(!evaluate(&flag, "u1", &HashMap::new()));
    }

    #[test]
    fn no_rules_returns_flag_enabled() {
        let flag = Flag {
            name: "x".into(),
            description: "".into(),
            enabled: true,
            rules: vec![],
        };
        assert!(evaluate(&flag, "u1", &HashMap::new()));
    }

    #[test]
    fn eq_rule_matches() {
        let flag = flag_with_rules(vec![Rule {
            attribute: "country".into(),
            operator: "eq".into(),
            value: serde_json::json!("ES"),
            enabled: true,
            variant: None,
        }]);
        let mut attrs = HashMap::new();
        attrs.insert("country".to_string(), "ES".to_string());
        assert!(evaluate(&flag, "u1", &attrs));
    }

    #[test]
    fn eq_rule_mismatch() {
        let flag = flag_with_rules(vec![Rule {
            attribute: "country".into(),
            operator: "eq".into(),
            value: serde_json::json!("ES"),
            enabled: true,
            variant: None,
        }]);
        let mut attrs = HashMap::new();
        attrs.insert("country".to_string(), "FR".to_string());
        assert!(evaluate(&flag, "u1", &attrs)); // no rule match → fallback flag.enabled
    }

    #[test]
    fn percent_is_stable() {
        let flag = flag_with_rules(vec![Rule {
            attribute: "user_id".into(),
            operator: "percent".into(),
            value: serde_json::json!(100),
            enabled: true,
            variant: None,
        }]);
        let attrs = HashMap::new();
        let r1 = evaluate(&flag, "user123", &attrs);
        let r2 = evaluate(&flag, "user123", &attrs);
        assert_eq!(r1, r2);
    }
}
