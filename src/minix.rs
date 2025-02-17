extern crate colored;

use colored::*;

use notify::{
    Config,
    Watcher, 
    RecursiveMode, 
    RecommendedWatcher, 
};

use crate::{
    args_cli::Flags,
    core::minify::Minify
};

use std::{
    path::Path,
    error::Error,
    sync::mpsc::channel,

    io::{
        Write,
        ErrorKind,
        Error as IoError,
    },

    fs::{
        File,
        read_dir,
        read_to_string,
    },
};

pub struct Minix {
    input: String,
    output: Option<String>,
    watch: bool,
}

impl Minix {
    
    pub fn new(flags: Flags) -> Self {
        Self {
            input: flags.input,
            output: flags.output,
            watch: flags.watch,
        }
    }

    fn read(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let content = if Path::new(&input).is_file() {
            read_to_string(&input)?
        } else {
            "".to_string()
        };

        Ok(content)
    }

    fn scan_path(&self, input: &str, filter: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
        let paths = read_dir(input)?;

        if let Some(output) = output {
            let mut content = String::new();

            for path in paths {
                let path = path?.path();
                let path_str = path.to_str().unwrap();

                if path.is_file() && !path_str.contains(".min") && path_str.ends_with(filter) {
                    let content_file = self.read(path_str)?;
                    content.push_str(&content_file);
                }
            }

            let output_file = output.to_string();
            self.append_write(&content, input, &output_file, filter)?;
        } else {
            for path in paths {
                let path = path?.path();
                let path_str = path.to_str().unwrap();
    
                if path.is_file() && !path_str.contains(".min") && path_str.ends_with(filter) {
                    let output_file = path.to_string_lossy().replace(
                        filter, format!("min.{}", filter).as_str()
                    );
                    
                    self.write(path_str, &output_file)?;
                }
            }
        }

        Ok(())
    }

    fn write(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let content = self.read(input)?;
        let content_minified = if input.ends_with("js") {
            Minify.js(&content)
        } else {
            Minify.css(&content)
        };

        let mut file = File::create(output)?;
        file.write_all(content_minified.as_bytes())?;

        println!("File minified from {} to {} was successfully!", input.blue(), output.green());
        Ok(())
    }
    
    fn append_write(&self, content: &str, input: &str, output: &str, filter: &str) -> Result<(), Box<dyn Error>> {
        let content_minified = if filter == "js" {
            Minify.js(&content)
        } else {
            Minify.css(&content)
        };

        let mut file = File::create(output)?;
        file.write_all(content_minified.as_bytes())?;

        println!("File minified from {} to {} was successfully!", input.bold().blue(), output.bold().green());
        Ok(())
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.watch {
            self.watch_mode()?;
        } else {
            self.minify_once()?;
        }
        
        Ok(())
    }

    fn minify_once(&self) -> Result<(), Box<dyn Error>> {
        if self.input.contains("*") {
            let filter: Vec<&str> = self.input.split("*.").collect();
            
            self.scan_path(
                filter[0], 
                filter[1], 
                self.output.as_deref()
            )?;
        } else {
            self.write(
                &self.input, 
                self.output.as_deref().unwrap_or("")
            )?;
        }

        Ok(())
    }

    fn watch_mode(&self) -> Result<(), Box<dyn Error>> {
        let (tx, rx) = channel();
        let watch_path_str = self.input.rsplit_once("/*").map(|(dir, _)| dir).unwrap_or(&self.input);
        let watch_path = Path::new(watch_path_str);
    
        if !watch_path.is_dir() {
            return Err(Box::new(IoError::new(
                ErrorKind::NotFound,
                format!("Directory '{}' not found.", watch_path.display()),
            )));
        }
    
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(watch_path, RecursiveMode::Recursive)?;
    
        println!("🔍 Watching for changes in {}...", self.input.yellow());
    
        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    for path in event.paths {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if ext == "js" || ext == "css" {
                                self.minify_once()?;
                            }
                        }
                    }
                }
                Ok(Err(e)) => eprintln!("⚠ Watch error: {:?}", e),
                Err(e) => eprintln!("❌ Channel error: {:?}", e),
            }
        }
    }
    
}