use std::fs::read_to_string;

fn main() {
  let input: Vec<Vec<i32>> = read_input("input.txt");

  let result = calorie_counting(&input);
  println!("The elf holding the maximum calories has {} calories", result);

  let result = calorie_counting_top_3(&input);
  println!("The elves holding the maximum calories has {} calories", result);
}

fn calorie_counting(elf_inventories: &Vec<Vec<i32>>) -> i32 {
  let mut max_calories = 0;

  for inventory in elf_inventories {
    let mut sum: i32 = 0;

    for food in inventory {
      sum += food;
    }
    if sum > max_calories {
      max_calories = sum;
    }
  }

  max_calories
}


fn calorie_counting_top_3(elf_inventories: &Vec<Vec<i32>>) -> i32 {
  let mut calories: Vec<i32> = vec![0; elf_inventories.len()];

  for (i, elf_calories) in elf_inventories.iter().enumerate() {
    calories[i as usize] = elf_calories.iter().sum();
  }

  calories.sort_by(|a, b| b.cmp(a));
  calories[0..3].iter().sum()
}


fn read_input(filename: &str) -> Vec<Vec<i32>> {
  let mut result: Vec<Vec<i32>> = Vec::new();
  let mut current_vec: Vec<i32> = Vec::new();

  for line in read_to_string(filename).unwrap().lines() {
    match line {
      "" => {
        result.push(current_vec.clone());
        current_vec = Vec::new();
      },
      _ => {
        current_vec.push(line.to_string().parse().unwrap());
      }
    }
  }

  result
}
