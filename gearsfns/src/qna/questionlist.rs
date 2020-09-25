#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct QuestionList {
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum Question {
    Integer(question::Integer),
    FreeText(question::FreeText),
    TrueOrFalse(question::TrueOrFalse),
    FixedList(question::FixedList),
    ArrayOf(Box<question::ArrayOf>),
}

impl Question {
    pub fn get_id(&self) -> i32 {
        match self {
            Question::Integer(q) => q.id,
            Question::FreeText(q) => q.id,
            Question::TrueOrFalse(q) => q.id,
            Question::FixedList(q) => q.id,
            Question::ArrayOf(q) => q.id,
        }
    }
    pub fn is_required(&self) -> bool {
        match self {
            Question::Integer(q) => q.required,
            Question::FreeText(q) => q.required,
            Question::TrueOrFalse(q) => q.required,
            Question::FixedList(q) => q.required,
            Question::ArrayOf(q) => q.required,
        }
    }
}

pub mod question {

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct FreeText {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub default: Option<String>,
        pub required: bool,
        pub min_length: Option<i32>,
        pub max_length: Option<i32>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct TrueOrFalse {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub default: Option<bool>,
        pub required: bool,
        pub ui: Option<ui::TrueOrFalse>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct Integer {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub required: bool,
        pub default: Option<i32>,
        pub step: Option<i32>,
        pub min: Option<i32>,
        pub max: Option<i32>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct FixedList {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub required: bool,
        pub default: Vec<String>,
        pub items: Vec<String>,
        pub item_names: Vec<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct ArrayOf {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub required: bool,
        pub question: super::Question,
    }

    pub mod ui {
        use std::fmt;

        #[derive(Debug, Deserialize, Serialize, Clone)]
        #[serde(deny_unknown_fields)]
        pub enum TrueOrFalseWidget {
            #[serde(rename = "radio")]
            Radio,
            #[serde(rename = "select")]
            Select,
        }

        #[derive(Debug, Deserialize, Serialize, Clone)]
        #[serde(deny_unknown_fields)]
        pub struct TrueOrFalse {
            pub widget: Option<TrueOrFalseWidget>,
        }

        #[derive(Debug, Deserialize, Serialize, Clone)]
        #[serde(deny_unknown_fields)]
        pub enum IntegerWidget {
            #[serde(rename = "updown")]
            UpDown,
            #[serde(rename = "range")]
            Range,
            #[serde(rename = "radio")]
            Radio,
        }

        #[derive(Debug, Deserialize, Serialize, Clone)]
        #[serde(deny_unknown_fields)]
        pub struct Integer {
            pub widget: Option<IntegerWidget>,
        }

        impl fmt::Display for TrueOrFalseWidget {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", match self {
                    Self::Radio => "radio",
                    Self::Select => "select",
                })
            }
        }
    }
}

pub mod answer {

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct FreeText {
        pub question_id: i32,
        pub value: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct TrueOrFalse {
        pub question_id: i32,
        pub value: bool,
    }
}

pub fn sample_string_questions() -> QuestionList {
    QuestionList {
        title: "Zork".into(),
        description: "Title zoek description".into(),
        questions: vec![
            Question::FreeText(question::FreeText {
                id: 1,
                title: "What is your favourite food?".into(),
                description: "Tell be about your food preferences".into(),
                default: Some("Ice".into()),
                required: true,
                min_length: Some(0),
                max_length: Some(64),
            }),
            Question::ArrayOf(Box::new(question::ArrayOf {
                id: 8,
                title: "What is your favourite food?".into(),
                description: "Tell be about your food preferences".into(),
                required: true,
                question: Question::FreeText(question::FreeText {
                    id: 1,
                    title: "What is your favourite food?".into(),
                    description: "Tell be about your food preferences".into(),
                    default: Some("Ice".into()),
                    required: true,
                    min_length: Some(0),
                    max_length: Some(64),
                }),
            })),
            Question::Integer(question::Integer {
                id: 88,
                title: "A number".into(),
                description: "Some number".into(),
                default: Some(1),
                required: true,
                step: Some(10),
                min: Some(0),
                max: Some(100),
            }),
            Question::TrueOrFalse(question::TrueOrFalse {
                id: 222,
                title: "Some T/F 1".into(),
                description: "Some T/f".into(),
                default: Some(false),
                required: true,
                ui: None,
            }),
            Question::FixedList(question::FixedList {
                id: 24,
                title: "Zork and or Bork?".into(),
                description: "Some T/f".into(),
                default: vec![],
                required: true,
                items: vec!["zork".to_string(), "bork".to_string()],
                item_names: vec!["Zork".to_string(), "Bork".to_string()],
            }),
            Question::FreeText(question::FreeText {
                id: 3,
                title: "Some title 2".into(),
                description: "Some 2 desc".into(),
                default: None,
                required: true,
                min_length: Some(0),
                max_length: Some(64),
            }),
            Question::TrueOrFalse(question::TrueOrFalse {
                id: 899,
                title: "I have read the terms and conditions".into(),
                description: "Some T/f".into(),
                default: Some(false),
                required: true,
                ui: Some(question::ui::TrueOrFalse {
                    widget: Some(question::ui::TrueOrFalseWidget::Radio),
                }),
            }),
        ],
    }
}
