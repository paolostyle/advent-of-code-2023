use itertools::Itertools;
use std::string::ToString;

fn get_times_and_distances(input: &[String]) -> (Vec<u64>, Vec<u64>) {
  input
    .iter()
    .map(|line| {
      line
        .split_whitespace()
        .skip(1)
        .flat_map(str::parse)
        .collect()
    })
    .collect_tuple()
    .unwrap()
}

fn calculate_ways_to_win(time: u64, distance: u64) -> u64 {
  let mut limit = 0;

  for hold in 0..=time {
    if hold * (time - hold) > distance {
      limit = hold;
      break;
    }
  }

  time - limit * 2 + 1
}

fn concat_numbers(numbers: &[u64]) -> u64 {
  numbers
    .iter()
    .map(ToString::to_string)
    .join("")
    .parse()
    .unwrap()
}

fn part_1(input: &[String]) -> u64 {
  let (times, distances) = get_times_and_distances(input);

  times
    .into_iter()
    .zip(distances)
    .map(|(time, distance)| calculate_ways_to_win(time, distance))
    .product()
}

fn part_2(input: &[String]) -> u64 {
  let (times, distances) = get_times_and_distances(input);
  let time = concat_numbers(&times);
  let distance = concat_numbers(&distances);

  calculate_ways_to_win(time, distance)
}

fn main() {
  aoc2023::run(6, part_1, part_2);
}
