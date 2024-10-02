use clap::{Arg, ArgGroup, Command};

mod base_conversion;
mod md5;
mod tests;
mod wutong_dev;

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
                        .args(&["binary", "octal", "decimal", "hexadecimal"])
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
            enum BaseConversion {
                Binary,
                Octal,
                Decimal,
                Hexadecimal,
            }
            let base= match () {
                _ if (*subcommand_bc).contains_id("binary") => BaseConversion::Binary,
                _ if (*subcommand_bc).contains_id("octal") => BaseConversion::Octal,
                _ if (*subcommand_bc).contains_id("decimal") => BaseConversion::Decimal,
                _ if (*subcommand_bc).contains_id("hexadecimal") => BaseConversion::Hexadecimal,
                _ => panic!("Invalid base conversion option"),
            };

            let result = match base {
                BaseConversion::Binary => base_conversion::math::example(
                    subcommand_bc
                        .get_one::<String>("binary")
                        .unwrap()
                        .to_string(),
                ),
                BaseConversion::Octal => base_conversion::math::example(
                    subcommand_bc
                        .get_one::<String>("octal")
                        .unwrap()
                        .to_string(),
                ),
                BaseConversion::Decimal => base_conversion::math::example(
                    subcommand_bc
                        .get_one::<String>("decimal")
                        .unwrap()
                        .to_string(),
                ),
                BaseConversion::Hexadecimal => base_conversion::math::example(
                    subcommand_bc
                        .get_one::<String>("hexadecimal")
                        .unwrap()
                        .to_string(),
                ),
            };
            println!("{}", result);
        }
        _ => {}
    }
}
