run day:
    cargo run --bin "day$(printf "%02d" {{day}})"

run_fast day:
    cargo run --release --bin "day$(printf "%02d" {{day}})"

run_test day:
    cargo run --bin "day$(printf "%02d" {{day}})" -- test

create day: (_get_input day) (_create_test_input day) (_template day)

_get_input day:
    curl "https://adventofcode.com/2023/day/{{day}}/input" \
        --cookie "session=$(cat .session)" > \
        "inputs/day$(printf "%02d" {{day}}).txt"

_create_test_input day:
    mkdir -p test_inputs
    touch "test_inputs/day$(printf "%02d" {{day}}).txt"

_template day:
    #!/usr/bin/env bash
    set -euxo pipefail
    cat > "src/bin/day$(printf "%02d" {{day}}).rs" << EOF
    fn part_1(input: &[String]) -> u32 {
      0
    }

    fn part_2(input: &[String]) -> u32 {
      0
    }

    fn main() {
      aoc2023::run({{day}}, part_1, part_2);
    }
    EOF
