#![cfg(test)]

use std::{collections::HashMap, fs};

use nom::error::VerboseError;

use crate::data::{Children, Element, Value};

macro_rules! parser_test {
    (string, $name:ident, $content:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(
                super::document::<VerboseError<&str>>($content).unwrap().1,
                $expected
            )
        }
    };
    (file, $name:ident, $filename:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(
                super::document::<VerboseError<&str>>(
                    fs::read_to_string($filename).unwrap().as_str()
                )
                .unwrap()
                .1,
                $expected
            )
        }
    };
}

parser_test!(
    file,
    test_example_html,
    "examples/html.mml",
    Element {
        name: "html".to_string(),
        attributes: HashMap::new(),
        children: Children::Elements(vec![
            Element {
                name: "head".to_string(),
                attributes: HashMap::new(),
                children: Children::Elements(vec![
                    Element {
                        name: "title".to_string(),
                        attributes: HashMap::new(),
                        children: Children::String("Example HTML in MML".to_string())
                    },
                    Element {
                        name: "meta".to_string(),
                        attributes: HashMap::from([(
                            "charset".to_string(),
                            Value::String("utf-8".to_string())
                        )]),
                        children: Children::Elements(vec![])
                    }
                ])
            },
            Element {
                name: "body".to_string(),
                attributes: HashMap::new(),
                children: Children::Elements(vec![Element {
                    name: "p".to_string(),
                    attributes: HashMap::from([(
                        "id".to_string(),
                        Value::String("content".to_string())
                    )]),
                    children: Children::String("A paragraph".to_string())
                }])
            }
        ])
    }
);
