use crate::{clear, enter_to_continue, extra, search, Chapter, SessionConfig};
use colored::Colorize;
use soup::{NodeExt, QueryBuilderExt, Soup};

pub fn search() -> Option<Vec<Chapter>> {
    let mut result;
    let sorting = extra::get_sorting();
    if sorting.is_none() {
        return None;
    }
    let mut pages: u64;
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
        match option.as_str() {
            "1" => result = search::advanced_search(),
            "2" => result = search::search_by_title(),
            "3" => result = search::search_by_keywords(),
            "4" => result = search::search_by_author(),
            "5" => result = search::search_by_tag(),
            "6" => result = search::search_by_rating(),
            "7" => result = search::search_by_pages_amount(),
            "8" => result = search::search_by_status(),
            "9" => return None,
            _ => {
                enter_to_continue("Invalid choice".to_string());
                continue;
            }
        }
        if result.is_none() {
            return None;
        }

        pages = get_num_of_pages(&result.clone().unwrap().1);
        if pages == 0 {
            enter_to_continue("No results found".to_string());
            continue;
        }
        break;
    }
    let sorting = sorting.unwrap();
    let book_decided = search::pick_book(result.unwrap(), pages, sorting);
    if !book_decided {
        return None;
    }
    let config: SessionConfig = confy::load("rrl_cli_reader", "SessionConfig").unwrap_or_default();
    let chapters = get_chapters(config.book_id, false);
    if chapters.is_err() {
        enter_to_continue("No chapters found".to_string());
        return None;
    }
    Some(chapters.unwrap())
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

pub fn get_num_of_pages(url_segment: &str) -> u64 {
    let response = reqwest::blocking::get(format!(
        "https://www.royalroad.com/fictions/search?{}",
        url_segment
    ))
    .unwrap();
    let soup = Soup::new(&response.text().unwrap());
    if soup
        .tag("div")
        .attr("class", "search-container")
        .find()
        .unwrap()
        .text()
        .contains("No results matching ")
    {
        return 0;
    }
    if soup.tag("a").attr_name("data-page").find().is_none() {
        return 1;
    }
    soup.tag("a")
        .attr_name("data-page")
        .find_all()
        .last()
        .unwrap()
        .get("data-page")
        .unwrap()
        .parse::<u64>()
        .unwrap()
}
