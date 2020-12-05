use parse::day01;
use itertools::Itertools; // Has a combinations function which produces the combinations of inputs we need 

fn main() {
    let numbers = day01::parse_data_from_file("data/day1.txt");
    solve_problem_1(&numbers);
    solve_problem_2(&numbers);
}

fn solve_problem_1(numbers: &Vec<i32>) {
    let pairs = numbers.iter().combinations(2);
    let result = pairs.skip_while(|x| x[0] + x[1] != 2020).next().unwrap();
    println!("Problem 1 Solution: {:?}", result[0] * result[1]);
}

fn solve_problem_2(numbers: &Vec<i32>) {
    let pairs = numbers.iter().combinations(3);
    let result = pairs.skip_while(|x| x[0] + x[1] + x[2] != 2020).next().unwrap();
    println!("Problem 2 Solution: {:?}", result[0] * result[1] * result[2]);
}