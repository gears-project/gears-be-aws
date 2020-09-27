use jsonschema::{Draft, JSONSchema};
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, Response};
use serde_json::{json, Value};

use gearsfn::qna::{questiondto, questionlist};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

fn build_sample() -> questiondto::Node {
    let q = questionlist::sample_string_questions();
    questiondto::Node::Object(q.into())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(myhandler)).await?;
    Ok(())
}

struct ApiResponse {
    status: u16,
    body: Value,
}

impl ApiResponse {
    pub fn ok() -> Self {
        Self::default()
    }
    pub fn error() -> Self {
        Self {
            status: 400,
            body: json!({
                "message": "bad input",
            }),
        }
    }
}

impl Default for ApiResponse {
    fn default() -> Self {
        Self {
            status: 200,
            body: json!({}),
        }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.status)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Credentials", "true")
            .body(Body::Text(serde_json::to_string(&self.body).unwrap()))
            .expect("err creating response")
    }
}

async fn myhandler(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    if let Ok(val) = serde_json::from_slice::<Value>(req.body().as_ref()) {
        let question = serde_json::to_value(&build_sample()).unwrap();
        let compiled = JSONSchema::compile(&question, Some(Draft::Draft7)).unwrap();
        let result = compiled.validate(&val);

        if let Err(errors) = result {
            let mut errorlist = Vec::<String>::new();
            for error in errors {
                println!("Validation error: {}", error);
                errorlist.push(format!("{}", error));
            }
            Ok(ApiResponse {
                status: 400,
                body: json!({
                   "message": "input does not validate",
                    "errors": errorlist,
                }),
            })
        } else {
            Ok(ApiResponse::ok())
        }
    } else {
        Ok(ApiResponse::error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn _handles_empty_request() {
        let request = Request::default();
        let expected = json!({
            "message": "bad input"
        })
        .into_response();
        let response = myhandler(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
