use clap::{App, Arg};

fn main() {
    let args = App::new("part")
        .version(option_env!("CARGO_PKG_VERSION").unwrap())
        .author("JMARyA <jmarya0@icloud.com>")
        .about("split and combine files")
        .multicall(true)
        .subcommand(
            App::new("split")
            .alias("s")
            .about("split into multiple parts")
            .arg(
                Arg::with_name("parts")
                    .short('n')
                    .long("number")
                    .required(false)
                    .value_name("PARTS")
                    .conflicts_with("size")
                    .help("number of part files"),
            )
            .arg(
                Arg::with_name("size")
                    .short('s')
                    .long("size")
                    .required(false)
                    .value_name("SIZE")
                    .conflicts_with("parts")
                    .help("size of individual part files"),
            )
            .arg(
                Arg::with_name("file")
                    .required(true)
                    .value_name("FILE")
                    .help("file to split"),
            ))
            .subcommand(
                App::new("combine")
                .about("combine multiple parts")
                .alias("c")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .value_name("FILE")
                        .help("file partinfo"),
                )
            )
        .get_matches();

    match args.subcommand().unwrap() {
        ("split", args) => {
            let f = args.value_of("file").unwrap();
            let parts = args.value_of("parts");
            let size = args.value_of("size");

            if parts.is_some() {
                part::split_file(f, part::SplitOptions::NumberOfParts(parts.unwrap().parse().unwrap()));
            }
            if size.is_some() {
                part::split_file(f, part::SplitOptions::SizeOfParts(size.unwrap().parse().unwrap()));
            }
        }
        ("combine", args) => {
            let f = args.value_of("file").unwrap();
            part::combine_file(f);
        }
        _ => {
            std::process::exit(1);
        }
    }
}