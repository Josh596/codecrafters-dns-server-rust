use crate::utils;

pub struct ResourceRecord {
    pub domain_name: String,
    pub type_: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdata: Vec<u8>,
}

impl ResourceRecord {
    // fn new() -> Self {
    //     Self {
    //         domain_name: String::from("codecrafters.io"),
    //         type_: 1,
    //         class: 1,
    //         ttl: 60,
    //         RDATA: Vec::from((192168701 as u32).to_be_bytes()),
    //     }
    // }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&utils::encode_domain_name(&self.domain_name));
        buffer.extend_from_slice(&self.type_.to_be_bytes());
        buffer.extend_from_slice(&self.class.to_be_bytes());
        buffer.extend_from_slice(&self.ttl.to_be_bytes());
        buffer.extend_from_slice(&(self.rdata.len() as u16).to_be_bytes());
        buffer.extend_from_slice(&self.rdata);

        buffer
    }
}

pub struct DNSAnswer {
    pub rr: Vec<ResourceRecord>,
}

impl DNSAnswer {
    pub fn new() -> Self {
        let mut records = Vec::new();

        records.push(ResourceRecord {
            domain_name: String::from("codecrafters.io"),
            type_: 1,
            class: 1,
            ttl: 60,
            rdata: Vec::from((192168701 as u32).to_be_bytes()),
        });

        Self { rr: records }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        for resource in &self.rr {
            buffer.extend_from_slice(&resource.encode());
        }
        buffer
    }
}
