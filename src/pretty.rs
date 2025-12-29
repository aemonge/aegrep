use owo_colors::OwoColorize; // Powers Up Strings with colors

pub fn print_found(line: &str, line_no: usize, char_no: usize, file: &str, query: &str) {
    let colored_line = line.replace(query, &query.bright_yellow().to_string());
    println!(
        "{}:{}:{} {}",
        file.bold(),
        (line_no + 1).to_string().bold(),
        (char_no).to_string().bold(),
        colored_line
    );
}
