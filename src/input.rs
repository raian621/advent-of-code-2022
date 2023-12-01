use std::{fs::File, io::{BufReader, BufRead}};

pub fn read(file: File) -> Vec<String> {
  let reader = BufReader::new(file);
  let input: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

  input
}