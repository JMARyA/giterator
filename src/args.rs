use clap::{arg, command, ArgMatches};

pub fn get_args() -> ArgMatches {
    command!()
        .about("Iterate over Git commits and run commands")
        .arg(arg!([command] "Command to run for each commit").required(true))
        .arg(
            arg!([repository] "Git Repository")
                .required(false)
                .default_value("."),
        )
        .arg(
            clap::Arg::new("allow-dirty")
                .long("allow-dirty")
                .help("Allow working with unclean repository")
                .num_args(0)
                .required(false),
        )
        .arg(
            clap::Arg::new("script_file")
                .short('s')
                .long("script")
                .help("Use the content of a script file as command")
                .num_args(0)
                .required(false),
        )
        .arg(
            clap::Arg::new("json")
                .short('j')
                .long("json")
                .conflicts_with("csv")
                .help("Output as JSON")
                .num_args(0)
                .required(false),
        )
        .arg(
            clap::Arg::new("streaming")
                .long("streaming")
                .help("Output results as soon as they are available")
                .num_args(0)
                .required(false),
        )
        .arg(
            clap::Arg::new("csv")
                .short('c')
                .long("csv")
                .conflicts_with("json")
                .help("Output as CSV")
                .num_args(0)
                .required(false),
        )
        .get_matches()
}
