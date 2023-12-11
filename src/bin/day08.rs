use std::{
  collections::{HashMap, HashSet},
  iter::Cycle,
  str::Chars,
};

use regex::Regex;

#[derive(Debug)]
struct Node<'a> {
  value: &'a str,
  left: &'a str,
  right: &'a str,
}

fn get_directions_and_nodes(input: &[String]) -> (Cycle<Chars<'_>>, HashMap<&str, Node>) {
  let mut it = input.iter();
  let directions = it.next().unwrap().chars().cycle();
  let pattern =
    Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").expect("invalid regex");

  let nodes: HashMap<&str, Node> = it
    .skip(1)
    .map(|str| {
      let (_, [value, left, right]) = pattern.captures(str).unwrap().extract();
      (value, Node { value, left, right })
    })
    .collect();

  (directions, nodes)
}

fn part_1(input: &[String]) -> u32 {
  let (mut directions, nodes) = get_directions_and_nodes(input);

  let mut count = 0;
  let mut node: &Node = &nodes["AAA"];

  loop {
    let direction = directions.next().unwrap();
    if direction == 'L' {
      node = &nodes[node.left];
    } else {
      node = &nodes[node.right];
    }

    count += 1;

    if node.value == "ZZZ" {
      break;
    }
  }

  count
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
  while b != 0 {
    let t = b;
    b = a % b;
    a = t;
  }

  a
}

fn lcm(a: u64, b: u64) -> u64 {
  a * b / gcd(a, b)
}

fn part_2(input: &[String]) -> u64 {
  let (mut directions, nodes) = get_directions_and_nodes(input);

  let active_nodes: Vec<&Node> = nodes
    .iter()
    .filter(|(key, _)| key.ends_with('A'))
    .map(|(_, node)| node)
    .collect();

  let mut cycle_lengths = vec![];

  for active_node in active_nodes {
    let mut visited_nodes: HashSet<&str> = HashSet::new();
    let mut current_node: &Node = active_node;
    let mut round: u64 = 0;

    loop {
      visited_nodes.insert(current_node.value);
      if current_node.value.ends_with('Z') {
        cycle_lengths.push(round);
        break;
      }

      let direction = directions.next().unwrap();
      if direction == 'L' {
        current_node = &nodes[current_node.left];
      } else {
        current_node = &nodes[current_node.right];
      }

      round += 1;
    }
  }

  cycle_lengths.into_iter().reduce(lcm).unwrap()
}

fn main() {
  aoc2023::run(8, part_1, part_2);
}
