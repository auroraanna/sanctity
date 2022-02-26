// This file is part of the sanctity source code.
//
// ©️ 2022 papojari <mailto:papojari-git.ovoid@aleeas.com> <https://matrix.to/#/@papojari:artemislena.eu> <https://papojari.codeberg.page>
//
// For the license information, please view the README.md file that was distributed with this source code.

use clap::Parser;
use colorful::Colorful;
use colorful::Color;

#[derive(Parser, Debug)]
#[clap(name = "sanctity", version, about="ruSt ANsi16 Color Test utIliTY")]
struct Args {
    /// Text that will be used to test the colors.
    #[clap(long, short, default_value = " ••• ")]
    text: String,

    /// Print the background colors in either columns or rows.
    #[clap(long, short, possible_values = ["column", "row"], default_value = "column")]
    layout: String,
}

fn main() {
    let args = Args::parse();
    let text = args.text;

    let ansi16 = [
        (Color::Black),
        (Color::DarkGray),
        (Color::Red),
        (Color::LightRed),
        (Color::Green),
        (Color::LightGreen),
        (Color::Yellow),
        (Color::LightYellow),
        (Color::Blue),
        (Color::LightBlue),
        (Color::Magenta),
        (Color::LightMagenta),
        (Color::Cyan),
        (Color::LightCyan),
        (Color::LightGray),
        (Color::White),
    ];

    let layout = args.layout;

    if layout == "column" {
        for color in ansi16 {
            let column_color = color;
            for color in ansi16 {
                print!("{}", text.clone().bg_color(color).color(column_color));
            };
            println!();
        };
    } else {
        for color in ansi16 {
            let column_color = color;
            for color in ansi16 {
                print!("{}", text.clone().bg_color(column_color).color(color));
            };
            println!();
        };
    };
}
