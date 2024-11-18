use std::io::{BufRead, BufReader, Read};

use crate::utils;
#[derive(Clone, Debug)]
pub struct ResourceRecord {
    pub domain_name: String,
    pub type_: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdata: Vec<u8>,
}

impl ResourceRecord {
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
#[derive(Clone, Debug)]
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

    pub fn from_bytes(bytes: &[u8], ancount: u16) -> (Self, usize) {
        let mut records = Vec::new();
        let mut records_reader = BufReader::new(bytes);

        let mut current_offset = 0;
        for _ in 0..ancount {
            let mut buffer: Vec<u8> = Vec::new();
            records_reader
                .read_until(0, &mut buffer)
                .expect("Could not read bytes");

            let mut extra_bytes = [0u8; 10];
            records_reader.read_exact(&mut extra_bytes);

            let type_ = u16::from_be_bytes([extra_bytes[0], extra_bytes[1]]);
            let class = u16::from_be_bytes([extra_bytes[2], extra_bytes[3]]);
            let ttl = u32::from_be_bytes([
                extra_bytes[4],
                extra_bytes[5],
                extra_bytes[6],
                extra_bytes[7],
            ]);
            let rdata_length = u16::from_be_bytes([extra_bytes[8], extra_bytes[9]]);

            let mut rdata = vec![0; rdata_length as usize];
            records_reader.read_exact(&mut rdata);

            let (domain_name, _) =
                utils::decode_domain_name_label_sequence(&buffer, bytes, current_offset);
            current_offset += buffer.len() + extra_bytes.len() + rdata.len();
            let rr = ResourceRecord {
                domain_name: domain_name,
                type_,
                class,
                ttl,
                rdata,
            };

            records.push(rr);
        }

        (Self { rr: records }, current_offset)
    }
}
