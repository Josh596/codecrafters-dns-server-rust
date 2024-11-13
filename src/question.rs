use crate::{header::DNSHeader, utils};

pub struct DNSQuestion {
    domain_name: String,
    type_: u16,
    class: u16,
}

impl DNSQuestion {
    pub fn new() -> Self {
        Self {
            domain_name: String::from("codecrafters.io"),
            type_: 1,
            class: 1,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        // Encode domain name
        let mut buffer = Vec::new();
        let labels = utils::encode_domain_name(&self.domain_name);
        buffer.extend_from_slice(&labels);

        // Type byte
        buffer.extend_from_slice(&(self.type_ as u16).to_be_bytes());

        // Class byte
        buffer.extend_from_slice(&(self.class as u16).to_be_bytes());

        buffer
    }
}
