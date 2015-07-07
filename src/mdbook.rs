use std::path::PathBuf;
use std::fs::{self, File, metadata};
use std::io::Write;
use std::io::{Error, ErrorKind};

pub struct MDBook {
    dest: PathBuf,
    src: PathBuf,
}

impl MDBook {

    pub fn new() -> Self {
        MDBook {
            dest: PathBuf::from("book"),
            src: PathBuf::from("src"),
        }
    }

    pub fn init(&self, dir: &PathBuf) -> Result<(), Error> {

        // Hacky way to check if the directory exists... Until PathExt moves to stable
        match metadata(dir) {
            Err(_) => return Err(Error::new(ErrorKind::Other, "Directory does not exist")),
            _ => {}
        }

        // Logic problem: When self.dest is absolute, the directory given
        // as parameter is never used...
        let dest = if self.dest.is_relative() {
            dir.join(&self.dest)
        } else {
            self.dest.clone()
        };

        let src = if self.src.is_relative() {
            dir.join(&self.src)
        } else {
            self.src.clone()
        };

        // Hacky way to check if the directory exists... Until PathExt moves to stable
        match metadata(&dest) {
            Err(_) => {
                // There is a very high chance that the error is due to the fact that
                // the directory / file does not exist
                fs::create_dir(&dest).unwrap();
            },
            Ok(_) => { /* If there is no error, the directory / file does exist */ }
        }

        // Hacky way to check if the directory exists... Until PathExt moves to stable
        match metadata(&src) {
            Err(_) => {
                // There is a very high chance that the error is due to the fact that
                // the directory / file does not exist
                fs::create_dir(&src).unwrap();
            },
            Ok(_) => { /* If there is no error, the directory / file does exist */ }
        }

        // Hacky way to check if the directory exists... Until PathExt moves to stable
        let summary = match metadata(&src.join("SUMMARY.md")) {
            Err(_) => {
                // There is a very high chance that the error is due to the fact that
                // the directory / file does not exist
                Result::Ok(File::create(&src.join("SUMMARY.md")).unwrap())
            },
            Ok(_) => {
                /* If there is no error, the directory / file does exist */
                Result::Err("SUMMARY.md does already exist")
            }
        };

        if let Ok(mut f) = summary {
            try!(writeln!(f, "# Summary"));
            try!(writeln!(f, ""));
            try!(writeln!(f, "[Chapter 1](./chapter_1.md)"));

            let mut chapter_1 = File::create(&src.join("chapter_1.md")).unwrap();
            try!(writeln!(chapter_1, "# Chapter 1"));
        }

        return Ok(());
    }

    pub fn build(&self, dir: &PathBuf) -> Result<(), Error> {

        Ok(())
    }

    pub fn set_dest(mut self, dest: PathBuf) -> Self {
        self.dest =  dest;
        self
    }

    pub fn set_src(mut self, src: PathBuf) -> Self {
        self.src = src;
        self
    }

}