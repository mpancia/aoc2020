use parse::day02::Rule;
use parse::day02::path_to_str;
use parse::day02::parse_lines;
use parse::day02::validate;

fn main() {
    let password_input = path_to_str("data/day2.txt").unwrap();
    let parsed_passwords = parse_lines(&password_input);
    solve_problem_1(&parsed_passwords)
}

fn solve_problem_1(parsed_passwords: &Vec<(Rule, & str)>) {
    let mut valid_count = 0;
    for val in parsed_passwords.into_iter() {
       if validate(val) {
           valid_count += 1
       }
    }
    println!("{:?}", valid_count)
}