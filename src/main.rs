use ancli_poems_rs;
use clap::{Arg, Command};
use std::path::Path;

fn main() {
    let cmd = Command::new("ancli-poems")
        .version("v1.0")
        .about("CLI tool to help you create anki cards and learn poems")
        .author("dazaisan99")
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .help("The path of the input file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("name")
                .long("name")
                .short('n')
                .help("The name for the output file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let path_arg = cmd.get_one::<String>("path").expect("Path is required");
    let outpath = cmd.get_one::<String>("name").expect("Name is required");

    let path = Path::new(&path_arg);

    ancli_poems_rs::create_deck(ancli_poems_rs::get_contents(&path), outpath).unwrap();
}
