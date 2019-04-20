use std::collections::HashMap;

fn main() {
  let puzzle = "abcdef,bababc,abbcde,abcccd,aabcdd,abcdee,ababab";

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
  println!("checksum: {}", checksum);
}