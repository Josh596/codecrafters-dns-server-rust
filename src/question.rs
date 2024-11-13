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

    fn encode_domain_name(domain_name: &str) -> Vec<u8> {
        // Split based on "."
        let splits: Vec<&str> = domain_name.split(".").collect();
        let mut buffer = Vec::new();

        for split in splits {
            let length = split.len() as u8;
            // let label = format!("{:#x}{}", length, split);
            buffer.push(length);
            buffer.extend_from_slice(&split.as_bytes());
        }
        buffer.push(0);

        buffer
    }
    pub fn encode(&self) -> Vec<u8> {
        // Encode domain name
        let mut buffer = Vec::new();
        let labels = Self::encode_domain_name(&self.domain_name);
        buffer.extend_from_slice(&labels);

        // Type byte
        buffer.extend_from_slice(&(self.type_ as u16).to_be_bytes());

        // Class byte
        buffer.extend_from_slice(&(self.class as u16).to_be_bytes());

        buffer
    }
}
