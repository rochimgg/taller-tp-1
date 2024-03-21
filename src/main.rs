mod tests;
mod reader;
mod alnum_checker;

fn main() {
    let reader = reader::Reader::new(String::from("/Users/mac/repos/taller-tp-1/src/text.txt"));
    let result = reader.read_file();
    match result {
        Ok(content) => {
            let checker = alnum_checker::AlnumChecker::new();
            let lines: Vec<&str> = content.split("\n").collect();
            for line in lines {
                if checker.is_match(line) {
                    println!("{}", line);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}