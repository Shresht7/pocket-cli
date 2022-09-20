// =======================
// DEFAULT REQUEST HEADERS
// =======================

/// Returns the default_headers
pub fn get() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

    //  Content-Type: application/json; charset=UTF-8
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_str("application/json; charset=UTF-8").unwrap(),
    );

    //  X-Accept: application/json
    headers.insert(
        "X-Accept",
        reqwest::header::HeaderValue::from_str("application/json").unwrap(),
    );

    return headers;
}
