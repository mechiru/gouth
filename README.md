# gouth

[![ci](https://github.com/mechiru/gouth/workflows/ci/badge.svg)](https://github.com/mechiru/gouth/actions?query=workflow:ci)
[![Rust Documentation](https://docs.rs/gouth/badge.svg)](https://docs.rs/gouth)
[![Latest Version](https://img.shields.io/crates/v/gouth.svg)](https://crates.io/crates/gouth)

This library provides auto-renewed tokens for GCP service authentication.

## Notes
| Authentication flow                  | Status                              |
|--------------------------------------|-------------------------------------|
| API key                              | Not supported / No plans to support |
| OAuth 2.0 client                     | Supported                           |
| Environment-provided service account | Supported                           |
| Service account key                  | Supported                           |

## Example

### Default
- Scope is `https://www.googleapis.com/auth/cloud-platform`
- Looks for credentials in the following places, preferring the first location found:
  - A JSON file whose path is specified by the `GOOGLE_APPLICATION_CREDENTIALS` environment variable.
  - A JSON file in a location known to the gcloud command-line tool.
  - On Google Compute Engine, it fetches credentials from the metadata server.

```rust
use gouth::Token;

let token = Token::new().unwrap();
println!("authorization: {}", token.header_value().unwrap());
```

### Custom
scope:
```rust
use gouth::Builder;

let token = Builder::new()
	.scopes(&["https://www.googleapis.com/auth/bigquery"])
	.build()
	.unwrap();
println!("authorization: {}", token.header_value().unwrap());
```

json:
```rust
use gouth::Builder;

let token = Builder::new().json("credentials-data").build().unwrap();
println!("authorization: {}", token.header_value().unwrap());
```

file:
```rust
use gouth::Builder;

let token = Builder::new().file("credentials-path").build().unwrap();
println!("authorization: {}", token.header_value().unwrap());
```

## License
Licensed under [MIT license](./LICENSE).
