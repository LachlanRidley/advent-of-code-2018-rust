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

      // for (key, value) in &map {
      //   println!("{}: {}", key, value);
      // }
    
    // scan through map values for any 2s
    //   increment twos
    // scan through map values for any 3s
    //   increment threes
  } 
  
  // let checksum = twos * threes
  // print checksum
}