use jsonschema::{Draft, JSONSchema};
use lambda_http::{handler, lambda, Context, IntoResponse, Request};
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

async fn myhandler(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    match serde_json::from_slice::<Value>(req.body().as_ref()) {
        Ok(val) => {
            let question = serde_json::to_value(&build_sample()).unwrap();
            let compiled = JSONSchema::compile(&question, Some(Draft::Draft7)).unwrap();
            let result = compiled.validate(&val);

            if let Err(errors) = result {
                for error in errors {
                    println!("Validation error: {}", error)
                }
                Ok(json!({"status":"ok"}))
            } else {
                Ok(json!({"status":"error"}))
            }
        }
        Err(_e) => Ok(json!({"status":"error"})),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn _handles() {
        let request = Request::default();
        // let expected = to_value(build_sample()).unwrap().into_response();
        let expected = json!({
            "status": "error"
        })
        .into_response();
        let response = myhandler(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
