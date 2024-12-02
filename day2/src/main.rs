mod input;

fn main() {
    let data = input::INPUT;

    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in data.lines() {
        let report: Vec<i32> = line.split_whitespace().map(|a| a.parse::<i32>().unwrap()).collect();
        reports.push(report);
    }

    let safe_count_1 = count_safe_reports_with(&reports, |report| is_safe_report(report));
    println!("{safe_count_1}");

    let safe_count_dampener = count_safe_reports_with(&reports, |report| problem_dampener(report));
    println!("{safe_count_dampener}")
}

// count safe reports given a mapping function
fn count_safe_reports_with<F>(reports: &Vec<Vec<i32>>, check_fn: F) -> i32
where
    F: Fn(&[i32]) -> i32,
{
    reports.iter()
           .map(|report| check_fn(report))
           .sum()
}

// return 1 if a report is safe
fn is_safe_report(report: &[i32]) -> i32 {
    if report.len() == 0 {
        return 0;
    }

    for index in 1..report.len() - 1 {
        let prev = report[index - 1];
        let current = report[index];
        let next = report[index + 1];

        let pattern_change: bool = pattern_change(prev - current, current - next);
        let difference1: bool = safe_distance(prev, current);
        let difference2: bool = safe_distance(current, next);

        if pattern_change || !difference1 || !difference2 {
            return 0;
        }
    }

    1
}

// return 1 if report can tolerate a bad single level
fn problem_dampener(report: &[i32]) -> i32 {
    if is_safe_report(report) == 1 {
        return 1;
    }

    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe_report(&modified_report) == 1 {
            return 1;
        }
    }

    0
}

// return true if there is a pattern change
fn pattern_change(num1: i32, num2: i32) -> bool {
    (num1 < 0 && num2 > 0) || (num1 > 0 && num2 < 0)
}

// check if distance between two numbers is safe
fn safe_distance(num1: i32, num2: i32) -> bool {
    let difference: i32 = (num1 - num2).abs();
    difference >= 1 && difference <= 3
}

