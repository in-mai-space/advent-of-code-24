fn main() {    
    // both approaches is O(nlogn): sorting is O(nlogn), while map and reduce is O(n)
    let for_loop = for_loop_approach(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]);
    let zip = zipping_approach(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]);

    println!("{for_loop}");
    println!("{zip}");
}

fn for_loop_approach(arr1: &[i32], arr2: &[i32]) -> i32 {
    let mut sorted_arr1 = arr1.to_vec();
    sorted_arr1.sort();
    let mut sorted_arr2 = arr2.to_vec();
    sorted_arr2.sort();

    let mut distance: i32 = 0;

    for (index, _) in sorted_arr1.iter().enumerate() {
        if let (Some(&value1), Some(&value2)) = (sorted_arr1.get(index), sorted_arr2.get(index)) {
            distance += (value2 - value1).abs()
        }
    }
    distance
}

fn zipping_approach(arr1: &[i32], arr2: &[i32]) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        let arr1 = [3, 4, 2, 1, 3, 3];
        let arr2 = [4, 3, 5, 3, 9, 3];
        assert_eq!(for_loop_approach(&arr1, &arr2), 11);
        assert_eq!(zipping_approach(&arr1, &arr2), 11);
    }
}