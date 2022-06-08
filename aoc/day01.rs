use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Input failed");
    let strings: Vec<&str> = input.trim().split('\n').collect();
    let nums: Vec<i32> = strings.iter().map(|s| s.parse().unwrap()).collect();
    //println!("{:?}", nums);
    let mut count = 0;
    for i in 0..(nums.len()-1) {
        if nums[i] < nums[i+1] {
            count += 1;
        }
    }
    println!("{}", count);
}
