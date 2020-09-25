use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum Node {
    #[serde(rename = "string")]
    String(StringNode),
    #[serde(rename = "boolean")]
    Boolean(BooleanNode),
    #[serde(rename = "object")]
    Object(ObjectNode),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub type ObjectNode = HashMap<String, Node>;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct StringNode {
    pub title: String,
    pub description: Option<String>,
    pub default: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BooleanNode {
    pub title: String,
    pub description: Option<String>,
    pub default: Option<bool>,
}

impl From<questionlist::QuestionList> for ObjectNode {
    fn from(question_list: questionlist::QuestionList) -> Self {
        let questions = question_list.questions;

        let req: Vec<String> = questions
            .iter()
            .filter(|e| e.is_required())
            .map(|e| e.get_id().to_string() )
            .collect();

        let mut props = HashMap::<String, Node>::new();
        for q in questions {
            props.insert(q.get_id().to_string(), q.into());
        }
        ObjectNode {
            title: question_list.title,
            description: question_list.description,
            properties: props,
            additional_properties: false,
            required: req,
        }
    }
}

impl From<questionlist::Question> for Node {
    fn from(question: questionlist::Question) -> Self {
         match question {
            questionlist::Question::FreeText(q) => {
                Node::String(StringNode {
                    title: q.title,
                    description: q.description,
                    default: q.default,
                })
            }
            questionlist::Question::TrueOrFalse(q) => {
                Node::Boolean(BooleanNode {
                    title: q.title,
                    description: q.description,
                    default: q.default,
                })

            }
        }
    }
}

