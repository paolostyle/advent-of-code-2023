use std::{env, fmt::Display, fs::File, io::BufRead, io::BufReader, time::Instant};

fn load_file(path: String) -> Vec<String> {
  let input_file = File::open(path).expect("Input file doesn't exist");
  BufReader::new(input_file).lines().flatten().collect()
}

pub fn run<R1: Display, R2: Display>(
  day: u8,
  part_1: fn(&[String]) -> R1,
  part_2: fn(&[String]) -> R2,
) {
  let test = matches!(env::args().nth(1).as_deref(), Some("test"));

  println!(
    "DAY {day}{}ANSWERS:",
    if test { " TEST DATA " } else { " " }
  );

  let path = format!(
    "{}inputs/day{:0>2}.txt",
    if test { "test_" } else { "" },
    day
  );

  let now = Instant::now();

  let input = load_file(path);
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let time = now.elapsed();
  println!("Execution time: {:.2?}", time);
}
