use clap::{arg, value_parser, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("man")
        .bin_name("man")
        .author("Bradford Fults <bfults@gmail.com>")
        .version("1.0") //.
        .disable_version_flag(true)
        .about("A better manual command that finds the answer you're looking for.")
        .allow_missing_positional(true)
        .arg(arg!([section] "Manual section").value_parser(value_parser!(u8)))
        .arg(arg!(<command> "Command to look up"))
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::Version)
                .help("Print version"),
        )
        .get_matches();

    if let Some(command) = matches.get_one::<String>("command") {
        if let Some(section) = matches.get_one::<u8>("section") {
            println!("command to run: {section} {command}");
        } else {
            println!("command to run: {command}");
        }
    }
    // shell out to the man command
    // println!(
    //     "{:?}",
    //     std::process::Command::new("man").arg(command).spawn()
    // );
}
