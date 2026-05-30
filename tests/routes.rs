use axum::body::Body;
use axum::extract::Json as AxumJson;
use axum::http::{Request, StatusCode};
use axum::routing::post;
use axum::{Json, Router as AxumRouter};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use srvcs_cylindervolume::{api::Deps, health, router, telemetry};
use tower::ServiceExt;

const DEAD_URL: &str = "http://127.0.0.1:1";

// --- Computing mocks for every srvcs primitive this family composes over.
//
// Each reads its operands from the request body and returns the *real* answer,
// so the orchestration is genuinely exercised rather than fed a canned value.
// cylindervolume only calls `srvcs-pi` and `srvcs-floatmultiply`; the rest are
// provided for completeness of the family's contract.

/// `srvcs-pi`: returns the real `pi` for *any* request body (a constant
/// service called with an empty body).
async fn spawn_pi() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|| async { Json(json!({ "result": std::f64::consts::PI })) }),
    );
    serve(app).await
}

/// `srvcs-floatadd`: reads `{a, b}` -> `{"result": a + b}` (as f64).
#[allow(dead_code)]
async fn spawn_floatadd() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let a = body.get("a").and_then(Value::as_f64).unwrap_or(0.0);
            let b = body.get("b").and_then(Value::as_f64).unwrap_or(0.0);
            Json(json!({ "result": a + b }))
        }),
    );
    serve(app).await
}

/// `srvcs-floatsubtract`: reads `{a, b}` -> `{"result": a - b}` (as f64).
#[allow(dead_code)]
async fn spawn_floatsubtract() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let a = body.get("a").and_then(Value::as_f64).unwrap_or(0.0);
            let b = body.get("b").and_then(Value::as_f64).unwrap_or(0.0);
            Json(json!({ "result": a - b }))
        }),
    );
    serve(app).await
}

/// `srvcs-floatmultiply`: reads `{a, b}` -> `{"result": a * b}` (as f64).
async fn spawn_floatmultiply() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let a = body.get("a").and_then(Value::as_f64).unwrap_or(0.0);
            let b = body.get("b").and_then(Value::as_f64).unwrap_or(0.0);
            Json(json!({ "result": a * b }))
        }),
    );
    serve(app).await
}

/// `srvcs-floatdivide`: reads `{a, b}` -> `{"result": a / b}` (as f64).
#[allow(dead_code)]
async fn spawn_floatdivide() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let a = body.get("a").and_then(Value::as_f64).unwrap_or(0.0);
            let b = body.get("b").and_then(Value::as_f64).unwrap_or(1.0);
            Json(json!({ "result": a / b }))
        }),
    );
    serve(app).await
}

/// `srvcs-sqrt`: reads `{value}` -> `{"result": sqrt(value)}` (as f64).
#[allow(dead_code)]
async fn spawn_sqrt() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let value = body.get("value").and_then(Value::as_f64).unwrap_or(0.0);
            Json(json!({ "result": value.sqrt() }))
        }),
    );
    serve(app).await
}

/// `srvcs-sin`: reads `{value}` -> `{"result": sin(value)}` (as f64).
#[allow(dead_code)]
async fn spawn_sin() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let value = body.get("value").and_then(Value::as_f64).unwrap_or(0.0);
            Json(json!({ "result": value.sin() }))
        }),
    );
    serve(app).await
}

/// `srvcs-cos`: reads `{value}` -> `{"result": cos(value)}` (as f64).
#[allow(dead_code)]
async fn spawn_cos() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let value = body.get("value").and_then(Value::as_f64).unwrap_or(0.0);
            Json(json!({ "result": value.cos() }))
        }),
    );
    serve(app).await
}

/// `srvcs-tan`: reads `{value}` -> `{"result": tan(value)}` (as f64).
#[allow(dead_code)]
async fn spawn_tan() -> String {
    let app = AxumRouter::new().route(
        "/",
        post(|AxumJson(body): AxumJson<Value>| async move {
            let value = body.get("value").and_then(Value::as_f64).unwrap_or(0.0);
            Json(json!({ "result": value.tan() }))
        }),
    );
    serve(app).await
}

/// Spawn a mock returning a fixed status + body (used for error-path tests).
async fn spawn_fixed(status: StatusCode, body: Value) -> String {
    let app = AxumRouter::new().route(
        "/",
        post(move || {
            let body = body.clone();
            async move { (status, Json(body)) }
        }),
    );
    serve(app).await
}

async fn serve(app: AxumRouter) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{addr}")
}

fn app(pi_url: &str, floatmultiply_url: &str) -> axum::Router {
    router(
        telemetry::metrics_handle_for_tests(),
        Deps {
            pi_url: pi_url.to_string(),
            floatmultiply_url: floatmultiply_url.to_string(),
        },
    )
}

async fn cylindervolume(
    pi_url: &str,
    floatmultiply_url: &str,
    radius: Value,
    height: Value,
) -> (StatusCode, Value) {
    let res = app(pi_url, floatmultiply_url)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "radius": radius, "height": height }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

async fn status_of(uri: &str) -> StatusCode {
    app(DEAD_URL, DEAD_URL)
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
        .status()
}

fn result_f64(body: &Value) -> f64 {
    body["result"].as_f64().expect("result is a JSON number")
}

// --- Standard endpoints. ---

#[tokio::test]
async fn healthz_ok() {
    assert_eq!(status_of("/healthz").await, StatusCode::OK);
}

