// This file is part of the sanctity source code.
//
// ©️ 2022 papojari <mailto:papojari-git.ovoid@aleeas.com> <https://matrix.to/#/@papojari:artemislena.eu> <https://papojari.codeberg.page>
//
// For the license information, please view the README.md file that was distributed with this source code.

use clap::Parser;
use recolored::*;

#[derive(Parser, Debug)]
#[clap(name = "sanctity", version, about="ruSt ANsi16 Color Test utIliTY")]
struct Args {
    /// Text that will be used to test the colors.
    #[clap(long, short, default_value = " ••• ")]
    text: String,

    /// Print the background colors in either columns or rows.
    #[clap(long, short, possible_values = ["column", "row"], default_value = "column")]
    layout: String,

    /// Select which background colors to print. "all" is not a color. It means all colors.
    #[clap(long, short, possible_values = ["black", "bright_black", "red", "bright_red", "green", "bright_green", "yellow", "bright_yellow", "blue", "bright_blue", "magenta", "bright_magenta", "cyan", "bright_cyan", "white", "bright_white", "all"], default_value = "all")]
    bgcolor: String,
}

struct BgColors {
    black: bool,
    bright_black: bool,
    red: bool,
    bright_red: bool,
    green: bool,
    bright_green: bool,
    yellow: bool,
    bright_yellow: bool,
    blue: bool,
    bright_blue: bool,
    magenta: bool,
    bright_magenta: bool,
    cyan: bool,
    bright_cyan: bool,
    white: bool,
    bright_white: bool,
}

fn main() {
    let args = Args::parse();
    let text = args.text.as_str();
    let layout = args.layout;
    let bgcolor = args.bgcolor.as_str();

    let bgcolor_match = match bgcolor {
        "black" => BgColors {
            black: true,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "bright_black" => BgColors {
            black: false,
            bright_black: true,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "red" => BgColors {
            black: false,
            bright_black: false,
            red: true,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "bright_red" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: true,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "green" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: true,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "bright_green" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: true,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "yellow" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: true,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "bright_yellow" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: true,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "blue" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: true,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "bright_blue" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: true,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "magenta" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: true,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "bright_magenta" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: true,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "cyan" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: true,
            bright_cyan: false,
            white: false,
            bright_white: false,
        },
        "bright_cyan" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: true,
            white: false,
            bright_white: false,
        },
        "white" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: true,
        },
        "bright_white" => BgColors {
            black: false,
            bright_black: false,
            red: false,
            bright_red: false,
            green: false,
            bright_green: false,
            yellow: false,
            bright_yellow: false,
            blue: false,
            bright_blue: false,
            magenta: false,
            bright_magenta: false,
            cyan: false,
            bright_cyan: false,
            white: false,
            bright_white: true,
        },
        "all" => BgColors {
            black: true,
            bright_black: true,
            red: true,
            bright_red: true,
            green: true,
            bright_green: true,
            yellow: true,
            bright_yellow: true,
            blue: true,
            bright_blue: true,
            magenta: true,
            bright_magenta: true,
            cyan: true,
            bright_cyan: true,
            white: true,
            bright_white: true,
        },
        _ => panic!("Invalid input: {bgcolor}"),
    };

    // Set background colors based on `--bgcolor` input.
    let mut bg_ansi16 = Vec::new();

    if bgcolor_match.black {
        bg_ansi16.push(Color::Black);
    };
    if bgcolor_match.bright_black {
        bg_ansi16.push(Color::BrightBlack);
    };
    if bgcolor_match.red {
        bg_ansi16.push(Color::Red);
    };
    if bgcolor_match.bright_red {
        bg_ansi16.push(Color::BrightRed);
    };
    if bgcolor_match.green {
        bg_ansi16.push(Color::Green);
    };
    if bgcolor_match.bright_green {
        bg_ansi16.push(Color::BrightGreen);
    };
    if bgcolor_match.yellow {
        bg_ansi16.push(Color::Yellow);
    };
    if bgcolor_match.bright_yellow {
        bg_ansi16.push(Color::BrightYellow);
    };
    if bgcolor_match.blue {
        bg_ansi16.push(Color::Blue);
    };
    if bgcolor_match.bright_blue {
        bg_ansi16.push(Color::BrightBlue);
    };
    if bgcolor_match.magenta {
        bg_ansi16.push(Color::Magenta);
    };
    if bgcolor_match.bright_magenta {
        bg_ansi16.push(Color::BrightMagenta);
    };
    if bgcolor_match.cyan {
        bg_ansi16.push(Color::Cyan);
    };
    if bgcolor_match.bright_cyan {
        bg_ansi16.push(Color::BrightCyan);
    };
    if bgcolor_match.white {
        bg_ansi16.push(Color::White);
    };
    if bgcolor_match.bright_white {
        bg_ansi16.push(Color::BrightWhite);
    };

    // Define foreground colors.
    let fg_ansi16 = [
        (Color::Black),
        (Color::BrightBlack),
        (Color::Red),
        (Color::BrightRed),
        (Color::Green),
        (Color::BrightGreen),
        (Color::Yellow),
        (Color::BrightYellow),
        (Color::Blue),
        (Color::BrightBlue),
        (Color::Magenta),
        (Color::BrightMagenta),
        (Color::Cyan),
        (Color::BrightCyan),
        (Color::White),
        (Color::BrightWhite),
    ];

    // Print the test with the background colors in column or rows.
    if layout == "column" {
        // Do the below comment but 16 times with a different color for the text each time.
        for color in fg_ansi16.clone() {
            // Print a row of text in one color with every background color.
            let column_color = color;
            for color in bg_ansi16.clone() {
                print!("{}", text.clone().on_color(color).color(column_color));
            };
            println!();
        };
    } else {
        // Do the below comment but 16 times with a different background color each time.
        for color in bg_ansi16.clone() {
            // Print a row of text in every color with only one background color.
            let column_color = color;
            for color in fg_ansi16.clone() {
                print!("{}", text.clone().on_color(column_color).color(color));
            };
            println!();
        };
    };
}
