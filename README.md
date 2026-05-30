# srvcs-cylindervolume

The cylinder-volume orchestrator of the srvcs.cloud distributed standard
library.

Its single concern: **geometry: volume of a cylinder.** It owns the *control
flow* — composing two float primitives — but does no arithmetic of its own. It
asks [`srvcs-pi`](https://github.com/srvcs/pi) for the constant `pi`, then
composes [`srvcs-floatmultiply`](https://github.com/srvcs/floatmultiply) three
times: once to square the radius, once to multiply that square by `pi`, and once
more by the height.

```
cylindervolume(radius, height):
    p    = pi()                       # the constant service, called with {}
    r2   = floatmultiply(radius, radius)
    base = floatmultiply(p, r2)       # pi * radius^2
    return floatmultiply(base, height)  # pi * radius^2 * height
```

The result is an `f64` — a JSON number that may be fractional. For example
`cylindervolume(2, 5) == 62.83185307179586` (`pi * 2^2 * 5`).

Validation is not handled here. This service never calls `srvcs-isnumber`
directly; instead its dependencies validate their own operands, and any `422`
they raise is forwarded verbatim.

## API

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/` | Service identity, concern, and dependency list |
| `POST` | `/` | Compute the volume of a cylinder of `radius` and `height` |
| `GET` | `/healthz` `/readyz` `/metrics` `/openapi.json` | srvcs service standard surface |

```sh
curl -s -X POST localhost:8080/ -H 'content-type: application/json' \
  -d '{"radius": 2, "height": 5}'
# {"radius":2,"height":5,"result":62.83185307179586}
```

Responses:

- `200 {"radius": n, "height": n, "result": n}` — evaluated; `result` is a float.
- `422` — a dependency rejected the input (forwarded verbatim).
- `500` — a reachable dependency returned a `200` without a numeric `result`
  (a contract violation).
- `503` — a dependency is unavailable.

## Dependencies

- [`srvcs-pi`](https://github.com/srvcs/pi)
- [`srvcs-floatmultiply`](https://github.com/srvcs/floatmultiply)

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `SRVCS_BIND_ADDR` | `0.0.0.0:8080` | Bind address |
| `SRVCS_PI_URL` | `http://127.0.0.1:8090` | Base URL of `srvcs-pi` |
| `SRVCS_FLOATMULTIPLY_URL` | `http://127.0.0.1:8091` | Base URL of `srvcs-floatmultiply` |
| `SRVCS_ENV` | `development` | Environment label for logs |
| `RUST_LOG` | `info,tower_http=info` | Tracing filter |

## Local checks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Orchestration tests stand up *computing* mock dependency services in-process —
`srvcs-pi` returns the real `pi` for any body and `srvcs-floatmultiply` returns
the real `a * b`, so the composition is genuinely exercised against the asserted
cases (compared approximately, since the result is a float). See
[`srvcs/platform`](https://github.com/srvcs/platform) for the shared standard.

> Note: the `cargoHash` in `flake.nix` is inherited from the template and must be
> refreshed with a `nix build` before the Nix gates pass.
