use clap::{Arg, ArgGroup, Command};

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
        _ => {}
    }
}
