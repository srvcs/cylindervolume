# srvcs-cylindervolume

## Name

| Field | Value |
| --- | --- |
| Service | `srvcs-cylindervolume` |
| Slug | `cylindervolume` |
| Repository | `srvcs/cylindervolume` |
| Package | `srvcs-cylindervolume` |
| Kind | `orchestrator` |

## Function

geometry: volume of a cylinder

## Dependencies

| Dependency | Repository |
| --- | --- |
| `srvcs-pi` | [srvcs/pi](https://github.com/srvcs/pi) |
| `srvcs-floatmultiply` | [srvcs/floatmultiply](https://github.com/srvcs/floatmultiply) |

## API

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/` | Service identity |
| `POST` | `/` | Evaluate the service function |
| `GET` | `/healthz` | Liveness probe |
| `GET` | `/readyz` | Readiness probe |
| `GET` | `/metrics` | Prometheus metrics |
| `GET` | `/openapi.json` | OpenAPI document |

## Inputs

| Name | Type | Required |
| --- | --- | --- |
| `radius` | `json` | yes |
| `height` | `json` | yes |

## Outputs

| Name | Type |
| --- | --- |
| `radius` | `json` |
| `height` | `json` |
| `result` | `number` |

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `SRVCS_BIND_ADDR` | `0.0.0.0:8080` | Bind address |
| `SRVCS_ENV` | `development` | Environment label for logs |
| `RUST_LOG` | `info,tower_http=info` | Tracing filter |
| `SRVCS_FLOATMULTIPLY_URL` | `` | Base URL for srvcs-floatmultiply |
| `SRVCS_PI_URL` | `http://127.0.0.1:8090` | Base URL for srvcs-pi |

## Error Behavior

- `422` means the request could not be evaluated for the documented input shape.
- `503` means a required dependency was unavailable or returned an unexpected response.
- Dependency validation errors are forwarded when this service delegates validation.

## Local Checks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

See the [srvcs service standard](https://github.com/srvcs/platform/blob/main/STANDARD.md) for the full operational contract.

## Metadata

Machine-readable service metadata lives in `srvcs.yaml`. Keep it aligned with this README when the service contract changes.
