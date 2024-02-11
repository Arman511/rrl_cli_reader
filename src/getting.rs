use crate::{clear, enter_to_continue, extra, search, Chapter, SessionConfig};
use colored::Colorize;
use soup::{NodeExt, QueryBuilderExt, Soup};
struct Fiction {
    id: u64,
    title: String,
    tags: Vec<String>,
    description: String,
    pages: u64,
    chapters: u64,
    rating: f32,
    views: u64,
}
pub fn search() -> Option<Vec<Chapter>> {
    loop {
        clear();
        println!("1: Advanced Search");
        println!("2: Search by title");
        println!("3: Search by keywords");
        println!("4: Search by author");
        println!("5: Search by tag");
        println!("6: Search by rating");
        println!("7: Search by amount of pages");
        println!("8: Search by status");
        println!("9: Go back");
        let mut option = String::new();
        println!("Enter option: ");
        std::io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        option = option.trim().to_string();
        let title;
        let url_segment;
        match option.as_str() {
            //"1" => (title, url_segment) = advanced_search(),
            "2" => (title, url_segment) = search::search_by_title(),
            //"3" => (title, url_segment) = search_by_keywords(),
            // "4" => (title, url_segment) = search_by_author(),
            "5" => (title, url_segment) = search::search_by_tag(),
            // "6" => (title, url_segment) = search_by_rating(),
            // "7" => (title, url_segment) = search_by_pages(),
            // "8" => (title, url_segment) = search_by_status(),
            "9" => return None,
            _ => {
                enter_to_continue("Invalid choice".to_string());
                continue;
            }
        }
    }
    let sorting = extra::get_sorting();
    None
}

pub fn get_input(msg: &str) -> String {
    let mut input = String::new();
    loop {
        println!("{}", msg);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        return input.trim().to_string();
    }
}

pub fn get_chapters(book_id: String, new_book: bool) -> Result<Vec<Chapter>, u8> {
    let url = format!("https://www.royalroad.com/fiction/{}", book_id);
    let response = reqwest::blocking::get(&url).unwrap();

    let soup = Soup::new(&response.text().unwrap());
    if soup
        .tag("div")
        .attr("class", "number font-red-sunglo")
        .find()
        .is_some()
    {
        return Err(0);
    }

    let chapters_result = soup
        .tag("table")
        .attr("id", "chapters")
        .find()
        .unwrap()
        .tag("a")
        .find_all();
    let mut chapters = Vec::new();
    for (i, chapter) in chapters_result.enumerate() {
        if i % 2 == 1 {
            continue;
        }
        let title = chapter.text().trim().to_string();
        let url = chapter.get("href").unwrap();
        chapters.push(Chapter {
            title,
            order: i as u64,
            url: url.to_string(),
        });
    }
    let title = soup.tag("h1").find().unwrap().text();
    let current_config: SessionConfig =
        confy::load("rrl_cli_reader", "SessionConfig").unwrap_or_default();

    let new_chapter_num = if new_book {
        which_chapter(&chapters, title.clone())
    } else {
        Some(0)
    };
    if new_chapter_num.is_none() {
        return Err(1);
    }
    let new_chapter_num = new_chapter_num.unwrap();

    let new_config = SessionConfig {
        book_name: title,
        book_id,
        chapter_num: new_chapter_num,
        color: current_config.color.clone(),
    };
    confy::store("rrl_cli_reader", "SessionConfig", new_config).unwrap();

    Ok(chapters)
}

pub fn which_chapter(chapters: &Vec<Chapter>, title: String) -> Option<u64> {
    let mut input;
    loop {
        clear();
        println!(
            "{}",
            format!("{} - Choose a chapter to read", title)
                .blue()
                .bold()
        );
        for (i, chapter) in chapters.iter().enumerate() {
            println!("{}: {}", (i + 1).to_string().blue().bold(), chapter.title);
        }

        input = String::new();
        println!("Enter the number of the chapter you want to read(default is first one, exit to go back): ");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            return Some(0);
        } else if input.trim().to_uppercase() == "EXIT" {
            return None;
        }

        match input.trim().parse::<u64>() {
            Ok(num) => {
                if num > 0 && num <= chapters.len() as u64 {
                    return Some(num - 1);
                }
            }
            Err(_) => {
                enter_to_continue("Invalid input".to_string());
                continue;
            }
        }
    }
}
