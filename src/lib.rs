pub mod bitarr;
pub mod cli;
pub mod trimat;

use bitarr::BitArrNa;
use std::fs;
use std::io;
use std::io::BufRead;

/// read input file with samples in rows like
/// 010X11001
/// 01100X010
/// where 'X' specifies unknown values
pub fn read_file_samples_rows(infname: &str, na_char: char) -> Vec<BitArrNa> {
    let mut bitarrs: Vec<BitArrNa> = Vec::new();

    let infile = fs::File::open(&infname).unwrap_or_else(|err| {
        eprintln!("Error opening input file {}: {}", infname, err);
        std::process::exit(1);
    });
    let lines = io::BufReader::new(infile).lines();

    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            let bitarr = BitArrNa::from_string(&line, na_char).unwrap_or_else(|err| {
                eprintln!(
                    "Error generating bitarrs at input file {} line {}: {}",
                    infname,
                    i + 1,
                    err
                );
                std::process::exit(1);
            });
            bitarrs.push(bitarr);
        } else {
            eprintln!("Error reading input file at line {}", i + 1);
            std::process::exit(1);
        }
    }
    bitarrs
}

/// read input file with samples in columns like
/// 00
/// 11
/// 01
/// X0
/// 10
/// 1X
/// 00
/// 01
/// 10
/// where 'X' specifies unknown values
pub fn read_file_samples_columns(infname: &str, na_char: char) -> Vec<BitArrNa> {
    // traverse file first to determine dimensions
    let infile = fs::File::open(&infname).unwrap_or_else(|err| {
        eprintln!("Error opening input file {}: {}", infname, err);
        std::process::exit(1);
    });
    let mut lines = io::BufReader::new(infile).lines();

    // get first line and count number of characters
    let n_chars = match lines.next() {
        Some(Ok(line)) => line.len(),
        _ => {
            eprintln!("Error parsing input file. Is it empty?");
            std::process::exit(1);
        }
    };

    // get number of lines (i.e. number of samples)
    let n_samples = lines.count() + 1; // add 1 for the first line

    // initialize empty `bitarrs` with `BitArrNa`s of the correct length. all
    // `bits` will be set to `0` and `not_nas` to `1`.
    let mut bitarrs: Vec<BitArrNa> = (0..n_chars).map(|_| BitArrNa::new(n_samples)).collect();

    // now the buffer's iterator has been consumed and we have to open the file again. we can call
    // `unwrap` here, because it has already been opened without error.
    let infile = fs::File::open(&infname).unwrap();
    let lines = io::BufReader::new(infile).lines();

    for (i, line) in lines.enumerate() {
        if let Ok(sample) = line {
            for (j, c) in sample.chars().enumerate() {
                if c == '0' {
                    continue;
                } else if c == '1' {
                    bitarrs[j].bits.set(i, true);
                } else if c == na_char {
                    bitarrs[j].not_nas.set(i, false);
                } else {
                    eprintln!(
                        "Char at position {} was \'{}\'; expected \'0\', \'1\' or \'{}\'.",
                        j + 1,
                        c,
                        na_char
                    );
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error reading input file at line {}", i + 1);
            std::process::exit(1);
        }
    }
    bitarrs
}
