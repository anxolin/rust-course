use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    SET(Payload),
    GET,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    // pub payload: Payload,
    OK,
    Result(Option<Payload>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub data: String,
    pub count: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
