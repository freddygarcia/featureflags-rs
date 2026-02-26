use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[pyclass]
struct FeatureClient {
    base_url: String,
    client: reqwest::blocking::Client,
    cache: Mutex<HashMap<String, (bool, Instant)>>,
    cache_ttl_secs: u64,
}

#[pymethods]
impl FeatureClient {
    #[new]
    #[pyo3(signature = (server_url, cache_ttl_secs=60))]
    fn new(server_url: String, cache_ttl_secs: u64) -> Self {
        let base_url = server_url.trim_end_matches('/').to_string();
        FeatureClient {
            base_url,
            client: reqwest::blocking::Client::new(),
            cache: Mutex::new(HashMap::new()),
            cache_ttl_secs,
        }
    }

    fn is_enabled(
        &self,
        flag_name: String,
        user_id: Option<String>,
        attributes: Option<HashMap<String, String>>,
    ) -> PyResult<bool> {
        let uid = user_id.as_deref().unwrap_or("");
        let cache_key = cache_key(&flag_name, uid, attributes.as_ref());
        {
            let mut cache = self.cache.lock().map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("cache lock: {}", e))
            })?;
            if let Some((result, at)) = cache.get(&cache_key) {
                if at.elapsed() < Duration::from_secs(self.cache_ttl_secs) {
                    return Ok(*result);
                }
                cache.remove(&cache_key);
            }
        }
        let url = self.build_evaluate_url(&flag_name, uid, attributes.as_ref());
        let res = self.client.get(&url).send().map_err(|e| {
            pyo3::exceptions::PyConnectionError::new_err(format!("request failed: {}", e))
        })?;
        let enabled = if res.status().is_success() {
            let body: serde_json::Value = res.json().map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("invalid response: {}", e))
            })?;
            body["enabled"].as_bool().unwrap_or(false)
        } else {
            false
        };
        {
            let mut cache = self.cache.lock().map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("cache lock: {}", e))
            })?;
            cache.insert(cache_key, (enabled, Instant::now()));
        }
        Ok(enabled)
    }
}

impl FeatureClient {
    fn build_evaluate_url(
        &self,
        flag: &str,
        user_id: &str,
        attributes: Option<&HashMap<String, String>>,
    ) -> String {
        let mut url = format!(
            "{}/evaluate?flag={}&user_id={}",
            self.base_url,
            urlencoding::encode(flag),
            urlencoding::encode(user_id)
        );
        if let Some(attrs) = attributes {
            for (k, v) in attrs {
                url.push_str("&attr=");
                url.push_str(&urlencoding::encode(&format!("{}:{}", k, v)));
            }
        }
        url
    }
}

fn cache_key(
    flag: &str,
    user_id: &str,
    attributes: Option<&HashMap<String, String>>,
) -> String {
    let mut parts = vec![flag.to_string(), user_id.to_string()];
    if let Some(attrs) = attributes {
        let mut pairs: Vec<String> = attrs.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
        pairs.sort();
        parts.extend(pairs);
    }
    parts.join("|")
}

#[pymodule]
fn featureflags_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FeatureClient>()?;
    Ok(())
}
