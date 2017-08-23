use std::env;
use std::fmt;
use std::io;
use std::process;
use std::fs;
use std::io::{Read, Write};


fn main() {
    let (src, dst) = match get_src_dst(env::args()) {
        Ok((src, dst)) => (src, dst),
        Err(e) => {
            println!("{}", e);
            process::exit(0);
        }
    };

    let mut i_file = match fs::File::open(src) {
        Ok(f) => f,
        Err(e) => {
            println!("{}",e);
            process::exit(0);
        }
    };

    let mut o_file = match fs::File::open(dst) {
        Ok(f) => f,
        Err(e) => {
            println!("{}",e);
            process::exit(0);
        }
    };

    let mut buffer = [0u8;1024];
    loop {
        let n_bytes = i_file.read(&mut buffer).unwrap();
        if n_bytes == 0 {
            break;
        }
        o_file.write(&buffer[..n_bytes]).unwrap();
    }
}

fn get_src_dst(mut arg: env::Args) -> Result<(String, String), CopyFileError> {
    if arg.len() < 3 {
        return Err(CopyFileError::LessArgs);
    }
    arg.next();
    let src = arg.next().unwrap();
    let dst = arg.next().unwrap();
    Ok((src, dst))
}

#[derive(Debug)]
enum CopyFileError {
    LessArgs,
    IoError(io::Error),
}

impl fmt::Display for CopyFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopyFileError::LessArgs => write!(f, "必须输入2个filepath (src, dst)"),
            CopyFileError::IoError(ref e) => e.fmt(f),
        }
    }
}
