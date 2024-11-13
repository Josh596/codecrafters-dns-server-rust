pub struct DNSHeader {
    id: u16, // 2 bytes

    // Query/Response Indicator
    pub qr: bool, // 1 bit

    // Operation Code
    pub opcode: u8, // 4 bits

    // Authoritative Answer
    pub aa: bool, // 1 bit

    // Truncation
    pub tc: bool, // 1 bit

    // Recursion Desired
    pub rd: bool, // 1 bit

    // Recursion Available
    pub ra: bool, // 1 bit

    // Reserved
    pub z: u8, // 3 bits

    // Response Code
    pub rcode: u8, // 4 bits

    // Question Count
    pub qdcount: u16, // 2 bytes

    // Answer Record Count
    pub ancount: u16, // 2 bytes

    // Authority Record Count
    nscount: u16, // 2 bytes

    // Additional Record Count
    arcount: u16, // 2 bytes
}

impl DNSHeader {
    pub fn new(question_count: u16, answer_count: u16) -> Self {
        DNSHeader {
            id: 1234,
            qr: true,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: 0,
            qdcount: question_count,
            ancount: answer_count,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn encode(&self) -> [u8; 12] {
        let mut buffer = [0u8; 12];

        buffer[0..2].copy_from_slice(&self.id.to_be_bytes());
        buffer[2] = (self.qr as u8) << 7
            | (self.opcode & 0x0F) << 3
            | (self.aa as u8) << 2
            | (self.tc as u8) << 1
            | (self.rd as u8);

        buffer[3] = (self.ra as u8) << 7 | (self.z & 0x07) << 4 | (self.rcode & 0x0F);

        buffer[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        buffer[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        buffer[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        buffer[10..].copy_from_slice(&self.arcount.to_be_bytes());

        buffer
    }
}

impl From<&[u8]> for DNSHeader {
    fn from(data: &[u8]) -> Self {
        if data.len() != 12 {
            panic!("Invalid u8 slice. The length of header is 12 bytes")
        }

        DNSHeader {
            id: u16::from_be_bytes([data[0], data[1]]),
            qr: (data[2] & 0b10000000) != 0,
            opcode: (data[2] & 0b01111000) >> 3,
            aa: (data[2] & 0b00000100) != 0,
            tc: (data[2] & 0b00000010) != 0,
            rd: (data[2] & 0b00000001) != 0,
            ra: (data[3] & 0b10000000) != 0,
            z: (data[3] & 0b0111_0000) >> 4,
            rcode: (data[3] & 0b0000_1111),
            qdcount: u16::from_be_bytes([data[4], data[5]]),
            ancount: u16::from_be_bytes([data[6], data[7]]),
            nscount: u16::from_be_bytes([data[8], data[9]]),
            arcount: u16::from_be_bytes([data[10], data[11]]),
        }
    }
}
