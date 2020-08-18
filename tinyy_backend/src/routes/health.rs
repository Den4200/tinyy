use rocket_contrib::json::JsonValue;

#[get("/health")]
pub fn health() -> JsonValue {
    json!({
        "status": "ok"
    })
}
