use lambda::{handler_fn, Context};

use gearsfn::qna::{questiondto, questionlist};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(taker)).await?;
    Ok(())
}

/*
#[derive(Deserialize, Debug, Clone, PartialEq)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Debug, PartialEq)]
struct CustomOutput {
    message: String,
}
*/

fn build_sample() -> questiondto::Node {
    let q = questionlist::sample_string_questions();
    questiondto::Node::Object(q.into())
}

async fn taker(_e: (), _c: Context) -> Result<questiondto::Node, Error> {
    Ok(build_sample())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn taker_handles() {
        assert_eq!(
            taker((), Context::default())
                .await
                .expect("expected Ok(_) value"),
            build_sample()
        )
    }
}
