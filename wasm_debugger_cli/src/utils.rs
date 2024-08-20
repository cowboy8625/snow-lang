pub fn into_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X} ", b))
        .collect::<String>()
        + "\n"
}
