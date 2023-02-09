use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use sha2::{Digest, Sha256};

fn main() {
    let file_in = std::env::args_os().nth(1).unwrap();
    let file_out = std::env::args_os().nth(2);

    let file_in = File::open(file_in).unwrap();
    let file_in = BufReader::new(file_in);

    let mut file_out = match file_out {
        Some(file_out) => {
            let file_out = File::create(file_out).unwrap();
            let file_out = BufWriter::new(file_out);
            Some(file_out)
        }
        None => None,
    };

    let mut hasher = Sha256::new();
    for line in file_in.lines() {
        let line = line.unwrap();
        let line = format!("{}\n", line);
        hasher.update(&line);

        if let Some(file_out) = &mut file_out {
            file_out.write(line.as_bytes()).unwrap();
        }
    }

    let hash = hasher.finalize();

    let hex_hash = base16ct::lower::encode_string(&hash);
    println!("{}", hex_hash);
}
