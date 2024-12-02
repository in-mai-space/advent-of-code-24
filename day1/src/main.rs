mod input;

fn main() {   
    let data = input::INPUT;

    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in data.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(&num1), Some(&num2)) = (parts.get(0), parts.get(1)) {
            column1.push(num1.parse::<i32>().unwrap());
            column2.push(num2.parse::<i32>().unwrap());
        }
    }

    let total_distance = total_distance(&column1, &column2);
    println!("Distance: {total_distance}");

    let similarity_score = similarity_score(&column1, &column2);
    println!("Similarity score: {similarity_score}");
}

fn total_distance(arr1: &[i32], arr2: &[i32]) -> i32 {
    let mut sorted_arr1 = arr1.to_vec();
    sorted_arr1.sort();
    let mut sorted_arr2 = arr2.to_vec();
    sorted_arr2.sort();

    let distance = sorted_arr1.iter()
        .zip(sorted_arr2.iter())
        .map(|(a, b)| (b-a).abs())
        .sum();
    distance
}

fn similarity_score(arr1: &[i32], arr2: &[i32]) -> i32 {
    let mut similarity_score = 0;

    for left_item in arr1.iter() {
        for right_item in arr2.iter() {
            if left_item == right_item {
                similarity_score += right_item;
            }
        }
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        let arr1 = [3, 4, 2, 1, 3, 3];
        let arr2 = [4, 3, 5, 3, 9, 3];
        assert_eq!(total_distance(&arr1, &arr2), 11);
    }

    #[test]
    fn test_similarity_score() {
        let arr1 = [3, 4, 2, 1, 3, 3];
        let arr2 = [4, 3, 5, 3, 9, 3];
        assert_eq!(similarity_score(&arr1, &arr2), 31);
    }
}