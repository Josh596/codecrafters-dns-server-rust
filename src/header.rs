pub struct DNSHeader {
    id: u16,                      // 2 bytes
    response_indicator: bool,     // 1 bit
    opcode: u8,                   // 4 bits
    authoritative_answer: bool,   // 1 bit
    truncation_required: bool,    // 1 bit
    recursion_desired: bool,      // 1 bit
    recursion_available: bool,    // 1 bit
    reserved: u8,                 // 3 bits
    response_code: u8,            // 4 bits
    question_count: u16,          // 2 bytes
    answer_record_count: u16,     // 2 bytes
    authority_record_count: u16,  // 2 bytes
    additional_record_count: u16, // 2 bytes
}

impl DNSHeader {
    pub fn new(question_count: u16) -> Self {
        DNSHeader {
            id: 1234,
            response_indicator: true,
            opcode: 0,
            authoritative_answer: false,
            truncation_required: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: 0,
            response_code: 0,
            question_count: question_count,
            answer_record_count: 0,
            authority_record_count: 0,
            additional_record_count: 0,
        }
    }

    pub fn encode(&self) -> [u8; 12] {
        let mut buffer = [0u8; 12];

        buffer[0..2].copy_from_slice(&self.id.to_be_bytes());
        buffer[2] = (self.response_indicator as u8) << 7
            | (self.opcode & 0x0F) << 3
            | (self.authoritative_answer as u8) << 2
            | (self.truncation_required as u8) << 1
            | (self.recursion_desired as u8);

        buffer[3] = (self.recursion_available as u8) << 7
            | (self.reserved & 0x07) << 4
            | (self.response_code & 0x0F);

        buffer[4..6].copy_from_slice(&self.question_count.to_be_bytes());
        buffer[6..8].copy_from_slice(&self.answer_record_count.to_be_bytes());
        buffer[8..10].copy_from_slice(&self.authority_record_count.to_be_bytes());
        buffer[10..].copy_from_slice(&self.additional_record_count.to_be_bytes());

        buffer
    }
}
