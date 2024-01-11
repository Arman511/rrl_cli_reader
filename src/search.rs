use crate::get_chapters;
use crate::SessionConfig;
use colored::*;
use crossterm::execute;
use crossterm::terminal;
use std::io::stdout;
use crate::filter; 

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
/**
 * 0: Advanced Search
 * 1: Title
 * 2: Keywords
 * 3: Author
 * 4: Tag
 * 5: Rating
 * 6: Pages
 * 7: Status
 */

pub fn search_advanced(option: u64) -> u64 {
    let mut url_segment: String = String::new();
    let sorting = get_sorting();
    if sorting == ""{
        return 0;
    }
    let mut search_title: String = String::new();
    loop {
        match option {
            0 => loop {
                let mut attributes = vec![(String::new(), String::new()); 7];

                execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
                println!("Advanced Search:");
                println!(
                    "1: Title {}",
                    if attributes[0].1 != "" && attributes[0].1 != "0" {
                        format!(" - {}", attributes[0].0)
                    } else {
                        String::new()
                    }
                );
                println!(
                    "2: Keywords {}",
                    if attributes[1].1 != "" && attributes[1].1 != "0" {
                        format!(" - {}", attributes[1].0)
                    } else {
                        String::new()
                    }
                );
                println!(
                    "3: Author {}",
                    if attributes[2].1 != "" && attributes[2].1 != "0" {
                        format!(" - {}", attributes[2].0)
                    } else {
                        String::new()
                    }
                );
                println!(
                    "4: Tag {}",
                    if attributes[3].1 != "" && attributes[3].1 != "0" {
                        format!(" - {}", attributes[3].0)
                    } else {
                        String::new()
                    }
                );
                println!(
                    "5: Rating {}",
                    if attributes[4].1 != "" && attributes[4].1 != "0" {
                        format!(" - {}", attributes[4].0)
                    } else {
                        String::new()
                    }
                );
                println!(
                    "6: Pages {}",
                    if attributes[5].1 != "" && attributes[5].1 != "0" {
                        format!(" - {}", attributes[5].0)
                    } else {
                        String::new()
                    }
                );
                println!(
                    "7: Status {}",
                    if attributes[6].1 != "" && attributes[6].1 != "0" {
                        format!(" - {}", attributes[6].0)
                    } else {
                        String::new()
                    }
                );
                let option =
                    get_input("Enter the number of the option you want to use(exit to go back)");
                if option == "exit" {
                    return 0;
                }
                let option = option.parse::<usize>();
                if option.is_err() {
                    println!("Invalid input - press enter to continue");
                    std::io::stdin().read_line(&mut String::new()).unwrap();
                    continue;
                }
                let option = option.unwrap();
                match option {
                    1 => attributes[option - 1] = search_by_title(),
                    2 => attributes[option - 1] = search_by_keywords(),
                    3 => attributes[option - 1] = search_by_author(),
                    4 => attributes[option - 1] = search_by_tag(),
                    5 => attributes[option - 1] = search_by_rating(),
                    6 => attributes[option - 1] = search_by_pages_amount(),
                    7 => attributes[option - 1] = search_by_status(),
                    _ => {
                        println!("Invalid input - press enter to continue");
                        std::io::stdin().read_line(&mut String::new()).unwrap();
                        continue;
                    }
                };
                url_segment = String::new();
                attributes.clone().iter().for_each(|attribute| {
                    if attribute.1 != "0" && attribute.1 != "" {
                        url_segment.push_str("&");
                        url_segment.push_str(attribute.1.as_str());
                        search_title.push_str(format!(", {}", attribute.0).as_str());
                    }
                });

                let pages: u64 = get_num_of_pages(url_segment.clone());
                if pages == 0 {
                    println!(
                        "No results matching these criteria were found - press enter to continue"
                    );
                    std::io::stdin().read_line(&mut String::new()).unwrap();
                    continue;
                }
            },
            1 => (search_title, url_segment) = search_by_title(),
            2 => (search_title, url_segment) = search_by_keywords(),
            3 => (search_title, url_segment) = search_by_author(),
            4 => (search_title, url_segment) = search_by_tag(),
            5 => (search_title, url_segment) = search_by_rating(),
            6 => (search_title, url_segment) = search_by_pages_amount(),
            7 => (search_title, url_segment) = search_by_status(),
            _ => (),
        }

        if url_segment == "0" {
            return 0;
        }
        url_segment.push_str(format!("&{}", sorting).as_str());

        let pages: u64 = get_num_of_pages(url_segment.clone());
        if pages == 0 {
            println!("No results matching these criteria were found - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        return show_and_select_book(search_title, pages, url_segment);
    }
}

fn search_by_rating() -> (String, String) {
    let mut lower_bound: f32;
    let mut upper_bound: f32;

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        let input = get_input(
            "Enter the lower bound of the rating you want to search for(exit to go back)",
        );
        if input == "exit" {
            return ("0".to_string(), "0".to_string());
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
        break;
    }
    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        let input = get_input(
            "Enter the upper bound of the rating you want to search for(exit to go back)",
        );
        if input == "exit" {
            return ("0".to_string(), "0".to_string());
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
        break;
    }
    let url_segment = format!("minRating={}&maxRating={}", lower_bound, upper_bound);
    return (format!("{} - {}", lower_bound, upper_bound), url_segment);
}

fn search_by_tag() -> (String, String) {
    let mut tags: Vec<(&str, &str, i32)> = vec![
        ("Action", "action", 0),
        ("Adventure", "adventure", 0),
        ("Comedy", "comedy", 0),
        ("Contemporary", "contemporary", 0),
        ("Drama", "drama", 0),
        ("Fantasy", "fantasy", 0),
        ("Historical", "historical", 0),
        ("Horror", "horror", 0),
        ("Mystery", "mystery", 0),
        ("Psychological", "psychological", 0),
        ("Romance", "romance", 0),
        ("Satire", "satire", 0),
        ("Sci-Fi", "sci_fi", 0),
        ("Short Story", "one_shot", 0),
        ("Tragedy", "tragedy", 0),
        ("Anti-Hero Lead", "anti-hero_lead", 0),
        ("Artificial Intelligence", "artificial_intelligence", 0),
        ("Attractive Lead", "attractive_lead", 0),
        ("Cyberpunk", "cyberpunk", 0),
        ("Dungeon", "dungeon", 0),
        ("Dystopia", "dystopia", 0),
        ("Female Lead", "female_lead", 0),
        ("First Contact", "first_contact", 0),
        ("GameLit", "gamelit", 0),
        ("Gender Bender", "gender_bender", 0),
        ("Genetically Engineered", "genetically_engineered%20", 0),
        ("Grimdark", "grimdark", 0),
        ("Hard Sci-fi", "hard_sci", 0),
        ("Harem", "harem", 0),
        ("High Fantasy", "high_fantasy", 0),
        ("LitRPG", "litrpg", 0),
        ("Low Fantasy", "low_fantasy", 0),
        ("Magic", "magic", 0),
        ("Male Lead", "male_lead", 0),
        ("Martial Arts", "martial_arts", 0),
        ("Multiple Lead Characters", "multiple_lead", 0),
        ("Mythos", "mythos", 0),
        ("Non-Human Lead", "non-human_lead", 0),
        ("Portal Fantasy / Isekai", "summoned_hero", 0),
        ("Post Apocalyptic", "post_apocalyptic", 0),
        ("Progression", "progression", 0),
        ("Reader Interactive", "reader_interactive", 0),
        ("Reincarnation", "reincarnation", 0),
        ("Ruling Class", "ruling_class", 0),
        ("School Life", "school_life", 0),
        ("Secret Identity", "secret_identity", 0),
        ("Slice of Life", "slice_of_life", 0),
        ("Soft Sci-fi", "soft_sci-fi", 0),
        ("Space Opera", "space_opera", 0),
        ("Sports", "sports", 0),
        ("Steampunk", "steampunk", 0),
        ("Strategy", "strategy", 0),
        ("Strong Lead", "strong_lead", 0),
        ("Super Heroes", "super_heroes", 0),
        ("Supernatural", "supernatural", 0),
        (
            "Technologically Engineered",
            "technologically_engineered",
            0,
        ),
        ("Time Loop", "loop", 0),
        ("Time Travel", "time_travel", 0),
        ("Urban Fantasy", "urban_fantasy", 0),
        ("Villainous Lead", "villainous_lead", 0),
        ("Virtual Reality", "virtual_reality", 0),
        ("War and Military", "war_and_military", 0),
        ("Wuxia", "wuxia", 0),
        ("Xianxia", "xianxia", 0),
        ("Profanity", "profranity", 0),
        ("Sexual Content", "sexuality", 0),
        ("Graphic Violence", "graphic_violence", 0),
        ("Sensitive Content", "sensitive", 0),
        ("AI-Assisted Content", "ai_assisted", 0),
        ("AI-Generated Content", "ai_generated", 0),
    ];
    tags.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("Tags:");
        tags.iter().enumerate().for_each(|(i, tag)| {
            let mut msg = format!("{}: {}", i + 1, tag.0);
            if &tag.2 == &-1 {
                msg.push_str(" - excluded");
            } else if &tag.2 == &1 {
                msg.push_str(" - included");
            }
            println!("{}", msg);
        });

        let option = get_input("Enter the number of the tag you want to search for, use '-' exclude that tag e.g. -6 & use the id number again to remove it from the search - enter search to continue (exit to go back): ");
        if option == "exit" {
            return ("0".to_string(), "0".to_string());
        }
        if option == "search" {
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
                if current_tag_option != 0 {
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
    let mut tag_string = String::new();
    let mut search_msg: Vec<String> = vec![];
    for tag in tags.clone() {
        if tag.2 == 0 {
            continue;
        }
        if tag.2 > 0 {
            tag_string.push_str(format!("tagsAdd={}&", tag.1).as_str());
            search_msg.push(format!("{} - Included", tag.0));
        } else {
            tag_string.push_str(format!("tagsRemove={}&", tag.1).as_str());
            search_msg.push(format!("{} - Excluded", tag.0));
        }
    }
    return (search_msg.join(", "), tag_string);
}

fn search_by_keywords() -> (String, String) {
    let keywords: String;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    keywords =
        get_input("Enter the keywords of the fiction you want to search for(exit to go back)");
    if keywords == "exit" {
        return ("0".to_string(), "0".to_string());
    }
    let url_segment = format!("keywords={}", keywords);
    return (keywords, url_segment);
}

fn search_by_author() -> (String, String) {
    let author: String;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    author = get_input("Enter the author of the fiction you want to search for(exit to go back)");
    if author == "exit" {
        return ("0".to_string(), "0".to_string());
    }
    let url_segment = format!("author={}", author);
    return (author, url_segment);
}

fn search_by_title() -> (String, String) {
    let title: String;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    title = get_input("Enter the title of the fiction you want to search for(exit to go back)");
    if title == "exit" {
        return ("0".to_string(), "0".to_string());
    }
    let url_segment = format!("title={}", title);
    return (title, url_segment);
}

fn show_and_select_book(search_title: String, pages: u64, url_segment: String) -> u64 {
    let url_start = "https://www.royalroad.com/fictions/search?";
    let mut page: u32 = 1;
    let mut fiction_list: Vec<Fiction>;
    let mut response;
    let mut book_pick = String::new();
    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("Searching for {}", search_title.bold().blue());
        println!("Page {} of {}", page, pages);
        println!("Press enter to see the next one and on the page(type exit to quit) - Press enter to continue\n");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        response = reqwest::blocking::get(format!("{}{}&page={}", url_start, url_segment, page))
            .unwrap()
            .text()
            .unwrap();
        fiction_list = get_fiction_list(&response);
        for (i, fiction) in fiction_list.iter().enumerate() {
            println!(
                "{}: {}\n",
                (i + 1).to_string().bold().blue(),
                fiction.title.bold().blue()
            );
            println!("{} {}\n", "Tags:".red().bold(), fiction.tags.join(", "));
            println!("{} {}\n", "Description".red().bold(), fiction.description);
            println!("{} {}\n", "Pages:".red().bold(), fiction.pages);
            println!("{} {}\n", "Chapters:".red().bold(), fiction.chapters);
            println!("{} {}\n", "Rating:".red().bold(), fiction.rating);
            println!("{} {}\n", "Views:".red().bold(), fiction.views);
            println!("{} {}\n", "ID:".red().bold(), fiction.id);
            println!();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            match input.as_str() {
                "exit" => return 0,
                _ => {
                    let input_temp = input.clone();
                    let input_temp = input_temp.parse::<usize>();
                    if input_temp.is_err() {
                        continue;
                    }
                    let input_temp = input_temp.unwrap();
                    if input_temp > fiction_list.len() || input_temp == 0 {
                        continue;
                    }
                    book_pick = input;
                    break;
                }
            }
        }
        loop {
            if book_pick == "" {
                book_pick = get_input("Enter the number of the fiction you want to view(exit to go back, > to go to the next page, < to go to the previous page)");
            }
            match book_pick.as_str() {
                "exit" => return 0,
                ">" => {
                    if page == pages as u32 {
                        println!("Reached the last page - press enter to continue");
                        std::io::stdin().read_line(&mut String::new()).unwrap();
                        continue;
                    }
                    page += 1;
                    break;
                }
                "<" => {
                    if page == 1 {
                        println!("Reached the first page - press enter to continue");
                        std::io::stdin().read_line(&mut String::new()).unwrap();
                        continue;
                    }
                    break;
                }
                _ => {
                    let input = book_pick.parse::<usize>();
                    if input.is_err() {
                        println!("Invalid input - press enter to continue");
                        std::io::stdin().read_line(&mut String::new()).unwrap();
                        continue;
                    }
                    let input = input.unwrap();
                    if input > fiction_list.len() {
                        println!("Invalid input - press enter to continue");
                        std::io::stdin().read_line(&mut String::new()).unwrap();
                        continue;
                    }
                    let fiction = fiction_list.get(input - 1).unwrap();
                    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
                    println!("{} {}", "Title:".red().bold(), fiction.title.bold().blue());
                    println!("{} {}\n", "Tags:".red().bold(), fiction.tags.join(", "));
                    println!("{} {}\n", "Description:".red().bold(), fiction.description);
                    println!("{} {}\n", "Pages:".red().bold(), fiction.pages);
                    println!("{} {}\n", "Chapters:".red().bold(), fiction.chapters);
                    println!("{} {}\n", "Rating:".red().bold(), fiction.rating);
                    println!("{} {}\n", "Views:".red().bold(), fiction.views);
                    println!("{} {}\n", "ID:".red().bold(), fiction.id);
                    println!();
                    let config: SessionConfig =
                        confy::load("rrl_cli_reader", "SessionConfig").unwrap();
                    let new_chapter_id = get_chapters(fiction.id).first().unwrap().id;

                    let new_config = SessionConfig {
                        book_name: fiction.title.clone(),
                        book_id: fiction.id,
                        chapter_id: new_chapter_id,
                        color: config.color,
                    };
                    confy::store("rrl_cli_reader", "SessionConfig", new_config).unwrap();
                    println!("Press enter to continue");
                    std::io::stdin().read_line(&mut String::new()).unwrap();
                    return fiction.id;
                }
            }
        }
    }
}

pub fn search_by_status() -> (String, String) {
    let types = vec![
        ("Completed", "COMPLETED"),
        ("Ongoing", "ONGOING"),
        ("Hiatus", "HIATUS"),
        ("Dropped", "DROPPED"),
        ("Stub", "STUB"),
    ];
    let sorting = get_sorting();
    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("Status Types:");
        types.iter().enumerate().for_each(|(i, status_type)| {
            println!("{}: {}", i + 1, status_type.0);
        });
        let option =
            get_input("Enter the number of the status you want to search for(exit to go back)");
        if option == "exit" {
            return ("0".to_string(), "0".to_string());
        }
        let option = option.parse::<usize>();
        if option.is_err() {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        let option = option.unwrap();
        if option > types.len() {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        } else if option == 0 {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        let url_segment = format!("{}status={}", sorting, types.get(option - 1).unwrap().1);
        let search_msg = types.get(option - 1).unwrap().0.to_string();

        return (search_msg, url_segment);
    }
}

pub fn search_by_pages_amount() -> (String, String) {
    let lower_bound: u64;
    let mut upper_bound: u64;

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        let input =
            get_input("Enter the lower bound of the pages you want to search for(exit to go back)");
        if input == "exit" {
            return ("0".to_string(), "0".to_string());
        }
        let lower_bound_temp = input.parse::<u64>();
        if lower_bound_temp.is_err() {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        lower_bound = lower_bound_temp.unwrap();
        break;
    }
    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        let input =
            get_input("Enter the upper bound of the pages you want to search for(exit to go back)");
        if input == "exit" {
            return ("0".to_string(), "0".to_string());
        }
        let upper_bound_temp = input.parse::<u64>();
        if upper_bound_temp.is_err() {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        upper_bound = upper_bound_temp.unwrap();
        if upper_bound < lower_bound {
            println!("Upper bound must be greater than lower bound - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        break;
    }
    let url_segment = format!("minPages={}&maxPages={}", lower_bound, upper_bound);

    (format!("{} - {}", lower_bound, upper_bound), url_segment)
}

pub fn get_sorting() -> String {
    let order_types = vec![("Ascending", "asc"), ("Descending", "desc")];
    let sort_types = vec![
        ("Relevance", "relevance"),
        ("Popularity", "popularity"),
        ("Average Rating", "rating"),
        ("Last Update", "last_update"),
        ("Release Date", "release_date"),
        ("Followers", "followers"),
        ("Number of Pages", "length"),
        ("Views", "views"),
        ("Title", "title"),
        ("Author", "author"),
    ];
    let mut url_addition: String;
    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("Order Types:");
        order_types.iter().enumerate().for_each(|(i, order_type)| {
            println!("{}: {}", i + 1, order_type.0);
        });
        let option = get_input("Enter the number of the sorting you want to use(exit to go back, default is descending)");
        if option == "exit" {
            return "".to_string();
        }
        match option.as_str() {
            "1" => url_addition = format!("dir=asc"),
            "2" => url_addition = format!("dir=desc"),
            "" => url_addition = format!("dir=desc"),
            _ => {
                println!("Invalid input - press enter to continue");
                std::io::stdin().read_line(&mut String::new()).unwrap();
                continue;
            }
        }
        break;
    }
    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("Sort Types:");
        sort_types.iter().enumerate().for_each(|(i, sort_type)| {
            println!("{}: {}", i + 1, sort_type.0);
        });
        let mut option = get_input("Enter the number of the sorting you want to use(exit to go back)");
        if option == "exit" {
            return "".to_string();
        }else if option == "" {
            option = "1".to_string();
        }
        let option = option.parse::<usize>();
        if option.is_err() {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        let option = option.unwrap();
        if option > sort_types.len() || option == 0 {
            println!("Invalid input - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        url_addition
            .push_str(format!("&orderBy={}", sort_types.get(option - 1).unwrap().1).as_str());
        break;
    }
    url_addition
}

fn get_num_of_pages(url_segment: String) -> u64 {
    let url = format!("https://www.royalroad.com/fictions/search?{}", url_segment);
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    if response.contains("No results matching these criteria were found") {
        return 0;
    }

    let lines = response.lines().collect::<Vec<&str>>();
    let index = lines
        .iter()
        .position(|line| {
            line.contains("<div class=text-center><ul class='pagination justify-content-center'>")
        })
        .unwrap_or(0);
    if index != 0 {
        let line = lines[index];
        let line = line.split("data-page").collect::<Vec<&str>>();
        let line = line.last().unwrap();
        let line = line.split("'").collect::<Vec<&str>>();
        let line = line.get(1).unwrap();
        return line.parse::<u64>().unwrap();
    }
    return 1;
}

fn get_fiction_list(response: &str) -> Vec<Fiction> {
    let mut fiction_list: Vec<Fiction> = Vec::new();
    let lines: Vec<&str> = response.lines().collect();
    let index = lines
        .iter()
        .position(|line| line.contains("<div class=\"fiction-list\">"))
        .unwrap();
    let lines: Vec<&str> = lines.iter().skip(index + 1).copied().collect();
    let index = lines
        .iter()
        .position(|line| line.contains("<div class=\"col-md-4 hidden-xs hidden-sm\">"))
        .unwrap();
    let lines: Vec<&str> = lines.iter().take(index).copied().collect();
    let lines: Vec<&str> = lines.iter().map(|line| line.trim()).collect();
    // save the lines to a file for debugging
    let lines = lines.join("\n");
    let mut storys: Vec<&str> = lines
        .split("<div class=\"row fiction-list-item\">")
        .collect();

    storys.remove(0);
    for story in storys {
        let lines = story.lines().collect::<Vec<&str>>();
        let title_temp = lines
            .iter()
            .find(|line| line.contains("<img data-type"))
            .unwrap();
        let title_temp = title_temp.split("alt=").collect::<Vec<&str>>();
        let title_temp = title_temp.last().unwrap();
        let title_temp = title_temp.split("\"").collect::<Vec<&str>>();
        let title = title_temp.get(1).unwrap().to_string();

        let title = filter::add_ascii_symbols(title);
        let tags_temp_start = 2 + lines
            .iter()
            .position(|line| line.contains("<span class=\"tags\">"))
            .unwrap();
        let tags_temp_end = lines
            .iter()
            .position(|line| line.contains("<label for="))
            .unwrap();

        let tags_temp = lines.as_slice()[tags_temp_start..tags_temp_end].to_vec();
        let mut tags: Vec<String> = Vec::new();
        for tag_line in tags_temp {
            let tag = tag_line.split(">").collect::<Vec<&str>>();
            let tag = tag.get(1).unwrap();
            let tag = tag.split("<").collect::<Vec<&str>>();
            let tag = tag.first().unwrap();
            tags.push(tag.trim().to_string());
        }
        let description_temp_start = 1 + lines
            .iter()
            .position(|line| line.contains("<div id=\"description"))
            .unwrap();
        let description_temp_end = 1
            + description_temp_start
            + lines
                .iter()
                .skip(description_temp_start + 1)
                .position(|line| line.contains("</div>"))
                .unwrap();
        let description_temp =
            lines.as_slice()[description_temp_start..description_temp_end].to_vec();
        let mut description = String::new();
        for line in description_temp {
            let mut description_line = String::new();
            let mut skip = false;
            let line = line.replace("<br>", "\n");
            let line = line.replace("<hr>", "-------------------");
            let line = filter::add_ascii_symbols(line);
            if line == "<p></p>" {
                continue;
            }
            for c in line.chars() {
                if c == '<' {
                    skip = true;
                } else if c == '>' {
                    skip = false;
                } else if !skip {
                    description_line.push(c);
                }
            }
            description_line = description_line.replace("&lt;", "<");
            description_line = description_line.replace("&gt;", ">");
            description.push_str(format!("\n\n{}", &description_line).as_str());
        }
        description = description.trim().to_string().replace("&nbsp", "");
        let pages_temp = lines
            .iter()
            .find(|line| line.contains("Pages</span>"))
            .unwrap();
        let pages_temp = pages_temp
            .replace(" Pages</span>", "")
            .replace("<span>", "");
        let pages_temp = pages_temp.replace(",", "");
        let pages = pages_temp.parse::<u64>().unwrap();
        let chapters_temp = lines
            .iter()
            .find(|line| line.contains("Chapters</span>"))
            .unwrap();
        let chapters_temp = chapters_temp
            .replace(" Chapters</span>", "")
            .replace("<span>", "");
        let chapters = chapters_temp.parse::<u64>().unwrap();
        let rating_temp = lines
            .iter()
            .find(|line| line.contains("<span class=\"font-red-sunglo"))
            .unwrap();
        let rating_temp = rating_temp.split("title=\"").collect::<Vec<&str>>();
        let rating_temp = rating_temp.last().unwrap();
        let rating_temp = rating_temp.split("\"").collect::<Vec<&str>>();
        let rating_temp = rating_temp.first().unwrap();
        let rating = rating_temp.parse::<f32>().unwrap();
        let views_temp = lines
            .iter()
            .find(|line| line.contains(" Views</span>"))
            .unwrap();
        let views_temp = views_temp
            .replace(" Views</span>", "")
            .replace("<span>", "");
        let views_temp = views_temp.replace(",", "");
        let views = views_temp.parse::<u64>().unwrap();
        let id_temp = lines
            .iter()
            .find(|line| line.contains("<a href=\"/fiction/"))
            .unwrap();
        let id_temp = id_temp.split("href=\"/").collect::<Vec<&str>>();
        let id_temp = id_temp.last().unwrap();
        let id_temp = id_temp.split("/").collect::<Vec<&str>>();
        let id_temp = id_temp.get(1).unwrap();
        let id = id_temp.parse::<u64>().unwrap();
        fiction_list.push(Fiction {
            id,
            title,
            tags,
            description,
            pages,
            chapters,
            rating,
            views,
        });
    }

    fiction_list
}

fn get_input(msg: &str) -> String {
    let mut input = String::new();
    loop {
        println!("{}", msg);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        return input.trim().to_string();
    }
}
