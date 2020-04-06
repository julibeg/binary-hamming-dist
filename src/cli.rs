pub fn parse_cmd_line() -> (String, char, Option<String>, usize) {
    let matches = clap::App::new("Binary Hamming Distance Calculator")
        .about(
            "Calculates the pairwise distance matrix of binary strings and \
             tolerates missing values. \n\
             The input file should hold one sample per line and look like: \n\n\
             001X0 \n\
             011X0 \t where 'X' denotes a missing value. This yields \n\
             10X10 \n\n\
             0,1,1 \n\
             1,0,2 \t as result.\n\
             1,2,0 \n",
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
            clap::Arg::with_name("NA-char")
                .help("the character [A-Za-z2-9] specifying missing values")
                .takes_value(true)
                .default_value("X")
                .possible_values(&[
                    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
                    "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f",
                    "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                    "w", "x", "y", "z", "2", "3", "4", "5", "6", "7", "8", "9",
                ])
                .hide_possible_values(true)
                .short("n")
                .long("NA-value")
                .value_name("CHAR")
                .display_order(2),
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
                .value_name("NUM"),
        )
        .get_matches();

    // calling unwrap is safe here because `input` was `required` by clap
    // and `NA-char` has a default as well as allowed arguments.
    let infname = matches.value_of("input").unwrap().to_string();
    let na_char = matches.value_of("NA-char").unwrap().chars().next().unwrap();
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
    (infname, na_char, output, threads)
}
