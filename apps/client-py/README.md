# FeatureFlags Python Client

Python client for the FeatureFlags server (PyO3 native extension).

## Install

From the repo root:

```bash
pip install -e apps/client-py
```

Requires [maturin](https://www.maturin.rs/) as the build backend (installed automatically by pip).

## Usage

```python
from featureflags_client import FeatureClient

client = FeatureClient("http://localhost:8080")
if client.is_enabled("new_checkout", user_id="123", attributes={"country": "ES"}):
    show_new_checkout()
```

- `FeatureClient(server_url, cache_ttl_secs=60)` — optional cache TTL in seconds (default 60).
- `is_enabled(flag_name, user_id=None, attributes=None)` — returns `True` if the flag is enabled for the given context. Results are cached for `cache_ttl_secs`.

## Requirements

- Python 3.8+
- Rust toolchain (for building the extension)
