use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").unwrap();
  let lines = input.lines();
  let instructions = lines.map(|line| line.splitn(2, " "));

  let mut aim = 0;
  let mut horizontal = 0;
  let mut depth = 0;
  
  for mut i in instructions {
    let direction = i.next().unwrap();
    let number: i32 = i.next().unwrap().parse().unwrap();
    match direction {
      "forward" => {
        horizontal += number;
        depth += number * aim;
      },
      "up" => aim -= number,
      "down" => aim += number,
      _ => (),
    }
  }
  println!("{}", depth * horizontal);
}
