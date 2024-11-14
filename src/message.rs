use crate::{
    answer::{DNSAnswer, ResourceRecord},
    header::DNSHeader,
    question::{DNSQuestion, DNSQuestions},
};

pub struct DNSMessage {
    pub header: DNSHeader,
    questions: DNSQuestions,
    answer: DNSAnswer,
}

impl DNSMessage {
    // pub fn new() -> Self {
    //     Self {
    //         header: DNSHeader::new(1, 1),
    //         questions: DNSQuestions::new(),
    //         answer: DNSAnswer::new(),
    //     }
    // }
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.header.encode());
        buffer.extend_from_slice(&self.questions.encode());
        buffer.extend_from_slice(&self.answer.encode());
        // self.header.encode()

        buffer
    }
}

impl From<&[u8]> for DNSMessage {
    fn from(data: &[u8]) -> Self {
        let mut header = DNSHeader::from(&data[..12]);
        let mut questions = DNSQuestions::from_bytes(&data[12..], header.qdcount);
        let mut answer: DNSAnswer = DNSAnswer::new();

        header.qr = true;
        header.aa = false;
        header.tc = false;
        header.ra = false;
        header.z = 0;
        header.rcode = if header.opcode != 0 { 4 } else { 0 };

        // questions.questions[0].type_ = 1;
        // questions.questions[0].class = 1;

        let mut resource_records = Vec::new();
        for question in &mut questions.questions {
            question.type_ = 1;
            question.class = 1;
            resource_records.push(ResourceRecord {
                domain_name: question.domain_name.clone(),
                type_: question.type_,
                class: question.class,
                ttl: 60,
                rdata: Vec::from(&[8, 8, 8, 8]),
            });
        }

        answer.rr = resource_records;
        header.ancount = answer.rr.len() as u16;

        Self {
            header,
            questions,
            answer,
        }
    }
}
