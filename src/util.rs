use std::{fs::read_to_string, path::Path};

pub fn get_input(filename: &str) -> String {
    let path = Path::new(&filename);
    let file = Path::new(path.file_name().unwrap());
    let input_dir = path.parent().unwrap().join("input");
    read_to_string(input_dir.join(file.with_extension("txt"))).unwrap().to_string()
}
