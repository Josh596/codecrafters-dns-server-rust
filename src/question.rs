use std::io::{BufRead, BufReader, Read};

use crate::utils;

pub struct DNSQuestions {
    pub questions: Vec<DNSQuestion>,
}

impl DNSQuestions {
    pub fn from_bytes(bytes: &[u8], qd_count: u16) -> Self {
        let mut questions = Vec::new();
        let mut questions_reader = BufReader::new(bytes);

        dbg!(&qd_count);
        let mut buffer: Vec<u8> = Vec::new();
        let mut current_offset: usize = 0;
        for _ in 0..qd_count {
            println!("Getting Question");

            questions_reader
                .read_until(0, &mut buffer)
                .expect("Error occurred");

            // Read the next 4 bytes and append them to buffer
            let mut extra_bytes = [0u8; 4];
            questions_reader
                .read_exact(&mut extra_bytes)
                .expect("Error occurred while reading the next 4 bytes");

            buffer.extend_from_slice(&extra_bytes); // Append the 4 bytes to buffer

            println!("{}", &buffer.len());

            let question = DNSQuestion::from_bytes(&buffer[..], bytes, current_offset);
            dbg!(&question.domain_name);
            questions.push(question);
            current_offset += &buffer.len();

            dbg!(current_offset);
            buffer.clear();
            println!("Question added");
        }

        Self { questions }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for question in &self.questions {
            buf.extend_from_slice(&question.encode());
        }

        buf
    }
}

pub struct DNSQuestion {
    pub domain_name: String,
    pub type_: u16,
    pub class: u16,
    pub offset: u16,
}

impl DNSQuestion {
    // pub fn new() -> Self {
    //     Self {
    //         domain_name: String::from("codecrafters.io"),
    //         type_: 1,
    //         class: 1,
    //         offset: 0,
    //     }
    // }

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

    pub fn from_bytes(data: &[u8], full_questions_byte: &[u8], current_offset: usize) -> Self {
        println!("Length of buffer: {}", data.len());
        // if dns_message.len() < 12 {}
        let (domain_name, size) =
            utils::decode_domain_name_label_sequence(&data, full_questions_byte, current_offset);

        let type_ = data[size] as u16;
        let class = data[size + 1] as u16;

        Self {
            domain_name,
            type_,
            class,
            offset: 12,
        }
    }
}
