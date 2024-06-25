use colored::Colorize;

use crate::Config;

#[derive(Debug)]
struct TextComponent {
    pub text: String,
    pub color: String,
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
}

impl TextComponent {
    pub fn print(&self) {
        let text = &self.text;
        let mut colored = match self.color.as_str() {
            "black" => text.black(),
            "dark_blue" => text.blue(),
            "dark_green" => text.green(),
            "dark_aqua" => text.cyan(),
            "dark_red" => text.red(),
            "dark_purple" => text.purple(),
            "gold" => text.yellow(),
            "gray" => text.normal(),
            "dark_gray" => text.truecolor(21, 21, 21),
            "blue" => text.bright_blue(),
            "green" => text.bright_green(),
            "aqua" => text.bright_cyan(),
            "red" => text.bright_red(),
            "light_purple" => text.bright_purple(),
            "yellow" => text.bright_yellow(),
            "white" => text.white(),
            _ => {
                println!("{} {}", "Invalid color found".red(), self.color);
                text.normal()
            }
        };
        if self.bold {
            colored = colored.bold();
        }
        if self.italic {
            colored = colored.italic();
        }
        if self.underlined {
            colored = colored.underline();
        }
        if self.strikethrough {
            colored = colored.strikethrough();
        }
        print!("{}", colored);
    }
}

pub fn print_motd(config: Config) {
    let mut chars = config.motd.chars();
    chars.next();
    chars.next_back();
    chars.next_back();
    let mut texts = vec![];
    for x in chars.as_str().split("},") {
        let mut chars = x.chars();
        chars.next();
        let components = chars.as_str().split(",");
        let mut text = TextComponent {
            text: "".to_owned(),
            color: "gray".to_owned(),
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
        };
        for y in components {
            let kv: Vec<&str> = y.split(":").collect();
            let mut key = kv[0].chars();
            key.next();
            key.next_back();
            let key = key.as_str();
            let value = kv[1];

            match key {
                "text" => {
                    let mut chars = value.chars();
                    chars.next();
                    chars.next_back();
                    text.text = chars.as_str().to_string();
                }
                "color" => {
                    let mut chars = value.chars();
                    chars.next();
                    chars.next_back();
                    text.color = chars.as_str().to_string();
                }
                "bold" => match value {
                    "true" => text.bold = true,
                    "false" => text.bold = false,
                    _ => println!("Unknown boolean value {}", value),
                },
                "italic" => match value {
                    "true" => text.italic = true,
                    "false" => text.italic = false,
                    _ => println!("Unknown boolean value {}", value),
                },
                "underlined" => match value {
                    "true" => text.underlined = true,
                    "false" => text.underlined = false,
                    _ => println!("Unknown boolean value {}", value),
                },
                "strikethrough" => match value {
                    "true" => text.strikethrough = true,
                    "false" => text.strikethrough = false,
                    _ => println!("Unknown boolean value {}", value),
                },
                "obfuscated" => match value {
                    "true" => text.obfuscated = true,
                    "false" => text.obfuscated = false,
                    _ => println!("Unknown boolean value {}", value),
                },
                _ => println!("Didn't recognise key {} with value {}.", key, value),
            }
        }
        texts.push(text);
    }
    for text in texts {
        text.print();
    }
    print!("\n");
}
