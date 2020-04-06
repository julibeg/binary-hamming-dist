use binary_hamming_dist::bitarr::BitArrNa;
use binary_hamming_dist::cli::parse_cmd_line;
use binary_hamming_dist::trimat::TriMat;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::io;
use std::io::BufRead;

type Dist = u32;

fn main() {
    // get command line arguments
    let (infname, na_char, output, threads): (String, char, Option<String>, usize) =
        parse_cmd_line();

    // create thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .expect("Error initializing threadpool");

    // parse file into bitarr vec
    let mut bitarrs: Vec<BitArrNa> = Vec::new();
    let infile = fs::File::open(infname).unwrap_or_else(|err| {
        eprintln!("Error opening input file: {}", err);
        std::process::exit(1);
    });
    let infile = io::BufReader::new(infile);

    for (i, line) in infile.lines().enumerate() {
        if let Ok(line) = line {
            bitarrs.push(BitArrNa::from_string(&line, na_char).unwrap_or_else(|err| {
                eprintln!("Error generating bitarr at line {}: {}", i + 1, err);
                std::process::exit(1);
            }));
        } else {
            eprintln!("Error reading input file at line {}", i + 1);
            std::process::exit(1);
        }
    }

    // create Vec of Vecs for holding the distances (resembling a triangular matrix)
    let n = bitarrs.len();
    let mut dists: TriMat<Dist> = TriMat::new(n - 1);

    // setup progress bar
    let pb = ProgressBar::new(n as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:50.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-"),
    );

    // get the distances
    dists
        .mat
        .par_iter_mut()
        .zip(bitarrs[..n - 1].par_iter()) // skip last sample
        .enumerate()
        .for_each(|(i, (dists_i, bitarr_i))| {
            for bitarr_j in &bitarrs[i + 1..] {
                dists_i.push(bitarr_i.dist(bitarr_j));
            }
            pb.inc(1);
        });
    pb.finish_with_message("done");

    // write result to file
    if let Some(outfname) = output {
        let mut file = fs::File::create(&outfname).unwrap_or_else(|err| {
            eprintln!("Error opening output file: {}", err);
            std::process::exit(1);
        });
        dists.write_symmetric(&mut file);
        println!("Result written to {}", outfname);
    } else {
        dists.write_symmetric(&mut io::stdout());
    }
}
