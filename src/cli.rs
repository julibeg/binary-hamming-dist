pub fn parse_cmd_line() -> (String, Option<String>, usize, bool) {
    let matches = clap::App::new("Binary Hamming Distance Calculator")
        .about(
            "Calculates the pairwise distance matrix of binary strings and \
             ignores missing values. \n\
             The input file should hold one sample (i.e. bit string) per line and \
             look like: \n\n\
             1001X0X \n\
             1011X01   where anything other than '0' and '1' denotes missing values. This yields \n\
             X10X111 \n\n\
             0,1,2 \n\
             1,0,3     as result.\n\
             2,3,0 \n\n\
             For files with transposed data (one sample per column) use -T.",
        )
        .version(clap::crate_version!())
        .arg(
            clap::Arg::with_name("input")
                .help("input file")
                .takes_value(true)
                .short("i")
                .long("input")
                .required(true)
                .value_name("FILE")
                .display_order(1),
        )
        .arg(
            clap::Arg::with_name("output")
                .help("output file; if missing, result is printed to STDOUT")
                .takes_value(true)
                .short("o")
                .long("output")
                .value_name("FILE")
                .display_order(3),
        )
        .arg(
            clap::Arg::with_name("threads")
                .help("number of threads; '0' will use all available CPUs")
                .takes_value(true)
                .short("t")
                .long("threads")
                .default_value("1")
                .value_name("NUM")
                .display_order(4),
        )
        .arg(
            clap::Arg::with_name("transposed")
                .help("use when input file is transposed (bit strings in columns)")
                .short("T")
                .long("transposed")
                .display_order(5),
        )
        .get_matches();

    // calling unwrap is safe here because `input` was `required` by clap
    // and `NA-char` has a default as well as allowed arguments.
    let infname = matches.value_of("input").unwrap().to_string();
    let output = match matches.value_of("output") {
        None => None,
        Some(fname) => Some(fname.to_string()),
    };
    let threads: usize = matches
        .value_of("threads")
        .unwrap()
        .parse()
        .unwrap_or_else(|err| {
            eprintln!(
                "Error parsing command line arguments: {}. \n\
                 Please provide a valid integer value for the threads argument",
                err
            );
            std::process::exit(1);
        });
    let transposed = matches.is_present("transposed");
    (infname, output, threads, transposed)
}
