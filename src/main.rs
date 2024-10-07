use clap::{Arg, ArgGroup, Command};
use colored::*;

mod base;
mod base_conversion;
mod md5;

mod tests;

fn main() {
    let app = Command::new("Wutong")
        .version("0.1.0-alpha")
        .author("Gavin Zheng <gav.zheng@outlook.com>")
        .about("Wutong - A Swiss Army Knife of Developers.")
        .subcommand(
            Command::new("md5").about("MD5 hash calculation").arg(
                Arg::new("text")
                    .short('t')
                    .long("text")
                    .help("Text to be hashed"),
            ),
        )
        .subcommand(
            Command::new("bc")
                .about("Base Conversion")
                .arg(
                    Arg::new("binary")
                        .long("bin")
                        .help("The Binary to be converted."),
                )
                .arg(
                    Arg::new("octal")
                        .long("oct")
                        .help("The Octal to be converted."),
                )
                .arg(
                    Arg::new("decimal")
                        .long("dec")
                        .help("The Decimal to be converted."),
                )
                .arg(
                    Arg::new("hexadecimal")
                        .long("hex")
                        .help("The Hexadecimal to be converted."),
                )
                .group(
                    ArgGroup::new("base_conversion_options")
                        .args(["binary", "octal", "decimal", "hexadecimal"])
                        .required(true)
                        .multiple(false),
                ),
        )
        .subcommand(
            Command::new("base")
                .about("Encoding in Base16 and Base64")
                .arg(
                    Arg::new("encode")
                        .short('e')
                        .long("encode")
                        .help("Encode the text using Base encoding.")
                        .required(false)
                        .action(clap::ArgAction::SetTrue), // .conflicts_with("decode"),
                )
                /*
                todo: Add decode
                .arg(
                    Arg::new("decode")
                        .short('d')
                        .long("decode")
                        .help("Decode the text using Base decoding.")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .conflicts_with("encode"),
                )*/
                .arg(
                    Arg::new("text")
                        .short('t')
                        .long("text")
                        .help("Text to be encoded or decoded.")
                        .required_unless_present_any(["encode"]), // .conflicts_with("file"),
                ), /*
                   todo: Add file

                      .arg(
                          Arg::new("file")
                              .short('f')
                              .long("file")
                              .help("File to be encoded or decoded.")
                              .required_unless_present_any(["encode", "decode"])
                              .conflicts_with("text"),
                      ),*/
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("md5", subcommand_md5)) => {
            let text = subcommand_md5.get_one::<String>("text").unwrap();
            println!("{}", md5::md5_text::md5_text(text.to_string()));
        }
        Some(("bc", subcommand_bc)) => {
            match () {
                _ if subcommand_bc.contains_id("binary") => {
                    let result = base_conversion::math::binary(
                        subcommand_bc.get_one::<String>("binary").unwrap(),
                    );
                    println!(
                        "Binary:      {} \nOctal:       {} \nDecimal:     {} \nHexadecimal: {}",
                        subcommand_bc.get_one::<String>("binary").unwrap(), // Binary
                        result[0],                                          // Octal
                        result[1],                                          // Decimal
                        result[2]                                           // Hexadecimal
                    )
                }
                _ if subcommand_bc.contains_id("octal") => {
                    let result = base_conversion::math::octal(
                        subcommand_bc.get_one::<String>("octal").unwrap(),
                    );

                    println!(
                        "Binary:      {}\nOctal:       {}\nDecimal:     {} \nHexadecimal: {}",
                        result[0],                                         // Binary
                        subcommand_bc.get_one::<String>("octal").unwrap(), // Octal
                        result[1],                                         // Decimal
                        result[2]                                          // Hexadecimal
                    );
                }
                _ if subcommand_bc.contains_id("decimal") => {
                    let result = base_conversion::math::decimal(
                        subcommand_bc.get_one::<String>("decimal").unwrap(),
                    );
                    println!(
                        "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                        result[0],                                           // Binary
                        result[1],                                           // Octal
                        subcommand_bc.get_one::<String>("decimal").unwrap(), // Decimal
                        result[2]                                            // Hexadecimal
                    );
                }
                _ if subcommand_bc.contains_id("hexadecimal") => {
                    let result = base_conversion::math::hexadecimal(
                        subcommand_bc.get_one::<String>("hexadecimal").unwrap(),
                    );
                    println!(
                        "Binary:      {}\nOctal:       {}\nDecimal:     {}\nHexadecimal: {}",
                        result[0],                                               // Binary
                        result[1],                                               // Octal
                        result[2],                                               // Decimal
                        subcommand_bc.get_one::<String>("hexadecimal").unwrap()  // Hexadecimal
                    )
                }
                _ => panic!("Invalid base conversion option"),
            };
        }
        Some(("base", subcommand_base)) => match () {
            _ if subcommand_base.contains_id("encode") => {
                if subcommand_base.contains_id("text") {
                    match base::base_text::base_encode_text(
                        subcommand_base.get_one::<String>("text").unwrap(),
                    ) {
                        Ok(result) => {
                            println!("base16: {} \nbase64: {}", result[0], result[1])
                        }
                        Err(error) => println!(
                            "{} {}. \n\nFor more information, try '--help'.",
                            "error:".red(),
                            error
                        ),
                    }
                }
            }
            /*
            todo: Add decode
            _ if subcommand_base.contains_id("decode") => {
                if subcommand_base.contains_id("text") {
                    }
                }
            }
            */
            _ => panic!("Invalid base encoding option"),
        },
        _ => {
            println!("Invalid command")
        }
    };
}
