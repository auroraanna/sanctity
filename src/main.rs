use clap::Parser;
use colorful::Colorful;
use colorful::Color;

#[derive(Parser, Debug)]
#[clap(name = "sanctity", version, about="ruSt ANsi16 Color Test utIliTY")]
struct Args {
    /// Text that will be used to test the colors.
    #[clap(long, short, default_value = " ••• ")]
    text: String,
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

    for color in ansi16 {
        let column_color = color;
        for color in ansi16 {
            print!("{}", text.clone().bg_color(color).color(column_color));
        };
        println!();
    };
}
