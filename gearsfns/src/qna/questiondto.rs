use std::collections::HashMap;

use super::questionlist;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Node {
    #[serde(rename = "integer")]
    Integer(IntegerNode),
    #[serde(rename = "string")]
    String(StringNode),
    #[serde(rename = "boolean")]
    Boolean(BooleanNode),
    #[serde(rename = "string")]
    FixedList(FixedListNode),
    #[serde(rename = "object")]
    Object(ObjectNode),
    #[serde(rename = "array")]
    Array(ArrayNode),
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectNode {
    pub title: String,
    pub description: String,
    pub properties: HashMap<String, Node>,
    pub additional_properties: bool,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct StringNode {
    pub title: String,
    pub description: String,
    pub default: Option<String>,
}
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct BooleanNode {
    pub title: String,
    pub description: String,
    pub default: Option<bool>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntegerNode {
    pub title: String,
    pub description: String,
    pub default: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct FixedListNode {
    pub title: String,
    pub description: String,
    pub default: Vec<String>,
    #[serde(rename = "enum")]
    pub items: Vec<String>,
    #[serde(rename = "enumNames")]
    pub item_names: Vec<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ArrayNode {
    pub title: String,
    pub description: String,
    pub items: Box<Node>,
}

impl From<questionlist::QuestionList> for ObjectNode {
    fn from(question_list: questionlist::QuestionList) -> Self {
        let questions = question_list.questions;

        let req: Vec<String> = questions
            .iter()
            .filter(|e| e.is_required())
            .map(|e| e.get_id().to_string())
            .collect();

        let props: HashMap<String, Node> = questions
            .iter()
            .map(|q| (q.get_id().to_string(), q.clone().into()))
            .collect();

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
            questionlist::Question::Integer(q) => Node::Integer(IntegerNode {
                title: q.title,
                description: q.description,
                default: q.default,
                multiple_of: q.step,
                minimum: q.min,
                maximum: q.max,
            }),
            questionlist::Question::FreeText(q) => Node::String(StringNode {
                title: q.title,
                description: q.description,
                default: q.default,
            }),
            questionlist::Question::TrueOrFalse(q) => Node::Boolean(BooleanNode {
                title: q.title,
                description: q.description,
                default: q.default,
            }),
            questionlist::Question::FixedList(q) => Node::FixedList(FixedListNode {
                title: q.title,
                description: q.description,
                default: q.default,
                items: q.items,
                item_names: q.item_names,
            }),
            questionlist::Question::ArrayOf(q) => {
                let arr = q.as_ref().clone();
                Node::Array(ArrayNode {
                    title: arr.title,
                    description: arr.description,
                    items: Box::new(arr.question.into()),
                })
            }
        }
    }
}

pub mod ui_schema {
    use super::super::questionlist;
    use super::super::questionlist::question::ui;

    use std::collections::HashMap;

    pub type UiObject = HashMap<String, UiNode>;

    #[derive(Debug, Serialize, Clone, PartialEq)]
    pub struct UiNode {
        #[serde(skip_serializing_if = "Option::is_none")]
        placeholder: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ui:widget")]
        ui_widget: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "ui:options")]
        ui_options: Option<HashMap<String, String>>,
    }

    impl UiNode {
        pub fn empty() -> Self {
            Self {
                placeholder: None,
                ui_options: None,
                ui_widget: None,
            }
        }
    }

    impl From<ui::TrueOrFalse> for UiNode {
        fn from(ui: ui::TrueOrFalse) -> Self {
            UiNode {
                placeholder: None,
                ui_options: None,
                ui_widget: if let Some(widget) = ui.widget {
                    Some(widget.to_string())
                } else {
                    None
                },
            }
        }
    }
    impl From<questionlist::QuestionList> for UiObject {
        fn from(question_list: questionlist::QuestionList) -> Self {
            question_list
                .questions
                .iter()
                .map(|q| (q.get_id().to_string(), q.clone().into()))
                .collect()
        }
    }

    impl From<questionlist::Question> for UiNode {
        fn from(question: questionlist::Question) -> Self {
            match question {
                questionlist::Question::TrueOrFalse(q) => {
                    if let Some(ui) = q.ui {
                        UiNode {
                            placeholder: None,
                            ui_options: None,
                            ui_widget: if let Some(widget) = ui.widget {
                                Some(widget.to_string())
                            } else {
                                None
                            },
                        }
                    } else {
                        UiNode::empty()
                    }
                }
                _ => UiNode::empty(),
            }
        }
    }
}
