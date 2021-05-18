use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    #[serde(rename = "val_x")]
    pub x: i32,

    #[serde(rename = "val_y")]
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Answer {
    Yes,
    No,
    Maybe(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let point = Point { x: 1, y: 2 };
        let serialized = serde_json::to_string(&point).unwrap();

        assert_eq!(serialized, r#"{"val_x":1,"val_y":2}"#)
    }

    #[test]
    fn deserialize() {
        let json = r#"{"val_x":1,"val_y":2}"#;
        let deserialized: Point = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized, Point { x: 1, y: 2 });
    }
}
