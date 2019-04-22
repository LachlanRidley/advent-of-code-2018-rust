fn main() {
  let puzzle = "+1, +1, +1";

  let puzzle_items: Vec<&str> = puzzle.split(',').collect();

  let mut total = 0;

  for puzzle_item in puzzle_items {
    let trimmed_puzzle_item = puzzle_item.trim();
    let sign = &trimmed_puzzle_item[..1];
    let number = &trimmed_puzzle_item[1..].parse::<i32>().unwrap();

    if sign == "+" {
      total = total + number;
    } else if sign == "-" {
      total = total - number;
    };
  }

  println!("{}", total);
}