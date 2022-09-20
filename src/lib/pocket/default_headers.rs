// =======================
// DEFAULT REQUEST HEADERS
// =======================

pub fn get() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_str("application/json; charset=UTF-8").unwrap(),
    );
    headers.insert(
        "X-Accept",
        reqwest::header::HeaderValue::from_str("application/json").unwrap(),
    );
    return headers;
}
