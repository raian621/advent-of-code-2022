use std::fs::File;

#[path = "input.rs"] mod input;

pub fn solve(input_file_path: &str) {
  let input_file = match File::open(input_file_path) {
    Err(why) => { 
      println!("{}", why);
      return;
    },
    Ok(file_) => file_
  };
  
  let raw_input = input::read(input_file);
  let input = convert_input(raw_input);

  let result = calorie_counting(&input);
  println!("The elf holding the maximum calories has {} calories", result);

  let result = calorie_counting_top_3(&input);
  println!("The elves holding the maximum calories has {} calories", result);
}

fn convert_input(raw_input: Vec<String>) -> Vec<Vec<i32>> {
  let mut input: Vec<Vec<i32>> = Vec::new();
  let mut current_group: Vec<i32> = Vec::new();

  for line in raw_input {
    match line.as_str() {
      "" => {
        input.push(current_group);
        current_group = Vec::new();
      },
      _ => {
        current_group.push(line.parse().unwrap());
      }
    }
  }

  input
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
