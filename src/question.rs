use crate::utils;

pub struct DNSQuestion {
    pub domain_name: String,
    pub type_: u16,
    pub class: u16,
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

impl From<&[u8]> for DNSQuestion {
    fn from(data: &[u8]) -> Self {
        let domain_name = utils::decode_domain_name_label_sequence(data);

        let type_ = data[domain_name.len()] as u16;
        let class = data[domain_name.len() + 1] as u16;

        Self {
            domain_name,
            type_,
            class,
        }
    }
}
