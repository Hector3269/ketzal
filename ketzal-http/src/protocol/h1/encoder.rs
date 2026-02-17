use crate::response::Response;

pub fn encode(response: &Response) -> Vec<u8> {
    response.to_bytes()
}
