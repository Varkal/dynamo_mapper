use rusoto_dynamodb::{AttributeValue};
use std::collections::HashMap;

extern crate dynamo_mapper_macro;
pub use dynamo_mapper_macro::*;

pub type DynamoItem = HashMap<String, AttributeValue>;

pub trait DynamoMapper {
    fn to_dynamo(&self) -> DynamoItem;
    fn from_dynamo(item: &DynamoItem) -> Self;
}

pub trait DynamoAttribute {
    fn to_dynamo(&self) -> Option<AttributeValue>;
    fn from_dynamo(item: &DynamoItem, key: String) -> Option<Self> where Self: std::marker::Sized;
}

impl DynamoAttribute for String {
    fn to_dynamo(&self) -> Option<AttributeValue> {
        if self.to_string() != String::new() {
            Some(AttributeValue {
                s: Some(self.to_string()),
                ..Default::default()
            })
        } else {
            None
        }
    }

    fn from_dynamo(item: &DynamoItem, key: String) -> Option<String>{
        if let Some(attribute_value) = item.get(&key) {
            if let Some(string) = &attribute_value.s {
                return Some(string.to_string());
            }
        };

        return None;
    }
}

impl DynamoAttribute for u128 {
    fn to_dynamo(&self) -> Option<AttributeValue> {
        Some(AttributeValue {
            n: Some(self.to_string()),
            ..Default::default()
        })
    }

    fn from_dynamo(item: &DynamoItem, key: String) -> Option<u128>{
        if let Some(attribute_value) = item.get(&key) {
            if let Some(string_value) = &attribute_value.n {
                let int_value: u128 = string_value.parse().unwrap();
                return Some(int_value);
            };
        };

        return None;
    }
}

impl DynamoAttribute for bool {
    fn to_dynamo(&self) -> Option<AttributeValue> {
        Some(AttributeValue {
            bool: Some(*self),
            ..Default::default()
        })
    }

    fn from_dynamo(item: &DynamoItem, key: String) -> Option<bool>{
        if let Some(attribute_value) = item.get(&key) {
            return attribute_value.bool;
        };

        return None;
    }
}

#[cfg(feature = "uuid")]
mod dynamo_mapper_uuid {
    use uuid::Uuid;
    use crate::{DynamoAttribute, DynamoItem};
    use rusoto_dynamodb::{AttributeValue};

    impl DynamoAttribute for Uuid {
        fn to_dynamo(&self) -> Option<AttributeValue> {
            self.to_string().to_dynamo()
        }

        fn from_dynamo(item: &DynamoItem, key: String) -> Option<Uuid>{
            if let Some(string_value) = String::from_dynamo(item, key){
                if let Ok(uuid) = Uuid::parse_str(string_value.as_str()) {
                    return Some(uuid);
                }
            }

            return None;
        }
    }
}
