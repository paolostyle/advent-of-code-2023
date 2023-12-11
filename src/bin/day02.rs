use std::cmp::max;

#[derive(Debug, Copy, Clone)]
struct Set(u32, u32, u32);

#[derive(Debug)]
struct Game {
  id: u32,
  sets: Vec<Set>,
}

impl Game {
  pub fn new(game_record: &str) -> Self {
    let mut parts: Vec<_> = game_record.split(&[':', ';'][..]).collect();
    let mut sets: Vec<Set> = Vec::new();

    while parts.len() > 1 {
      let mut set = Set(0, 0, 0);
      if let Some(cubes) = parts.pop() {
        cubes.trim().split(", ").for_each(|cube| {
          let cube_info: Vec<_> = cube.split_whitespace().collect();
          match (cube_info[0].parse::<u32>(), cube_info[1]) {
            (Ok(count), "red") => set.0 = count,
            (Ok(count), "green") => set.1 = count,
            (Ok(count), "blue") => set.2 = count,
            _ => panic!("parsing went wrong, {cube_info:?}"),
          }
        });
        sets.push(set);
      }
    }

    let id = parts[0]
      .split_whitespace()
      .nth(1)
      .unwrap()
      .parse::<u32>()
      .unwrap();

    Self { id, sets }
  }

  fn analyze(&self) -> Set {
    let mut max_set = Set(0, 0, 0);
    self.sets.iter().for_each(|set| {
      max_set.0 = max(set.0, max_set.0);
      max_set.1 = max(set.1, max_set.1);
      max_set.2 = max(set.2, max_set.2);
    });

    max_set
  }

  pub fn is_possible(&self, set_to_compare: Set) -> bool {
    let max_set = self.analyze();

    set_to_compare.0 >= max_set.0 && set_to_compare.1 >= max_set.1 && set_to_compare.2 >= max_set.2
  }

  pub fn power(&self) -> u32 {
    let max_set = self.analyze();

    max_set.0 * max_set.1 * max_set.2
  }
}

fn part_1(input: &[String]) -> u32 {
  input
    .iter()
    .map(|game_record| {
      let game = Game::new(game_record);
      if game.is_possible(Set(12, 13, 14)) {
        game.id
      } else {
        0
      }
    })
    .sum()
}

fn part_2(input: &[String]) -> u32 {
  input
    .iter()
    .map(|game_record| Game::new(game_record).power())
    .sum()
}

fn main() {
  aoc2023::run(2, part_1, part_2);
}
