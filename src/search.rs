use colored::*;
use crossterm::execute;
use crossterm::terminal;
use std::io::stdout;

use crate::get_chapters;
use crate::SessionConfig;

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

pub fn search_by_title() -> u64 {
    let mut title: String;
    let mut response: String;
    let mut pages: u64 = 1;

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        title = get_input("Enter the title of the fiction you want to search for(exit to go back)");
        if title == "exit" {
            return 0;
        }

        let url = format!("https://www.royalroad.com/fictions/search?title={}", title);
        response = reqwest::blocking::get(url).unwrap().text().unwrap();

        if response.contains("No results matching these criteria were found") {
            println!("No results found - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        break;
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
        pages = line.parse::<u64>().unwrap();
    }

    return show_books(title, pages, response);
}

pub fn search_by_author() -> u64 {
    let mut author: String;
    let mut response: String;
    let mut pages: u64 = 1;

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        author =
            get_input("Enter the author of the fiction you want to search for(exit to go back)");
        if author == "exit" {
            return 0;
        }

        let url = format!(
            "https://www.royalroad.com/fictions/search?&author={}",
            author
        );
        response = reqwest::blocking::get(url).unwrap().text().unwrap();

        if response.contains("No results matching these criteria were found") {
            println!("No results found - press enter to continue");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            continue;
        }
        break;
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
        pages = line.parse::<u64>().unwrap();
    }

    return show_books(author, pages, response);
}

pub fn search_by_tag() -> u64{
    0
}

fn show_books(title: String, pages: u64, mut response: String) -> u64 {
    let mut fiction_list: Vec<Fiction>;
    let mut page: u32 = 1;

    loop {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("Searching for {}", title.bold().blue());
        println!("Page {} of \"{}\"", page, pages);
        println!("Press enter to see the next one and on the page(type exit to quit) - Press enter to continue\n");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        fiction_list = get_fiction_list(&response);
        for (i, fiction) in fiction_list.iter().enumerate() {
            println!(
                "{}: {}\n",
                (i + 1).to_string().bold().blue(),
                fiction.title.bold().blue()
            );
            println!("{} {}\n", "Tags:".red().bold(), fiction.tags.join(", "));
            println!("{}\n{}\n", "Description".red().bold(), fiction.description);
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
                _ => continue,
            }
        }
        loop {
            let input = get_input("Enter the number of the fiction you want to view(exit to go back, > to go to the next page, < to go to the previous page)");
            match input.as_str() {
                "exit" => return 0,
                ">" => {
                    if page == pages as u32 {
                        continue;
                    }
                    page += 1;
                    let url = format!(
                        "https://www.royalroad.com/fictions/search?page={}&title={}",
                        page, title
                    );
                    response = reqwest::blocking::get(url).unwrap().text().unwrap();
                    break;
                }
                "<" => {
                    if page == 1 {
                        continue;
                    }
                    page -= 1;
                    let url = format!(
                        "https://www.royalroad.com/fictions/search?page={}&title={}",
                        page, title
                    );
                    response = reqwest::blocking::get(url).unwrap().text().unwrap();
                    break;
                }
                _ => {
                    let input = input.parse::<usize>();
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

        let title = title.replace("&nbsp;", "");
        let title = title.replace("&amp;", "&");
        let title = title.replace("&#34;", "\"");
        let title = title.replace("&#39;", "'");
        let title = title.replace("&#169;", "©");
        let title = title.replace("&#xA9;", "©");
        let title = title.replace("&#174;", "®");
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
            tags.push(tag.to_string());
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
            let line = line.replace("&nbsp;", "");
            let line = line.replace("&amp;", "&");
            let line = line.replace("&#34;", "\"");
            let line = line.replace("&#39;", "'");
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
        input = input.trim().to_string();
        if input == "" {
            println!("Input must be at least 1 characters long");
            continue;
        }
        break;
    }
    input
}
