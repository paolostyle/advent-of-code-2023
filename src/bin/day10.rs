use std::collections::{BTreeSet, HashMap};

type Coord = (i32, i32);
type Moves = (i32, i32, i32, i32);
type Neighbours = (char, char, char, char);

fn find_starting_position_moves((top, right, bot, left): Neighbours) -> Moves {
  (
    get_allowed_moves(top).2,
    get_allowed_moves(right).3,
    get_allowed_moves(bot).0,
    get_allowed_moves(left).1,
  )
}

fn get_allowed_moves(kind: char) -> Moves {
  match kind {
    '|' => (1, 0, 1, 0),
    '-' => (0, 1, 0, 1),
    'L' => (1, 1, 0, 0),
    'J' => (1, 0, 0, 1),
    '7' => (0, 0, 1, 1),
    'F' => (0, 1, 1, 0),
    _ => (0, 0, 0, 0),
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Tile {
  coord: Coord,
  kind: char,
  next_moves: Vec<(Coord, [char; 3])>,
}

impl Tile {
  fn new(kind: char, coord: Coord, allowed_moves: Moves) -> Self {
    let next_moves = (0..4)
      .map(|idx| match idx {
        0 => ((coord.0 - allowed_moves.0, coord.1), ['|', '7', 'F']),
        1 => ((coord.0, coord.1 + allowed_moves.1), ['-', 'J', '7']),
        2 => ((coord.0 + allowed_moves.2, coord.1), ['|', 'L', 'J']),
        3 => ((coord.0, coord.1 - allowed_moves.3), ['-', 'L', 'F']),
        _ => panic!(),
      })
      .filter(|((y, x), _)| (y >= &0 || x >= &0) && (*y, *x) != coord)
      .collect();

    Tile {
      coord,
      kind,
      next_moves,
    }
  }
}

fn traverse<'a>(map: &'a HashMap<Coord, Tile>, start: &'a Tile) -> BTreeSet<&'a Tile> {
  let mut visited: BTreeSet<&Tile> = BTreeSet::new();
  let mut tiles_to_visit: Vec<&Tile> = vec![&start];

  while let Some(node) = tiles_to_visit.pop() {
    visited.insert(node);

    for (coords, allowed_fields) in &node.next_moves {
      let neighbour_tile = &map[coords];
      if allowed_fields.contains(&neighbour_tile.kind) && !visited.contains(neighbour_tile) {
        tiles_to_visit.push(neighbour_tile);
      }
    }
  }

  visited
}

fn part_1(input: &[String]) -> usize {
  let tiles: Vec<Vec<char>> = input.iter().map(|str| str.chars().collect()).collect();
  let mut tiles_map: HashMap<Coord, Tile> = HashMap::new();
  let mut start_position: Option<Coord> = None;

  for (y, row) in tiles.iter().enumerate() {
    for (x, kind) in row.iter().enumerate() {
      let coord = (y as i32, x as i32);
      let allowed_moves = if *kind == 'S' {
        start_position = Some(coord);
        find_starting_position_moves((
          tiles[y - 1][x],
          tiles[y][x + 1],
          tiles[y + 1][x],
          tiles[y][x - 1],
        ))
      } else {
        get_allowed_moves(*kind)
      };

      let tile = Tile::new(*kind, coord, allowed_moves);
      tiles_map.insert(coord, tile);
    }
  }

  let start = &tiles_map[&start_position.unwrap()];
  traverse(&tiles_map, start).len() / 2
}

fn part_2(_input: &[String]) -> i32 {
  0
}

fn main() {
  aoc2023::run(10, part_1, part_2);
}
