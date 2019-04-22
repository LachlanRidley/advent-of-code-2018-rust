use std::collections::HashMap;

pub fn calculate_checksums(puzzle: &str) -> i32 {
  let box_ids: Vec<&str> = puzzle.split(',').collect();

  let mut twos = 0;
  let mut threes = 0;

  for box_id in box_ids {
    let mut map = HashMap::new();
    
    // create map of char -> counts
    for c in box_id.chars() {
      let foo = match map.get(&c) {
        Some(x) => x + 1,
        None => 1
      };
      
      map.insert(c, foo);
    }

    let contains_two = map.values().find(|&val| *val == 2).is_some();
    if contains_two {
      twos = twos + 1
    }

    let contains_three = map.values().find(|&val| *val == 3).is_some();
    if contains_three {
      threes = threes + 1
    }
  } 
  
  let checksum = twos * threes;
  
  return checksum;
}

#[cfg(test)]
mod tests {
  use super::calculate_checksums;

  #[test]
  fn it_can_do_provided_example() {
    let puzzle: &str = "abcdef,bababc,abbcde,abcccd,aabcdd,abcdee,ababab";

    assert_eq!(calculate_checksums(puzzle), 12)
  }
}