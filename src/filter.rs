pub fn filter_content(url: String) -> String {
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let mut lines: Vec<&str> = response.lines().collect();
    let index = lines
        .iter()
        .position(|line| line.contains("chapter-inner chapter-content"))
        .unwrap();
    lines = lines.iter().skip(index + 1).copied().collect();
    let end_index = lines
        .iter()
        .position(|line| line.contains("</div>"))
        .unwrap();
    lines = lines.iter().take(end_index).copied().collect();
    lines = lines.iter().map(|line| line.trim()).collect();

    if lines.iter().any(|f| f.contains("<br />")) {
        lines = lines[0].split("<br />").collect::<Vec<&str>>();
        lines = lines
            .iter()
            .filter(|e| e.trim() != "")
            .map(|e| e.trim())
            .collect::<Vec<&str>>();
    }
    let lines: Vec<String> = lines
        .iter()
        .map(|line| {
            let mut new_line = String::new();
            let mut skip = false;
            let line = line.replace("<br>", "\n");
            for c in line.chars() {
                if c == '<' {
                    skip = true;
                } else if c == '>' {
                    skip = false;
                } else if !skip {
                    new_line.push(c);
                }
            }

            add_ascii_symbols(new_line)
        })
        .collect();
    lines.join("\n")
}

pub fn add_ascii_symbols(line: String) -> String {
    let line = line.replace("&nbsp;", " ");
    let line = line.replace("&amp;", "&");
    let line = line.replace("&quot;", "\"");
    let line = line.replace("&apos;", "'");
    let line = line.replace("&lt;", "<");
    let line = line.replace("&gt;", ">");
    let line = line.replace("&#34;", "\"");
    let line = line.replace("&#39;", "'");
    let line = line.replace("&#169;", "©");
    let line = line.replace("&#xA9;", "©");
    let line = line.replace("&#174;", "®");
    
    line
}
