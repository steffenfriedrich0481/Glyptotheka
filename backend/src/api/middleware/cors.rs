use axum::{
    http::{header, HeaderValue, Method, Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn cors_middleware(request: Request<axum::body::Body>, next: Next) -> Response {
    // Extract origin before consuming request
    let origin_header = request.headers().get(header::ORIGIN).cloned();
    let origin = origin_header
        .and_then(|v| v.to_str().ok().map(String::from))
        .filter(|o| o.starts_with("http://localhost:"))
        .unwrap_or_else(|| "http://localhost:5173".to_string());

    // Handle preflight OPTIONS requests
    if request.method() == Method::OPTIONS {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.as_str())
            .header(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                "GET, POST, PUT, DELETE, OPTIONS",
            )
            .header(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                "Content-Type, Authorization",
            )
            .header(header::ACCESS_CONTROL_MAX_AGE, "3600")
            .body(axum::body::Body::empty())
            .unwrap();
    }

    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_str(&origin).unwrap(),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type, Authorization"),
    );
    headers.insert(
        header::ACCESS_CONTROL_MAX_AGE,
        HeaderValue::from_static("3600"),
    );

    response
}
