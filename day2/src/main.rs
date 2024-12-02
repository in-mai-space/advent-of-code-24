fn main() {
    let count = count_safe_reports(&[&[7, 6, 4, 2, 1], &[1, 2, 7, 8, 9], &[9, 7, 6, 2, 1], &[1, 3, 2, 4, 5], &[8, 6, 4, 4, 1], &[1, 3, 6, 7, 9]]);
    println!("{count}");
}

// count how many safe reports there are
fn count_safe_reports(reports: &[&[i32]]) -> i32 {
    reports.iter().map(|report| check_safe_reports(report)).sum()
}

// check if a report is safe or not
fn check_safe_reports(report: &[i32]) -> i32 {
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

// return true if there is a pattern change
fn pattern_change(num1: i32, num2: i32) -> bool {
    (num1 < 0 && num2 > 0) || (num1 > 0 && num2 < 0)
}

// check if distance between two numbers is safe
fn safe_distance(num1: i32, num2: i32) -> bool {
    let difference: i32 = (num1 - num2).abs();
    difference >= 1 && difference <= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_reports_example() {
        let reports: &[&[i32]] = &[&[7, 6, 4, 2, 1], &[1, 2, 7, 8, 9], &[9, 7, 6, 2, 1], &[1, 3, 2, 4, 5], &[8, 6, 4, 4, 1], &[1, 3, 6, 7, 9]];
        assert_eq!(count_safe_reports(reports), 2);
    }

    #[test]
    fn test_empty_report() {
        let reports: &[&[i32]] = &[&[]];
        assert_eq!(count_safe_reports(reports), 0);
    }

    #[test]
    fn test_empty() {
        let reports: &[&[i32]] = &[];
        assert_eq!(count_safe_reports(reports), 0);
    }
}