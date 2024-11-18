use crate::{answer::DNSAnswer, header::DNSHeader, question::DNSQuestions};
#[derive(Clone, Debug)]
pub struct DNSMessage {
    pub header: DNSHeader,
    pub questions: DNSQuestions,
    pub answer: DNSAnswer,
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

    pub fn split(&self) -> Vec<Self> {
        let mut buf = Vec::new();

        for question in &self.questions.questions {
            let message = DNSMessage {
                header: self.header.clone(),
                questions: DNSQuestions {
                    questions: vec![question.clone()],
                },
                answer: DNSAnswer { rr: vec![] },
            };

            buf.push(message);
        }
        buf
    }

    pub fn merge(messages: Vec<Self>) -> Self {
        let mut questions = Vec::new();
        let mut answers = Vec::new();
        for message in &messages {
            questions.extend_from_slice(&message.questions.questions);
            answers.extend_from_slice(&message.answer.rr);
        }

        let mut header = messages[0].header.clone();
        header.ancount = messages.len() as u16;
        header.qdcount = messages.len() as u16;

        DNSMessage {
            header: header,
            questions: DNSQuestions {
                questions: questions,
            },
            answer: DNSAnswer { rr: answers },
        }
    }
}

impl From<&[u8]> for DNSMessage {
    fn from(data: &[u8]) -> Self {
        let mut cursor = 0;
        let mut header = DNSHeader::from(&data[..12]);
        cursor += 12;
        let (questions, bytes_read) = DNSQuestions::from_bytes(&data[12..], header.qdcount);
        cursor += bytes_read;
        let (answer, bytes_read) = DNSAnswer::from_bytes(&data[cursor..], header.ancount);
        cursor += bytes_read;

        // header.qr = true;
        // header.aa = false;
        // header.tc = false;
        // header.ra = false;
        // header.z = 0;
        // header.rcode = if header.opcode != 0 { 4 } else { 0 };

        Self {
            header,
            questions,
            answer,
        }
    }
}
