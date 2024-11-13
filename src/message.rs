use crate::{answer::DNSAnswer, header::DNSHeader, question::DNSQuestion};

pub struct DNSMessage {
    pub header: DNSHeader,
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

impl From<&[u8]> for DNSMessage {
    fn from(data: &[u8]) -> Self {
        let mut header = DNSHeader::from(&data[..12]);
        let question: DNSQuestion = DNSQuestion::new();
        let answer: DNSAnswer = DNSAnswer::new();

        header.qr = true;
        header.aa = false;
        header.tc = false;
        header.ra = false;
        header.z = 0;
        header.ancount = 1;

        header.rcode = if header.opcode != 0 { 4 } else { 0 };

        Self {
            header,
            question,
            answer,
        }
    }
}