#[tokio::test]
async fn readyz_reflects_state() {
    health::set_ready(true);
    assert_eq!(status_of("/readyz").await, StatusCode::OK);
}

#[tokio::test]
async fn metrics_ok() {
    assert_eq!(status_of("/metrics").await, StatusCode::OK);
}

#[tokio::test]
async fn openapi_ok() {
    assert_eq!(status_of("/openapi.json").await, StatusCode::OK);
}

#[tokio::test]
async fn generates_request_id_when_absent() {
    let res = app(DEAD_URL, DEAD_URL)
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        res.headers().contains_key("x-request-id"),
        "response must carry a generated x-request-id"
    );
}

#[tokio::test]
async fn index_reports_identity() {
    let res = app(DEAD_URL, DEAD_URL)
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["service"], "srvcs-cylindervolume");
    assert_eq!(body["concern"], "geometry: volume of a cylinder");
    assert_eq!(
        body["depends_on"],
        json!(["srvcs-pi", "srvcs-floatmultiply"])
    );
}

// --- Correctness cases, against the computing mocks. ---

#[tokio::test]
async fn cylindervolume_2_5_is_pi_times_20() {
    let (pi, m) = (spawn_pi().await, spawn_floatmultiply().await);
    let (status, body) = cylindervolume(&pi, &m, json!(2), json!(5)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["radius"], json!(2));
    assert_eq!(body["height"], json!(5));
    // pi * 2^2 * 5 = 62.83185307179586
    assert!((result_f64(&body) - 62.83185307179586).abs() < 1e-9);
}

#[tokio::test]
async fn cylindervolume_unit_is_pi() {
    let (pi, m) = (spawn_pi().await, spawn_floatmultiply().await);
    let (status, body) = cylindervolume(&pi, &m, json!(1), json!(1)).await;
    assert_eq!(status, StatusCode::OK);
    // pi * 1^2 * 1 = pi
    assert!((result_f64(&body) - std::f64::consts::PI).abs() < 1e-9);
}

#[tokio::test]
async fn cylindervolume_zero_radius_is_0() {
    let (pi, m) = (spawn_pi().await, spawn_floatmultiply().await);
    let (status, body) = cylindervolume(&pi, &m, json!(0), json!(10)).await;
    assert_eq!(status, StatusCode::OK);
    assert!((result_f64(&body) - 0.0).abs() < 1e-9);
}

#[tokio::test]
async fn cylindervolume_zero_height_is_0() {
    let (pi, m) = (spawn_pi().await, spawn_floatmultiply().await);
    let (status, body) = cylindervolume(&pi, &m, json!(7), json!(0)).await;
    assert_eq!(status, StatusCode::OK);
    assert!((result_f64(&body) - 0.0).abs() < 1e-9);
}

#[tokio::test]
async fn cylindervolume_fractional_inputs() {
    let (pi, m) = (spawn_pi().await, spawn_floatmultiply().await);
    let (status, body) = cylindervolume(&pi, &m, json!(2.5), json!(3.5)).await;
    assert_eq!(status, StatusCode::OK);
    // pi * 2.5^2 * 3.5 = pi * 6.25 * 3.5
    assert!((result_f64(&body) - std::f64::consts::PI * 6.25 * 3.5).abs() < 1e-9);
}

#[tokio::test]
async fn cylindervolume_3_10() {
    let (pi, m) = (spawn_pi().await, spawn_floatmultiply().await);
    let (status, body) = cylindervolume(&pi, &m, json!(3), json!(10)).await;
    assert_eq!(status, StatusCode::OK);
    // pi * 9 * 10 = pi * 90
    assert!((result_f64(&body) - std::f64::consts::PI * 90.0).abs() < 1e-9);
}

// --- Error / edge cases. ---

#[tokio::test]
async fn degrades_when_pi_unreachable() {
    let m = spawn_floatmultiply().await;
    let (status, body) = cylindervolume(DEAD_URL, &m, json!(2), json!(5)).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["dependency"], "srvcs-pi");
}

#[tokio::test]
async fn degrades_when_floatmultiply_unreachable() {
    // pi is reachable so the pipeline reaches the floatmultiply call, which
    // then degrades.
    let pi = spawn_pi().await;
    let (status, body) = cylindervolume(&pi, DEAD_URL, json!(2), json!(5)).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["dependency"], "srvcs-floatmultiply");
}

#[tokio::test]
async fn forwards_422_from_floatmultiply() {
    let pi = spawn_pi().await;
    let m = spawn_fixed(
        StatusCode::UNPROCESSABLE_ENTITY,
        json!({ "error": "value is not a number" }),
    )
    .await;
    let (status, body) = cylindervolume(&pi, &m, json!("nope"), json!(5)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"], "value is not a number");
}

#[tokio::test]
async fn malformed_pi_result_is_500() {
    let m = spawn_floatmultiply().await;
    let pi = spawn_fixed(StatusCode::OK, json!({ "result": "not-a-number" })).await;
    let (status, body) = cylindervolume(&pi, &m, json!(2), json!(5)).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body["dependency"], "srvcs-pi");
}

#[tokio::test]
async fn malformed_floatmultiply_result_is_500() {
    let pi = spawn_pi().await;
    let m = spawn_fixed(StatusCode::OK, json!({ "result": "not-a-number" })).await;
    let (status, body) = cylindervolume(&pi, &m, json!(2), json!(5)).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body["dependency"], "srvcs-floatmultiply");
}
