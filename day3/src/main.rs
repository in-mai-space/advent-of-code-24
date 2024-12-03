mod input;

fn main() {
    let data = input::INPUT;

    let part1_count = part_1(data);
    println!("{part1_count}");
    let part2_count = part_2(data);
    println!("{part2_count}");
}

fn part_1(input: &str) -> i32 {
    let mul_regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut count = 0;
    for mul in mul_regex.captures_iter(input) {
        let num1 = mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
        count += num1 * num2;
    }
    count
}

fn part_2(input: &str) -> i32 {
    let combined_regex = regex::Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();

    let mut count = 0;
    let mut skip: bool = false;
    for mul in combined_regex.captures_iter(input) {
        if let Some(_) = mul.get(1) {            
            let num1 = mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let num2 = mul.get(3).unwrap().as_str().parse::<i32>().unwrap();
            if !skip {
                count += num1 * num2;
            }
        }
        else if let Some(_) = mul.get(4) {
            skip = false;
        }
        else if let Some(_) = mul.get(5) {
            skip = true;
        }
    }
    count
}