// rust (port) of the figlet program https://github.com/cmatsuoka/figlet
use clap::{App, Arg};
use figlet_rs::FIGfont;
fn main() {
    let matches =
        App::new("figlet")
            .version("v0.1.0")
            .author("Tricked-dev <tricked@tricked.pro>")
            .arg(
                Arg::new("MESSAGE")
                    .help("Text to be figgeld")
                    .multiple_values(true),
            )
            .arg(Arg::new("fontfile").short('f').help(
                "Change the font file. for more awesome fonts: http://www.figlet.org/fontdb.cgi",
            ))
            .get_matches();
    let font = if let Some(fontfile) = matches.value_of("fontfile") {
        FIGfont::from_file(fontfile).expect("Font file not valid or not found!")
    } else {
        FIGfont::standand().unwrap()
    };

    let figure = font.convert(
        &matches
            .values_of("MESSAGE")
            .unwrap()
            .collect::<Vec<&str>>()
            .join(" "),
    );
    println!("{}", figure.unwrap());
}
