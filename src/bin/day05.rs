use std::collections::HashMap;

use convert_case::Casing;
use itertools::Itertools;

type Mapper = Box<dyn Fn(u64) -> u64>;
type AlmanacMap = HashMap<String, Mapper>;

fn parse_input(input: &[String]) -> (Vec<u64>, AlmanacMap) {
  let mut iter = input.iter();
  let seeds: Vec<u64> = iter
    .next()
    .expect("seeds fail")
    .split(':')
    .nth(1)
    .expect("splitting failed")
    .split_whitespace()
    .flat_map(|num| num.parse::<u64>())
    .collect();

  iter.next();

  let mut mappers: AlmanacMap = HashMap::new();

  let create_mapper = |local_group: Vec<(u64, u64, u64)>| {
    let mapper = move |val: u64| -> u64 {
      let copy = local_group.clone();
      for (destination, source, range) in copy {
        if val >= source && val < source + range {
          return destination + val.abs_diff(source);
        }
      }
      val
    };

    Box::new(mapper)
  };

  let mut current_group: Option<&str> = None;
  let mut group: Vec<(u64, u64, u64)> = vec![];

  for line in iter {
    if line.is_empty() {
      let mapper = create_mapper(group.clone());
      if let Some(group_name) = current_group {
        mappers.insert(group_name.to_case(convert_case::Case::Snake), mapper);
      }
      group.clear();
    } else if line.contains("map") {
      current_group = line.split_whitespace().next();
    } else if let Some((destination, source, range)) = line
      .split_whitespace()
      .flat_map(|num| num.parse::<u64>())
      .collect_tuple()
    {
      group.push((destination, source, range));
    }
  }

  let mapper = create_mapper(group.clone());
  if let Some(group_name) = current_group {
    mappers.insert(group_name.to_case(convert_case::Case::Snake), mapper);
  }

  (seeds, mappers)
}

fn part_1(input: &[String]) -> u64 {
  let (seeds, mappers) = parse_input(input);

  seeds
    .into_iter()
    .map(|seed| {
      Some(seed)
        .map(&mappers["seed_to_soil"])
        .map(&mappers["soil_to_fertilizer"])
        .map(&mappers["fertilizer_to_water"])
        .map(&mappers["water_to_light"])
        .map(&mappers["light_to_temperature"])
        .map(&mappers["temperature_to_humidity"])
        .map(&mappers["humidity_to_location"])
        .unwrap()
    })
    .min()
    .unwrap()
}

fn part_2(input: &[String]) -> u32 {
  let (seeds, mappers) = parse_input(input);

  let (starts, lengths): (Vec<_>, Vec<_>) = seeds
    .into_iter()
    .enumerate()
    .partition(|(idx, _)| idx % 2 == 0);

  let pairs: Vec<(u64, u64)> = starts
    .into_iter()
    .map(|(_, seed_start)| seed_start)
    .zip(lengths.into_iter().map(|(_, length)| length))
    .collect();

  dbg!(pairs);

  0
}

fn main() {
  aoc2023::run(5, part_1, part_2)
}
