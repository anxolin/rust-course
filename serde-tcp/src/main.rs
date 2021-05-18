use serde_tcp::{Answer, Point};

fn main() {
    // Serialize
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("Point: {:?}", point);
    println!("Serialized: {}", serialized);

    // Deserialize
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // Serialize
    let answers = vec![
        Answer::Yes,
        Answer::Yes,
        Answer::No,
        Answer::Maybe("i'm not sure".to_string()),
        Answer::Maybe("depends".to_string()),
    ];
    let serialized = serde_json::to_string(&answers).unwrap();
    println!("Point: {:?}", point);
    println!("Serialized: {}", serialized);

    // Deserialize
    let deserialized: Vec<Answer> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
