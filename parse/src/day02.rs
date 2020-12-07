use std::io::{self, Read};
use std::fs::File;
use std::ops::Range;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    count_range: Range<i32>,
    character: char
}

pub fn validate(password_rule: &Rule, password: &str) -> bool {
    let mut count_dict: HashMap<char, i32> = HashMap::new();
    for pass_char in password.chars() {
        let curr_count = count_dict.get(&pass_char);
        match curr_count.is_none() {
            false => count_dict.insert(pass_char, curr_count.unwrap() + 1),
            true => count_dict.insert(pass_char, 1)
        };
    }
    let char_count = count_dict.get(&password_rule.character).unwrap_or(&0);
    if password_rule.count_range.contains(char_count) {
        true
    } else {
        false
    }
}

pub fn path_to_str(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn parse_lines<'a>(s: &'a str) -> Vec<(Rule, &'a str)> {
    s.lines().map(|line| {
        let split_line : Vec<&str> = line.split(": ").collect();
        let rule = { 
            let rule_text : Vec<&str> = split_line[0].split(" ").collect();
            let range_text : Vec<&str> = rule_text[0].split("-").collect();
            Rule {
                count_range : std::ops::Range{ 
                    start : range_text[0].parse::<i32>().expect("Lower range error"), 
                    end: range_text[1].parse::<i32>().expect("Upper range error") + 1
            }, 
                character: rule_text[1].chars().next().expect("Empty character")
            }
        };
        let password = split_line[1];
        (rule, password)
    }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
       let all_lines = super::path_to_str("../data/day2.txt").unwrap();
       let all_parsed_lines = super::parse_lines(&all_lines);
       let first_parsed = super::validate(&all_parsed_lines[0].0, &all_parsed_lines[0].1);
       assert_eq!(false, first_parsed)
    }

    fn test_validate() {
       let pr = super::Rule{ count_range: std::ops::Range{ start: 1, end: 5}, character: 'a'} ;
       assert_eq!(super::validate(&pr, "caaaat"), true);
       assert_eq!(super::validate(&pr, "ct"), false);
       assert_eq!(super::validate(&pr, "caaaaat"), false);
    }
}