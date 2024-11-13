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
