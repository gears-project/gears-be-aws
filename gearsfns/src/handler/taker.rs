use lambda::{handler_fn, Context};

#[macro_use]
extern crate serde;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(taker)).await?;
    Ok(())
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Debug, PartialEq)]
struct CustomOutput {
    message: String,
}

async fn taker(e: CustomEvent, _c: Context) -> Result<CustomOutput, Error> {
    let res = format!("oi, oi, oi {}", e.first_name);
    Ok(CustomOutput { message: res })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn taker_handles() {
        let event = CustomEvent {
            first_name: "42".to_string(),
        };
        assert_eq!(
            taker(event.clone(), Context::default())
                .await
                .expect("expected Ok(_) value"),
            CustomOutput {
                message: "oi, oi, oi 42".to_string(),
            }
        )
    }
}
