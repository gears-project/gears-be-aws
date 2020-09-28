use jsonschema::{Draft, JSONSchema};
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, Response};
use serde_json::{json, to_value, Value};

use gearsfn::qna::{questiondto, questionlist};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

fn build_sample() -> questiondto::Node {
    let q = questionlist::sample_string_questions();
    questiondto::Node::Object(q.into())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(router)).await?;
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

async fn router(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    match req.method().as_str() {
        "POST" => Ok(post_answer(req)),
        "GET" => Ok(get_question(req)),
        _ => Ok(ApiResponse {
            status: 405,
            body: json!({}),
        }),
    }
}

fn get_question(_: Request) -> ApiResponse {
    ApiResponse {
        status: 200,
        body: to_value(build_sample()).unwrap(),
    }
}

fn post_answer(req: Request) -> ApiResponse {
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
            ApiResponse {
                status: 400,
                body: json!({
                   "message": "input does not validate",
                    "errors": errorlist,
                }),
            }
        } else {
            ApiResponse::ok()
        }
    } else {
        ApiResponse::error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn post_answer_handles_empty_request() {
        let request = Request::default();
        let expected = json!(build_sample()).into_response();
        let response = router(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }

    #[tokio::test]
    async fn get_question_handles() {
        let request = Request::default();
        // let expected = to_value(build_sample()).unwrap().into_response();
        let expected = json!(build_sample()).into_response();
        let response = router(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
