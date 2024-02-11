use crate::{clear, extra, getting::get_input};
use colored::Colorize;

pub fn search_by_tag() -> (String, String) {
    let mut tags: Vec<(&str, &str, i32)> = extra::get_tags();

    loop {
        clear();
        println!("Tags:");
        tags.iter().enumerate().for_each(|(i, tag)| {
            print!("{}: {}", i + 1, tag.0);
            if &tag.2 == &-1 {
                print!("{}", " - excluded".red().bold());
            } else if &tag.2 == &1 {
                print!("{}", " - included".blue().bold())
            };
            println!();
        });

        let option = get_input("Enter the number of the tag you want to search for, use '-' to exclude that tag e.g. -6 & use the id number again to remove it from the search - enter search to continue (exit to go back): ");
        if option == "exit" {
            return ("0".to_string(), "0".to_string());
        } else if option == "search" {
            break;
        }
        match option.parse::<i32>() {
            Ok(tag_id) => {
                let tag = tag_id.abs() as usize;
                if tag == 0 {
                    println!("Invalid input - press enter to continue");
                    std::io::stdin().read_line(&mut String::new()).unwrap();
                    continue;
                }
                if tag > tags.len() {
                    println!("Invalid input - press enter to continue");
                    std::io::stdin().read_line(&mut String::new()).unwrap();
                    continue;
                }
                let current_tag_option = tags.get(tag - 1).unwrap().2;
                if current_tag_option != 0 && current_tag_option == tag_id / tag as i32 {
                    tags.get_mut(tag - 1).unwrap().2 = 0;
                } else {
                    tags.get_mut(tag - 1).unwrap().2 = tag_id / tag as i32;
                }
            }
            Err(_) => {
                println!("Invalid input - press enter to continue");
                std::io::stdin().read_line(&mut String::new()).unwrap();
                continue;
            }
        }
    }
    let mut search_msg: Vec<String> = vec![];
    let mut search_title: Vec<String> = vec![];
    for tag in tags.clone() {
        if tag.2 == 0 {
            continue;
        }
        if tag.2 > 0 {
            search_msg.push(format!("tagsAdd={}", tag.1));
            search_title.push(format!("{} - Included", tag.0));
        } else {
            search_msg.push(format!("tagsRemove={}", tag.1));
            search_title.push(format!("{} - Excluded", tag.0));
        }
    }
    return (
        format!("Tags: {}", search_title.join(", ")),
        search_msg.join("&"),
    );
}

pub fn search_by_title() -> (String, String) {
    let title: String;
    clear();
    title = get_input("Enter the title of the fiction you want to search for(exit to go back)");
    if title == "exit" {
        return ("0".to_string(), "0".to_string());
    }
    let url_segment = format!("title={}", title);
    return (format!("Title: {}", title), url_segment);
}
