use crate::{answer::DNSAnswer, header::DNSHeader, question::DNSQuestion};

pub struct DNSMessage {
    header: DNSHeader,
    question: DNSQuestion,
    answer: DNSAnswer,
}

impl DNSMessage {
    pub fn new() -> Self {
        Self {
            header: DNSHeader::new(1, 1),
            question: DNSQuestion::new(),
            answer: DNSAnswer::new(),
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.header.encode());
        buffer.extend_from_slice(&self.question.encode());
        buffer.extend_from_slice(&self.answer.encode());
        // self.header.encode()

        buffer
    }
}
