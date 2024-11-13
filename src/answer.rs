use crate::utils;

pub struct ResourceRecord {
    domain_name: String,
    type_: u16,
    class: u16,
    ttl: u32,
    RDATA: Vec<u8>,
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
        buffer.extend_from_slice(&(self.RDATA.len() as u16).to_be_bytes());
        buffer.extend_from_slice(&self.RDATA);

        buffer
    }
}

pub struct DNSAnswer {
    RR: Vec<ResourceRecord>,
}

impl DNSAnswer {
    pub fn new() -> Self {
        let mut records = Vec::new();

        records.push(ResourceRecord {
            domain_name: String::from("codecrafters.io"),
            type_: 1,
            class: 1,
            ttl: 60,
            RDATA: Vec::from((192168701 as u32).to_be_bytes()),
        });

        Self { RR: records }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        for resource in &self.RR {
            buffer.extend_from_slice(&resource.encode());
        }
        buffer
    }
}
