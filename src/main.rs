use colored::{Color, Colorize};
use serde::{Deserialize, Serialize};

mod extra;
mod getting;
mod search;

#[derive(Serialize, Deserialize)]
pub struct SessionConfig {
    pub book_name: String,
    pub book_id: String,
    pub chapter_num: u64,
    pub color: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            book_name: String::from(""),
            book_id: String::from(""),
            chapter_num: 0,
            color: String::from("white"),
        }
    }
}

impl Clone for SessionConfig {
    fn clone(&self) -> Self {
        Self {
            book_name: self.book_name.clone(),
            book_id: self.book_id.clone(),
            chapter_num: self.chapter_num.clone(),
            color: self.color.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Chapter {
    title: String,
    order: u64,
    url: String,
}

impl Clone for Chapter {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            order: self.order.clone(),
            url: self.url.clone(),
        }
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let mut config: SessionConfig;
    loop {
        config = confy::load("rrl_cli_reader", "SessionConfig").unwrap_or_default();
        clear();
        let chapters = main_menu(&config);
        if chapters.is_empty() {
            enter_to_continue("No chapters found".to_string());
            continue;
        }
        config = confy::load("rrl_cli_reader", "SessionConfig").unwrap_or_default();
        for chapter in chapters {
            println!("{}", chapter.title);
        }
        std::process::exit(0);
    }
}

fn enter_to_continue(msg: String) {
    println!("{}", format!("{} - Press enter to continue", msg).red());
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn main_menu(config: &SessionConfig) -> Vec<Chapter> {
    let mut input;
    loop {
        clear();
        display_menu();
        input = String::new();
        println!("Enter your choice: ");
        std::io::stdin().read_line(&mut input).unwrap();
        match input.to_uppercase().trim() {
            "Q" => {
                println!("Goodbye!");
                std::process::exit(0);
            }
            "L" => {
                let result = getting::search();
                match result {
                    Some(chapters) => {
                        return chapters;
                    }
                    None => {
                        continue;
                    }
                }
            }
            "C" => {
                change_color(config);
            }
            "P" => {
                if config.book_id == "" {
                    enter_to_continue("No previous book found".to_string());
                } else {
                    return getting::get_chapters(config.book_id.clone(), false).unwrap();
                }
            }

            "B" => {
                input = String::new();
                println!("Enter book id: ");
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let book_id: String = input.trim().to_owned();
                let chapters_result = getting::get_chapters(book_id, true);
                let chapters;
                match chapters_result {
                    Ok(chapters_list) => {
                        chapters = chapters_list;
                    }
                    Err(err_msg) => {
                        match err_msg {
                            0 => {
                                enter_to_continue("Invalid book id".to_string());
                            }
                            1 => {
                                continue;
                            }
                            _ => {
                                enter_to_continue("Failed to fetch chapters".to_string());
                            }
                        }
                        continue;
                    }
                }
                return chapters;
            }

            _ => {
                enter_to_continue("Invalid choice".to_string());
            }
        }
    }
}

fn change_color(config: &SessionConfig) {
    let mut color = String::new();
    loop {
        clear();
        println!("Enter color(default white): ");
        std::io::stdin()
            .read_line(&mut color)
            .expect("Failed to read line");
        let check = colored::Color::from(color.trim());
        match check {
            Color::Black
            | Color::Red
            | Color::Green
            | Color::Yellow
            | Color::Blue
            | Color::Magenta
            | Color::Cyan
            | Color::BrightBlack
            | Color::BrightRed
            | Color::BrightGreen
            | Color::BrightYellow
            | Color::BrightBlue
            | Color::BrightMagenta
            | Color::BrightCyan
            | Color::BrightWhite => {
                break;
            }
            Color::White => match color.trim() {
                "white" => {
                    break;
                }
                _ => {
                    enter_to_continue("Invalid color".to_string());
                }
            },
            _ => {
                enter_to_continue("Invalid color".to_string());
            }
        }
    }
    let new_config = SessionConfig {
        book_name: config.book_name.clone(),
        book_id: config.book_id.clone(),
        chapter_num: config.chapter_num.clone(),
        color: color.trim().to_string(),
    };
    confy::store("rrl_cli_reader", "SessionConfig", new_config).unwrap();
}

fn display_menu() {
    clear();
    println!(
        "{}",
        "Welcome to the Royal Road CLI Reader!".yellow().bold()
    );
    println!("P: Continue previous book");
    println!("B: Load book");
    println!("C: Change colour of text");
    println!("L: Browse books");
    println!("Q: Quit");
}
