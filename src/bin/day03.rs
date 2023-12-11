use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coord(usize, usize);

#[derive(Debug)]
struct Number {
  value: u32,
  boundaries: HashSet<Coord>,
}

impl Number {
  fn new(slice: &str, coord: Coord, map_size: (usize, usize)) -> Self {
    let width = slice.len();
    let boundaries = Number::_generate_boundaries(&coord, width, map_size);

    Number {
      value: slice.parse::<u32>().unwrap(),
      boundaries,
    }
  }

  fn is_part_number(&self, symbols: &HashMap<Coord, char>) -> bool {
    self
      .boundaries
      .iter()
      .any(|coord| symbols.contains_key(coord))
  }

  fn get_gear_coord(&self, gears: &HashMap<Coord, char>) -> Option<Coord> {
    for boundary in &self.boundaries {
      if gears.contains_key(boundary) {
        return Some(*boundary);
      }
    }

    None
  }

  fn _generate_boundaries(coord: &Coord, width: usize, map_size: (usize, usize)) -> HashSet<Coord> {
    let has_top = usize::from(coord.0 > 0);
    let has_bot = usize::from(coord.0 < map_size.0 - 1);
    let has_left = usize::from(coord.1 > 0);
    let has_right = usize::from(coord.1 < map_size.1 - 1);

    let mut coords_to_check: HashSet<Coord> = ((coord.0 - has_top)..=(coord.0 + has_bot))
      .cartesian_product((coord.1 - has_left)..(coord.1 + width + has_right))
      .map(|(a, b)| Coord(a, b))
      .collect();

    [coord.0]
      .into_iter()
      .cartesian_product(coord.1..coord.1 + width)
      .for_each(|(y, x)| {
        coords_to_check.remove(&Coord(y, x));
      });

    coords_to_check
  }
}

fn parse(input: &[String]) -> (Vec<Number>, HashMap<Coord, char>) {
  let map_size = (input.len(), input[0].len());
  let mut numbers: Vec<Number> = vec![];
  let mut symbols: HashMap<Coord, char> = HashMap::new();

  for (line_idx, line) in input.iter().enumerate() {
    let mut num_start: Option<usize> = None;
    for (c_idx, c) in line.char_indices() {
      if c.is_ascii_digit() && num_start.is_none() {
        num_start = Some(c_idx);
      }
      if !c.is_ascii_digit() && c != '.' {
        symbols.insert(Coord(line_idx, c_idx), c);
      }
      if !c.is_ascii_digit() && num_start.is_some() {
        let start_idx = num_start.unwrap();
        let num = Number::new(
          &line[start_idx..c_idx],
          Coord(line_idx, start_idx),
          map_size,
        );
        numbers.push(num);
        num_start = None;
      }
    }
    if let Some(start_idx) = num_start {
      numbers.push(Number::new(
        &line[start_idx..],
        Coord(line_idx, start_idx),
        map_size,
      ));
    }
  }

  (numbers, symbols)
}

fn part_1(input: &[String]) -> u32 {
  let (numbers, symbols) = parse(input);

  numbers
    .iter()
    .filter(|num| num.is_part_number(&symbols))
    .map(|num| num.value)
    .sum()
}

fn part_2(input: &[String]) -> u32 {
  let (numbers, mut symbols) = parse(input);
  symbols.retain(|_, symbol| *symbol == '*');

  let mut gear_part_numbers: HashMap<Coord, Vec<u32>> = HashMap::new();

  for number in numbers {
    if let Some(gear_coord) = number.get_gear_coord(&symbols) {
      match gear_part_numbers.entry(gear_coord) {
        Vacant(entry) => {
          entry.insert(vec![number.value]);
        }
        Occupied(mut entry) => {
          entry.get_mut().push(number.value);
        }
      };
    }
  }

  gear_part_numbers
    .values()
    .filter(|part_numbers| part_numbers.len() == 2)
    .map(|numbers_pair| numbers_pair.iter().product::<u32>())
    .sum()
}

fn main() {
  aoc2023::run(3, part_1, part_2);
}
