wit_bindgen::generate!({ generate_all });
use wasmcloud::postgres::query::query;
use wasmcloud::postgres::types::PgValue;
use wasmcloud_component::http;


struct Component;

http::export!(Component);

const SELECT_QUERY: &str = r#"
SELECT description FROM example WHERE id = $1;
"#;

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let (parts, _body) = _request.into_parts();
        let query_string = parts
            .uri
            .query()
            .map(ToString::to_string)
            .unwrap_or_default();
        let count_str = match query_string.split("=").collect::<Vec<&str>>()[..] {
            ["count", count] => count,
            _ => "0", // Default value if count is not provided or invalid
        };
        let mut count: i32 = count_str.parse().unwrap_or(0); // Parse as integer, default to 0 on error
        if count > 5 {
            count = 1;
        }

        match query(SELECT_QUERY, &[PgValue::Integer(count)]) {
            Ok(rows) => {
                if let Some(row) = rows.first() {
                    if let Some(entry) = row.get(0) {
                        // Access the `Value` field of `ResultRowEntry`
                        if let PgValue::Text(description) = &entry.value {
                            return Ok(http::Response::new(format!("Description: {}\n", description)));
                        }
                    }
                }
                Ok(http::Response::new("Unexpected response.\n".to_string()))
            }
            Err(e) => Ok(http::Response::new(format!("ERROR: failed to retrieve inserted row: {e}"))),
        }
    }
}
