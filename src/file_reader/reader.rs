use std::fs;

pub fn read_file(filename: &str) -> Option<String> {
    let contents = match fs::read_to_string(filename) {
        Ok(mut contents) => {
            contents.retain(|c| !"\r\n ".contains(c));
            Some(contents)
        }
        Err(_e) => {
            None
        }
    };

    contents
}