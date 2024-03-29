use crate::{clear, enter_to_continue, extra, getting};
use colored::Colorize;
use soup::{NodeExt, QueryBuilderExt};

struct Fiction {
    id: String,
    title: String,
    tags: Vec<String>,
    description: String,
    pages: u64,
    chapters: u64,
    rating: f32,
    views: u64,
}

pub fn search_by_tag() -> Option<(String, String)> {
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

        let option = getting::get_input("Enter the number of the tag you want to search for, use '-' to exclude that tag e.g. -6 & use the id number again to remove it from the search - enter search to continue (exit to go back): ");
        if option == "exit" {
            return None;
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

    Some((
        format!("Tags: {}", search_title.join(", ")),
        search_msg.join("&"),
    ))
}

pub fn search_by_title() -> Option<(String, String)> {
    let title: String;
    clear();
    title = getting::get_input(
        "Enter the title of the fiction you want to search for(exit to go back)",
    );
    if title == "exit" {
        return None;
    }
    let url_segment = format!("title={}", title);
    return Some((format!("Title: {}", title), url_segment));
}

pub fn search_by_rating() -> Option<(String, String)> {
    let mut lower_bound: f32;
    let mut upper_bound: f32;

    loop {
        clear();
        let input = getting::get_input(
            "Enter the lower bound of the rating you want to search for(lowest is 0, highest is 5, default is 0, exit to go back)",
        );
        if input == "exit" {
            return None;
        } else if input == "" {
            lower_bound = 0.0;
            break;
        }
        let lower_bound_temp = input.parse::<f32>();
        if lower_bound_temp.is_err() {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        lower_bound = lower_bound_temp.unwrap();
        if lower_bound < 0.0 || lower_bound > 5.0 {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        lower_bound = (lower_bound * 100.0).round() / 100.0;
        break;
    }
    loop {
        clear();
        let input = getting::get_input(
            "Enter the upper bound of the rating you want to search for(lowest is 0, highest is 5, default is 5, exit to go back)",
        );
        if input == "exit" {
            return None;
        } else if input == "5" {
            upper_bound = 5.0;
            break;
        }
        let upper_bound_temp = input.parse::<f32>();
        if upper_bound_temp.is_err() {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        upper_bound = upper_bound_temp.unwrap();
        if upper_bound < 0.0 || upper_bound > 5.0 {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        } else if upper_bound < lower_bound {
            println!("Upper bound must be greater than lower bound - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        upper_bound = (upper_bound * 100.0).round() / 100.0;
        break;
    }
    let url_segment = format!("minRating={}&maxRating={}", lower_bound, upper_bound);

    Some((
        format!("{} stars to {} stars", lower_bound, upper_bound),
        url_segment,
    ))
}

pub fn search_by_keywords() -> Option<(String, String)> {
    let keywords: String;
    clear();
    keywords = getting::get_input(
        "Enter the keywords of the fiction you want to search for(exit to go back)",
    );
    if keywords == "exit" {
        return None;
    }
    let url_segment = format!("keyword={}", keywords);
    Some((keywords, url_segment))
}

pub fn search_by_author() -> Option<(String, String)> {
    let author: String;
    clear();
    author = getting::get_input(
        "Enter the author of the fiction you want to search for(exit to go back)",
    );
    if author == "exit" {
        return None;
    }
    let url_segment = format!("author={}", author);
    Some((author, url_segment))
}

pub fn search_by_status() -> Option<(String, String)> {
    let types = vec![
        ("Completed", "COMPLETED"),
        ("Ongoing", "ONGOING"),
        ("Hiatus", "HIATUS"),
        ("Dropped", "DROPPED"),
        ("Stub", "STUB"),
    ];
    loop {
        clear();
        println!("Status Types:");
        types.iter().enumerate().for_each(|(i, status_type)| {
            println!("{}: {}", i + 1, status_type.0);
        });
        let option = getting::get_input(
            "Enter the number of the status you want to search for(exit to go back)",
        );
        if option == "exit" {
            return None;
        }
        let option = option.parse::<usize>();
        if option.is_err() {
            enter_to_continue("Invalid input".to_string());
            continue;
        }
        let option = option.unwrap();
        if option > types.len() {
            enter_to_continue("Invalid input".to_string());
            continue;
        } else if option == 0 {
            enter_to_continue("Invalid input".to_string());
            continue;
        }
        let url_segment = format!("status={}", types.get(option - 1).unwrap().1);
        let search_msg = format!("Status: {}", types.get(option - 1).unwrap().0.to_string());

        return Some((search_msg, url_segment));
    }
}

pub fn search_by_pages_amount() -> Option<(String, String)> {
    let lower_bound: u64;
    let mut upper_bound: u64;

    loop {
        let input = getting::get_input(
            "Enter the lower bound of the pages you want to search for(exit to go back)",
        );
        if input == "exit" {
            return None;
        }
        let lower_bound_temp = input.parse::<u64>();
        if lower_bound_temp.is_err() {
            enter_to_continue("Invalid input".to_string());
            continue;
        }
        lower_bound = lower_bound_temp.unwrap();
        break;
    }
    loop {
        clear();
        let input = getting::get_input(
            "Enter the upper bound of the pages you want to search for(exit to go back)",
        );
        if input == "exit" {
            return None;
        }
        let upper_bound_temp = input.parse::<u64>();
        if upper_bound_temp.is_err() {
            enter_to_continue("Invalid input".to_string());
            continue;
        }
        upper_bound = upper_bound_temp.unwrap();
        if upper_bound < lower_bound {
            enter_to_continue("Upper bound must be greater than lower bound".to_string());
            continue;
        }
        break;
    }
    let url_segment = format!("minPages={}&maxPages={}", lower_bound, upper_bound);

    Some((
        format!("Pages from {} to {}", lower_bound, upper_bound),
        url_segment,
    ))
}

pub fn advanced_search() -> Option<(String, String)> {
    let mut attributes = vec![(String::new(), String::new()); 8];
    let mut url_segment = String::new();
    let mut search_title = String::new();
    loop {
        clear();
        println!("Advanced Search:");
        let search_types = vec![
            "Title", "Keywords", "Author", "Tag", "Rating", "Pages", "Status", "Search",
        ];
        attributes.iter().enumerate().for_each(|(i, item)| {
            println!(
                "{}: {} {}",
                i + 1,
                search_types[i],
                if item.1 != "" && item.1 != "0" {
                    format!("- {}", attributes[i].0)
                } else {
                    String::new()
                }
            )
        });
        let option =
            getting::get_input("Enter the number of the option you want to use(exit to go back)");
        if option == "exit" {
            return None;
        }
        let option = option.parse::<usize>();
        if option.is_err() {
            enter_to_continue("Invalid input".to_string());
            continue;
        }
        let option = option.unwrap();
        let result;
        match option {
            1 => result = search_by_title(),
            2 => result = search_by_keywords(),
            3 => result = search_by_author(),
            4 => result = search_by_tag(),
            5 => result = search_by_rating(),
            6 => result = search_by_pages_amount(),
            7 => result = search_by_status(),
            8 => return Some((search_title, url_segment)),
            _ => {
                enter_to_continue("Invalid input".to_string());
                continue;
            }
        };
        if result.is_none() {
            continue;
        }
        let result = result.unwrap();
        attributes.get_mut(option - 1).unwrap().0 = result.0;
        attributes.get_mut(option - 1).unwrap().1 = result.1;
        let mut search_title_temp: Vec<String> = Vec::new();
        let mut url_segment_temp: Vec<String> = Vec::new();
        attributes.clone().iter().for_each(|attribute| {
            if attribute.1 != "0" && attribute.1 != "" {
                search_title_temp.push(attribute.0.clone());
                url_segment_temp.push(attribute.1.clone());
            }
        });
        search_title = search_title_temp.join(", ");
        url_segment = url_segment_temp.join("&");
        let pages: u64 = getting::get_num_of_pages(&url_segment);
        if pages == 0 {
            enter_to_continue("No results matching these criteria were found".to_string());
            continue;
        }
    }
}

pub fn pick_book(result: (String, String), pages: u64, sorting: String) -> bool {
    clear();
    let mut page = 1;
    let mut moved_page;
    loop {
        moved_page = false;
        let url = format!(
            "https://www.royalroad.com/fictions/search?page={}&{}&{}",
            page, result.1, sorting
        );
        let books = get_search_results(&url);
        clear();
        println!(
            "{}{} - Page {} in {}\n",
            "Searching for: ".yellow().bold(),
            result.0.blue().bold(),
            page,
            pages
        );
        let mut input;
        enter_to_continue("Enter the book number to read it, exit to go back".to_string());
        for (i, book) in books.iter().enumerate() {
            input = String::new();
            println!(
                "{}: {}\n{}",
                (i + 1).to_string().blue().bold(),
                book.title.blue().bold(),
                book.description
            );
            println!(
                "\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n",
                "Tags: ".blue().bold(),
                book.tags.join(", "),
                "Pages: ".blue().bold(),
                book.pages,
                "Chapters: ".blue().bold(),
                book.chapters,
                "Rating: ".blue().bold(),
                book.rating,
                "Views: ".blue().bold(),
                book.views
            );
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "exit" {
                return false;
            } else if input.trim() == "" {
                continue;
            } else if input.trim() == ">" {
                if page == pages {
                    enter_to_continue("No more pages".to_string());
                    continue;
                }
                page += 1;
                moved_page = true;
                break;
            } else if input.trim() == "<" {
                if page == 1 {
                    enter_to_continue("No previous pages".to_string());
                    continue;
                }
                page -= 1;
                moved_page = true;
                break;
            } else {
                match input.trim().parse::<usize>() {
                    Ok(num) => {
                        if num > 0 && num <= books.len() {
                            load_book(books.get(num - 1).unwrap());
                            return true;
                        }
                    }
                    Err(_) => {
                        enter_to_continue("Invalid input".to_string());
                        continue;
                    }
                }
            }
        }
        if moved_page {
            continue;
        }
        loop {
            println!("Enter > to go to next page, < to go to previous page, exit to go back or book number to read it");
            input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "exit" {
                return false;
            } else if input.trim() == ">" {
                if page == pages {
                    enter_to_continue("No more pages".to_string());
                    continue;
                }
                page += 1;
                break;
            } else if input.trim() == "<" {
                if page == 1 {
                    enter_to_continue("No previous pages".to_string());
                    continue;
                }
                page -= 1;
                break;
            } else {
                match input.trim().parse::<usize>() {
                    Ok(num) => {
                        if num > 0 && num <= books.len() {
                            load_book(books.get(num - 1).unwrap());
                            return true;
                        }
                    }
                    Err(_) => {
                        enter_to_continue("Invalid input".to_string());
                        continue;
                    }
                }
            }
        }
    }
}

fn load_book(book: &Fiction) {
    getting::get_chapters(book.id.clone(), true).unwrap();
}

fn get_search_results(url: &str) -> Vec<Fiction> {
    let response = reqwest::blocking::get(url).unwrap();
    let soup = soup::Soup::new(&response.text().unwrap());
    let books_unfiltered = soup
        .tag("div")
        .attr("class", "fiction-list-item")
        .find_all();
    let mut books: Vec<Fiction> = vec![];
    for book in books_unfiltered {
        let title = book
            .tag("h2")
            .attr("class", "fiction-title")
            .find()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let id = book
            .tag("a")
            .attr("class", "font-red-sunglo")
            .find()
            .unwrap()
            .get("href")
            .unwrap();
        let id = id
            .split("/")
            .collect::<Vec<&str>>()
            .get(2)
            .unwrap()
            .to_string();
        let desc = book
            .tag("div")
            .attr("id", format!("description-{}", id))
            .find()
            .unwrap()
            .text()
            .trim()
            .to_string();
        let tags = book
            .tag("span")
            .attr("class", "tags")
            .find()
            .unwrap()
            .tag("a")
            .find_all()
            .into_iter()
            .map(|tag| tag.text().trim().to_string())
            .collect::<Vec<String>>();
        let row_stats = book
            .tag("div")
            .attr("class", "stats")
            .find()
            .unwrap()
            .tag("div")
            .find_all()
            .map(|tag| tag.text().trim().to_string())
            .collect::<Vec<String>>();
        let pages = row_stats
            .get(3)
            .unwrap()
            .replace(" Pages", "")
            .replace(",", "")
            .parse::<u64>()
            .unwrap();
        let views = row_stats
            .get(4)
            .unwrap()
            .replace(" Views", "")
            .replace(",", "")
            .parse::<u64>()
            .unwrap();
        let chapters = row_stats
            .get(5)
            .unwrap()
            .replace(" Chapters", "")
            .replace(",", "")
            .parse::<u64>()
            .unwrap();
        let ratings = book
            .tag("span")
            .attr("class", "star")
            .find()
            .unwrap()
            .get("title")
            .unwrap()
            .parse::<f32>()
            .unwrap();

        books.push(Fiction {
            id,
            title,
            tags,
            description: desc,
            pages: pages,
            chapters: chapters,
            rating: ratings,
            views: views,
        });
    }
    books
}
