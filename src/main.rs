mod day1;
mod day2;

struct Problem<'a> {
  solution: fn(&str) -> (),
  input_file_name: &'a str,
}

static PROBLEMS: [Problem; 2] = [
  Problem{ solution: day1::solve, input_file_name: "inputs/day1.txt" },
  Problem{ solution: day2::solve, input_file_name: "inputs/day2.txt" },
];

fn main() {
  for (i, problem) in PROBLEMS.iter().enumerate() {
    println!("======================================");
    println!("Solving problem for day {}!", i + 1);
    println!("======================================");
    (problem.solution)(problem.input_file_name);
    println!();
  }
}