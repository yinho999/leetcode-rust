use std::collections::{HashMap, HashSet};

fn contains_duplicate(nums: Vec<i32>) -> bool { try_case_2(nums) }


fn try_case_1(nums: Vec<i32>) -> bool {
    let mut mapping = HashMap::new();
    for i in nums {
        if mapping.contains_key(&i) {
            return true;
        } else {
            mapping.insert(i, true);
        }
    }
    false
}

fn try_case_2(nums: Vec<i32>) -> bool {
    let hashset: HashSet<&i32> = nums.iter().collect();
    hashset.len() != nums.len()
}

fn try_case_3(nums: Vec<i32>) -> bool {
    let mut hashset = HashSet::new();
    for i in nums {
        match hashset.get(&i) {
            Some(_) => return true,
            None => {
                hashset.insert(i);
            }
        }
    }
    false
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums.to_vec()), true);
    }

    #[test]
    fn example_2() {
        let nums = [1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums.to_vec()), false);
    }

    #[test]
    fn example_3() {
        let nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums.to_vec()), true);
    }
}
