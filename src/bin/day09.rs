use itertools::Itertools;

fn get_sequences(report: Vec<i64>) -> Vec<Vec<i64>> {
  let mut seqs = vec![report];
  let mut seq = seqs.last().unwrap();

  loop {
    let diffs: Vec<i64> = seq.iter().tuple_windows().map(|(a, b)| b - a).collect();
    let should_end = diffs.iter().all(|val| *val == 0);
    seqs.push(diffs);

    if should_end {
      break;
    }
    seq = seqs.last().unwrap();
  }

  seqs
}

fn extrapolate_forwards(report: Vec<i64>) -> i64 {
  get_sequences(report)
    .iter()
    .rev()
    .fold(0, |acc, seq| acc + seq.last().unwrap())
}

fn extrapolate_backwards(report: Vec<i64>) -> i64 {
  get_sequences(report)
    .iter()
    .rev()
    .fold(0, |acc, seq| seq.first().unwrap() - acc)
}

fn get_reports(input: &[String]) -> Vec<Vec<i64>> {
  input
    .iter()
    .map(|str| str.split_whitespace().flat_map(str::parse).collect())
    .collect()
}

fn part_1(input: &[String]) -> i64 {
  get_reports(input)
    .into_iter()
    .map(extrapolate_forwards)
    .sum()
}

fn part_2(input: &[String]) -> i64 {
  get_reports(input)
    .into_iter()
    .map(extrapolate_backwards)
    .sum()
}

fn main() {
  aoc2023::run(9, part_1, part_2);
}
