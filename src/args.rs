pub fn snow_source_file(filename: &str) -> Result<String, String> {
    dbg!(filename);
    if filename.ends_with(".snow") {
        match std::fs::read_to_string(filename) {
            Ok(file) => Ok(file),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("This is not `snow` source file.".into())
    }
}
