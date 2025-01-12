use clap::{Arg, Command};

pub struct CliArgs {
    pub token: String,
    pub alphabet: String,
    pub prefix: String,
    pub suffix: String,
    pub maxlen: usize,
}

pub fn parse_args() -> CliArgs {
    let matches = Command::new("JWT Cracker")
        .version("1.0")
        .author("Your Name")
        .about("Brute force JWT secret")
        .arg(
            Arg::new("token")
                .long("token")
                .value_name("TOKEN")
                .help("The full HS256 JWT token to crack")
                .required(true),
        )
        .arg(
            Arg::new("alphabet")
                .long("alphabet")
                .value_name("ALPHABET")
                .help("The alphabet to use for the brute force")
                .default_value("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .value_name("PREFIX")
                .help("A string that is always prefixed to the secret"),
        )
        .arg(
            Arg::new("suffix")
                .long("suffix")
                .value_name("SUFFIX")
                .help("A string that is always suffixed to the secret"),
        )
        .arg(
            Arg::new("maxlen")
                .long("maxlen")
                .value_name("MAXLEN")
                .help("The max length of the string generated during the brute force")
                .default_value("12"),
        )
        .get_matches();

    CliArgs {
        token: matches.get_one::<String>("token").unwrap().clone(),
        alphabet: matches.get_one::<String>("alphabet").unwrap().clone(),
        prefix: matches
            .get_one::<String>("prefix")
            .unwrap_or(&"".to_string())
            .clone(),
        suffix: matches
            .get_one::<String>("suffix")
            .unwrap_or(&"".to_string())
            .clone(),
        maxlen: matches
            .get_one::<String>("maxlen")
            .unwrap()
            .parse()
            .unwrap(),
    }
}
