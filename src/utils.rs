use std::io::{BufRead, BufReader};

pub fn encode_domain_name(domain_name: &str) -> Vec<u8> {
    // Split based on "."
    let splits: Vec<&str> = domain_name.split(".").collect();
    let mut buffer = Vec::new();

    for split in splits {
        let length = split.len() as u8;
        // let label = format!("{:#x}{}", length, split);
        buffer.push(length);
        buffer.extend_from_slice(&split.as_bytes());
    }
    buffer.push(0);

    buffer
}

pub fn decode_domain_name_label_sequence(sequence: &[u8]) -> String {
    let mut buffer = Vec::new();

    let mut sequence_reader = BufReader::new(sequence);
    let mut sequence: Vec<u8> = Vec::new();

    println!("Reading");
    sequence_reader
        .read_until(0, &mut sequence)
        .expect("Error occurred");
    println!("Found");

    // Remove the null byte
    sequence.pop();
    // Get length of first label, Get content of label
    let mut index = 0;
    while index < sequence.len() {
        let label_length = sequence[index] as usize;
        let label_bytes = &sequence[index + 1..label_length + index + 1];
        let label = String::from_utf8(Vec::from(label_bytes))
            .expect("Error occured while decoding domain name label");
        buffer.push(label);

        index += 1 + label_length
    }

    let result = buffer.join(".");
    dbg!(&result);

    result
}
