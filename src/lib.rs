pub mod bitarr;
pub mod cli;
pub mod trimat;

use bitarr::BitArrNa;
use std::fs;
use std::io::{self, BufRead, Seek, SeekFrom};
use std::iter;

/// read input file with samples in rows like
/// 010X11001
/// 01100X010
/// where 'X' specifies unknown values
pub fn read_file_samples_rows(infname: &str) -> Vec<BitArrNa> {
    let mut bitarrs: Vec<BitArrNa> = Vec::new();

    let infile = fs::File::open(&infname).unwrap_or_else(|err| {
        eprintln!("Error opening input file {}: {}", infname, err);
        std::process::exit(1);
    });
    let lines = io::BufReader::new(infile).lines();

    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            let bitarr = BitArrNa::from_string(&line);
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
pub fn read_file_samples_columns(infname: &str) -> Vec<BitArrNa> {
    // there are two ways of dealing with the file being transposed if
    // we don't want to load the whole file into memory:
    //   1. traverse the file for the first time to determine the
    //      dimensions  of the data, initialise `BitArrNa`s of
    //      corresponding lengths, and traverse another time to actually
    //      read the data and set the bits.
    //      this is not much slower than processing a non-transposed file,
    //      but requires being able to traverse the file twice, which won't
    //      work with pipes.
    //   2. traverse the file only once, but let the `BitArrNa`s grow
    //      while setting the bits. this requires many reallocations and
    //      is therefore slower.
    // when process substitution is used for piping the output of a command
    // to the program, `infname` will start with "/dev/fd/". in this case,
    // we call the function that traverses the file only once but grows the
    // bit vectors.
    if infname.starts_with("/dev/fd/") {
        return read_file_samples_columns_traverse_once(infname);
    }
    // traverse file first to determine dimensions
    let mut infile = fs::File::open(&infname).unwrap_or_else(|err| {
        eprintln!("Error opening input file {}: {}", infname, err);
        std::process::exit(1);
    });
    let mut lines = io::BufReader::new(&infile).lines();
    // get first line and count number of characters --> that's the number of samples
    let n_samples = match lines.next() {
        Some(Ok(line)) => line.len(),
        _ => {
            eprintln!("Error parsing input file. Is it empty?");
            std::process::exit(1);
        }
    };
    // get number of lines (i.e. length of the bit strings) and add 1 for the first line
    let length = lines.count() + 1;
    // initialize empty `bitarrs` with `BitArrNa`s of the correct length. all
    // `bits` will be set to `0` and `not_nas` to `1`.
    let mut bitarrs: Vec<BitArrNa> = (0..n_samples).map(|_| BitArrNa::new(length)).collect();
    // now the buffer's iterator has been consumed and we have to reset
    // the cursor to the top of the file
    infile
        .seek(SeekFrom::Start(0))
        .expect("Error: Could not reset cursor to top of input file. Was it just a pipe?");
    let lines = io::BufReader::new(infile).lines();
    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            for (j, c) in line.chars().enumerate() {
                // the bits are already set to not-na and 0 --> only need to
                // do something if a char is '1' or neither '1' or '0'
                match c {
                    '0' => (),
                    '1' => bitarrs[j].bits.set(i, true),
                    _ => bitarrs[j].not_nas.set(i, false),
                };
            }
        } else {
            eprintln!("Error reading input file at line {}", i + 1);
            std::process::exit(1);
        }
    }
    bitarrs
}

pub fn read_file_samples_columns_traverse_once(infname: &str) -> Vec<BitArrNa> {
    let infile = fs::File::open(&infname).unwrap_or_else(|err| {
        eprintln!("Error opening input file {}: {}", infname, err);
        std::process::exit(1);
    });
    let mut lines = io::BufReader::new(&infile).lines();
    // get first line and count number of characters --> that's the number of samples
    let first_line = lines.next();
    let n_samples = match &first_line {
        Some(Ok(line)) => line.len(),
        _ => {
            eprintln!("Error parsing input file. Is it empty?");
            std::process::exit(1);
        }
    };
    // initialize empty `bitarrs` with `BitArrNa`s of length 1
    // the first bit` will be set to '0' and `not_nas` to '1'
    let mut bitarrs: Vec<BitArrNa> = (0..n_samples).map(|_| BitArrNa::new(0)).collect();
    // add the first line back to the beginning of the iterator
    let lines = iter::once(first_line.unwrap()).chain(lines);
    // iterate over lines and push bits accordingly
    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            for (c, bitarr) in line.chars().zip(bitarrs.iter_mut()) {
                match c {
                    '0' => {
                        bitarr.bits.push(false);
                        bitarr.not_nas.push(true);
                    }
                    '1' => {
                        bitarr.bits.push(true);
                        bitarr.not_nas.push(true);
                    }
                    _ => {
                        bitarr.bits.push(false);
                        bitarr.not_nas.push(false);
                    }
                };
            }
        } else {
            eprintln!("Error reading input file at line {}", i + 1);
            std::process::exit(1);
        }
    }
    bitarrs
}
