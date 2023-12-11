fn part_1(input: &[String]) -> u32 {
  let mut sum = 0;

  for line in input {
    let mut new_number = String::with_capacity(2);
    let chars = line.chars().collect::<Vec<char>>();

    for char in &chars {
      if char.is_ascii_digit() {
        new_number.push(*char);
        break;
      }
    }

    for char in chars.iter().rev() {
      if char.is_ascii_digit() {
        new_number.push(*char);
        break;
      }
    }

    sum += new_number.parse::<u32>().unwrap();
  }

  sum
}

const DIGITS: [&str; 18] = [
  "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
  "seven", "eight", "nine",
];

fn spelled_to_digit(spelled_digit: &str) -> &str {
  match spelled_digit {
    "one" => "1",
    "two" => "2",
    "three" => "3",
    "four" => "4",
    "five" => "5",
    "six" => "6",
    "seven" => "7",
    "eight" => "8",
    "nine" => "9",
    digit => digit,
  }
}

fn find_number(str: &str) -> u32 {
  let mut value = String::new();
  let mut min = (usize::MAX, "");
  let mut max = (usize::MIN, "");
  for digit in DIGITS {
    for occurence in str.match_indices(digit) {
      if occurence.0 < min.0 {
        min = occurence;
      }
      if occurence.0 >= max.0 {
        max = occurence;
      }
    }
  }

  value += spelled_to_digit(min.1);
  value += spelled_to_digit(max.1);
  value.parse::<u32>().unwrap()
}

fn part_2(input: &[String]) -> u32 {
  input.iter().map(|line| find_number(line)).sum()
}

fn main() {
  aoc2023::run(1, part_1, part_2);
}
