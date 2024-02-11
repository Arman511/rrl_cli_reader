use crate::{clear, getting::get_input};

pub fn get_tags() -> Vec<(&'static str, &'static str, i32)> {
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
    return tags;
}

pub fn get_sort_types() -> Vec<(&'static str, &'static str)> {
    vec![
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
    ]
}

pub fn get_sorting() -> String {
    let order_types = vec![("Ascending", "asc"), ("Descending", "desc")];
    let sort_types = get_sort_types();
    let mut url_addition: String;
    loop {
        clear();
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
        clear();
        println!("Sort Types:");
        sort_types.iter().enumerate().for_each(|(i, sort_type)| {
            println!("{}: {}", i + 1, sort_type.0);
        });
        let mut option =
            get_input("Enter the number of the sorting you want to use(exit to go back, default is relevance)");
        if option == "exit" {
            return "".to_string();
        } else if option == "" {
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
