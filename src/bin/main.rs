use binary_hamming_dist::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::io;

type Dist = u32;

fn main() {
    // get command line arguments
    let (infname, output, threads, transposed) = cli::parse_cmd_line();

    // create thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .expect("Error initializing threadpool");

    // parse file into bitarr vec
    let bitarrs: Vec<bitarr::BitArrNa> = if transposed {
        read_file_samples_columns(&infname)
    } else {
        read_file_samples_rows(&infname)
    };

    // initialize triangular distance matrix
    let n = bitarrs.len();
    let mut dists: trimat::TriMat<Dist> = trimat::TriMat::new(n - 1);

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
