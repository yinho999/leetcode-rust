// Given two strings s and t, return true if t is an anagram of s, and false otherwise.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
// typically using all the original letters exactly once.

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    try_case_3(s, t)
}

fn try_case_1(s: String, t: String) -> bool {
    let mut character_counts: HashMap<char, i32> = HashMap::new();
    for i in s.chars() {
        let count = character_counts.get(&i).unwrap_or(&0);
        let count = count + 1;
        character_counts.insert(i, count);
    }
    for i in t.chars() {
        let count = character_counts.get(&i).unwrap_or(&0);
        let count = count - 1;
        character_counts.insert(i, count);
    }
    for (_, v) in character_counts.iter() {
        if *v != 0 { return false; };
    }
    return true;
}

fn try_case_2(s: String, t: String) -> bool {
    let mut word_list = [0; 26];
    for i in s.to_ascii_lowercase().chars() {
        let index = i as usize - 97;
        word_list[index] = word_list[index] + 1;
    }

    for i in t.to_ascii_lowercase().chars() {
        let index = i as usize - 97;
        word_list[index] = word_list[index] - 1;
    }
    for word in word_list {
        if word != 0 {
            return false;
        }
    }
    true
}

fn try_case_3(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; };
    let mut word_list = [0; 127];
    for i in s.to_ascii_lowercase().chars() {
        let index = i as usize;
        word_list[index] = word_list[index] + 1;
    }

    for i in t.to_ascii_lowercase().chars() {
        let index = i as usize;
        word_list[index] = word_list[index] - 1;
    }
    for word in word_list {
        if word != 0 {
            return false;
        }
    }
    true
}
fn try_case_4(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; };
    let mut s_vec = s.chars().collect::<Vec<char>>();
    let mut t_vec = t.chars().collect::<Vec<char>>();
    s_vec.sort();
    t_vec.sort();
    s_vec == t_vec
}
fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn test_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        assert_eq!(is_anagram(s, t), false);
    }
}