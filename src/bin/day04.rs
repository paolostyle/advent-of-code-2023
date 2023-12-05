use std::collections::{HashMap, HashSet};

fn parse_scratchcard(scratchcard: &str) -> usize {
  let mut parts = scratchcard.split([':', '|']);

  let owned_numbers: HashSet<u32> = parts
    .nth(1)
    .expect("splitting error")
    .split_whitespace()
    .flat_map(|num| num.parse::<u32>())
    .collect();

  let winning_numbers: HashSet<u32> = parts
    .next()
    .expect("splitting error")
    .split_whitespace()
    .flat_map(|num| num.parse::<u32>())
    .collect();

  winning_numbers.intersection(&owned_numbers).count()
}

fn part_1(input: &[String]) -> usize {
  let mut sum = 0;
  for scratchcard in input {
    let matches = parse_scratchcard(scratchcard);
    let points = match matches {
      0 => 0,
      val => 2_usize.pow((val - 1) as u32),
    };
    sum += points;
  }

  sum
}

fn part_2(input: &[String]) -> u32 {
  let card_nos_iter = 1..=input.len();
  let mut copies: HashMap<usize, u32> = card_nos_iter.clone().map(|card_no| (card_no, 1)).collect();

  for (scratchcard, card_no) in input.iter().zip(card_nos_iter) {
    let matches = parse_scratchcard(scratchcard);
    let card_copies = copies[&card_no];

    for next_card_no in (card_no + 1)..=(card_no + matches) {
      if let Some(entry) = copies.get_mut(&next_card_no) {
        *entry += card_copies;
      }
    }
  }

  copies.values().sum()
}

fn main() {
  aoc2023::run(4, part_1, part_2)
}
