mod day2;

pub fn main() {
  let puzzle: &str = "abcdef,bababc,abbcde,abcccd,aabcdd,abcdee,ababab";

  let answer = day2::calculate_checksums(puzzle);

  println!("checksum: {}", answer);
}