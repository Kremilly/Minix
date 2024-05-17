use std::{
    io::Write,
    path::Path,
    error::Error,

    fs::{
        self,
        File,
    },
};

use crate::minix::minify::Minify;

pub struct Engine;

impl Engine {

    fn read(input: &str) -> Result<String, Box<dyn Error>> {
        let content;

        if Path::new(&input).is_file() {
            content = fs::read_to_string(&input)?;
        } else {
            content = "".to_string();
        }

        Ok(content)
    }

    pub fn write(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let content_minified;
        let content = Self::read(input)?;

        if input.ends_with(".js") {
            content_minified = Minify::js(&content);
        } else {
            content_minified = Minify::css(&content);
        }

        let mut file = File::create(output)?;
        file.write_all(content_minified.as_bytes())?;

        println!("File minified from {} ({}) with successfully!", input, output);
        Ok(())
    }

    pub fn run(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        Self::write(input, output)?;
        Ok(())
    }

}
