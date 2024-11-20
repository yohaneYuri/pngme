use std::path::PathBuf;

use clap::{arg, ArgMatches, Command};

use pngme::{args::*, commands::*};

fn main() {
    let matches = build_command().get_matches();

    if let Some(e) = match matches.subcommand() {
        Some(("encode", sub_matches)) => {
            let file_path = get_required_string_arg(sub_matches, "FILE");
            let chunk_type = get_required_string_arg(sub_matches, "TYPE");
            let message = get_required_string_arg(sub_matches, "MESSAGE");
            let output_file = sub_matches.get_one::<String>("OUTPUT")
                .cloned()
                .map(|s| PathBuf::from(s));

            encode(EncodeArgs {
                file_path: PathBuf::from(file_path),
                chunk_type,
                message,
                output_file,
            }).err()
        },
        Some(("decode", sub_matches)) => {
            let file_path = get_required_string_arg(sub_matches, "FILE");
            let chunk_type = get_required_string_arg(sub_matches, "TYPE");

            decode(DecodeArgs {
                file_path: PathBuf::from(file_path),
                chunk_type,
            }).err()
        },
        Some(("remove", sub_matches)) => {
            let file_path = get_required_string_arg(sub_matches, "FILE");
            let chunk_type = get_required_string_arg(sub_matches, "TYPE");

            remove(RemoveArgs {
                file_path: PathBuf::from(file_path),
                chunk_type,
            }).err()
        },
        Some(("print", sub_matches)) => {
            let file_path = get_required_string_arg(sub_matches, "FILE");

            print(PrintArgs { file_path: PathBuf::from(file_path) }).err()
        },
        _ => unreachable!("Exhaust all possibilities"),
    } {
        eprint!("{}", e);
    }
}

fn build_command() -> Command {
    Command::new("Png")
        .version("0.0.1")
        .about("A program help you hide secrets in png file")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encode")
                .about("Encode a string into a chunk with specific chunk type")
                .arg(arg!(<FILE> "Name of the file to be encoded"))
                .arg(arg!(<TYPE> "Chunk type"))
                .arg(arg!(<MESSAGE> "What you want to hide"))
                .arg(arg!([OUTPUT] "Name of the output file"))
        )
        .subcommand(
            Command::new("decode")
                .about("Extract the data of specific chunk with chunk type, display which with UTF-8")
                .arg(arg!(<FILE> "Name of the file to be decoded"))
                .arg(arg!(<TYPE> "Chunk type"))
        )
        .subcommand(
            Command::new("remove")
                .about("Remove the first chunk with specific chunk type")
                .arg(arg!(<FILE> "Name of the file to be parsed"))
                .arg(arg!(<TYPE> "Chunk type"))
        )
        .subcommand(
            Command::new("print")
                .about("Print all data in the file")
                .arg(arg!(<FILE> "Name of the file to be parsed"))
        )
}

fn get_required_string_arg(sub_matches: &ArgMatches, arg_name: &str) -> String {
    sub_matches.get_one::<String>(arg_name)
            .expect("Error: missing file agruement")
            .to_owned()
}
