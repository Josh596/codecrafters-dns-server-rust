use std::io::{BufRead, BufReader, Read};

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

pub fn decode_domain_name_label_sequence(
    sequence: &[u8],
    full_label_sequences: &[u8],
    current_offset: usize,
) -> (String, usize) {
    let mut buffer = Vec::new();
    let adjusted_sequence = sequence;

    let mut sequence_reader = BufReader::new(adjusted_sequence);
    let mut sequence: Vec<u8> = Vec::new();

    sequence_reader
        .read_until(0, &mut sequence)
        .expect("Error occurred");
    println!("Length of Label encoded: {}", &sequence.len());
    // Remove the null byte
    sequence.pop();
    // Get length of first label, Get content of label
    let mut index = 0;
    let mut total_label_length = 0;
    while index < sequence.len() && sequence.len() != 0 {
        let label_length = sequence[index] as usize;

        if label_length < 63 {
            total_label_length += label_length + 1;
            dbg!(total_label_length);
            let label_bytes = &sequence[index + 1..label_length + index + 1];
            let label = String::from_utf8(Vec::from(label_bytes))
                .expect("Error occured while decoding domain name label");
            buffer.push(label);

            index += 1 + label_length
        } else {
            // A pointer
            let pointer = u16::from_be_bytes([sequence[index], sequence[index + 1]]);
            let offset = ((pointer & 0b0011_1111_1111_1111) - 12) as usize;
            dbg!("A pointer!!!");
            dbg!(offset);
            dbg!(format!("Added to total_label_length: {total_label_length}"));
            total_label_length += 2;
            dbg!(total_label_length);
            // Get remaining part of labels from
            let (output, _) = decode_domain_name_label_sequence(
                &full_label_sequences[offset..current_offset],
                full_label_sequences,
                current_offset + index,
            );
            buffer.push(output);

            index += 2
        }
        dbg!(&buffer);
    }

    let result = buffer.join(".");
    dbg!(&result);

    (result, total_label_length)
}
