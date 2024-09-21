use clap::{Arg, Command};

mod md5;
mod tests;

fn main() {
    let app = Command::new("Wutong")
        .version("0.1.0-alpha")
        .author("Gavin Zheng <gav.zheng@outlook.com>")
        .about("Wutong - A Swiss Army Knife of Developers.")
        .subcommand(
            Command::new("md5")
                .about("MD5 Hash")
                .arg(
                    Arg::new("text")
                        .short('t')
                        .long("text")
                        .help("Text to be hashed"),
                ),
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("md5", add_md5)) => {
            let text = add_md5.get_one::<String>("text").unwrap();
            println!("{}", md5::md5_text::md5_text(text.to_string()));
        }
        _ => {}
    }
}
