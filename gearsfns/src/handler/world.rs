use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::to_value;

use gearsfn::qna::{questiondto, questionlist};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

fn build_sample() -> questiondto::Node {
    let q = questionlist::sample_string_questions();
    questiondto::Node::Object(q.into())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(world)).await?;
    Ok(())
}

async fn world(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(to_value(build_sample())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn world_handles() {
        let request = Request::default();
        let expected = to_value(build_sample()).unwrap().into_response();
        let response = world(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
