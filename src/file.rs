use std::{error::Error, fs::File, io::prelude::*, path::Path};

pub fn write_to_file(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&Path::new(file_name))?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
