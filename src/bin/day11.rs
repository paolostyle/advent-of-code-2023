use ndarray::{Array2, Axis};

fn count_dimension_distance(a: usize, b: usize, empty_dims: &[usize], expansion: usize) -> usize {
  empty_dims
    .iter()
    .filter(|idx| (*idx > &a && *idx < &b) || (*idx < &a && *idx > &b))
    .count()
    * (expansion - 1)
    + a.abs_diff(b)
}

fn get_paths_length(input: &[String], expansion: usize) -> usize {
  let rows = input.len();
  let cols = input[0].len();

  let flat_input: Vec<char> = input.iter().flat_map(|str| str.chars()).collect();
  let space = Array2::from_shape_vec((rows, cols), flat_input).unwrap();
  let mut empty_rows = vec![];
  let mut empty_cols = vec![];

  for (i, row) in space.axis_iter(Axis(0)).enumerate() {
    if row.iter().all(|char| *char == '.') {
      empty_rows.push(i);
    }
  }

  for (i, col) in space.axis_iter(Axis(1)).enumerate() {
    if col.iter().all(|char| *char == '.') {
      empty_cols.push(i);
    }
  }

  let mut galaxies = vec![];
  for ((y, x), value) in space.indexed_iter() {
    if *value == '#' {
      galaxies.push((y, x));
    }
  }

  let mut total_paths_lenght = 0;

  for (i, galaxy) in galaxies.iter().enumerate() {
    for other_galaxy in galaxies.iter().skip(i + 1) {
      let row_diff = count_dimension_distance(galaxy.0, other_galaxy.0, &empty_rows, expansion);
      let col_diff = count_dimension_distance(galaxy.1, other_galaxy.1, &empty_cols, expansion);

      total_paths_lenght += row_diff + col_diff;
    }
  }

  total_paths_lenght
}

fn part_1(input: &[String]) -> usize {
  get_paths_length(input, 2)
}

fn part_2(input: &[String]) -> usize {
  get_paths_length(input, 1_000_000)
}

fn main() {
  aoc2023::run(11, part_1, part_2);
}
