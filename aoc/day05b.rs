use std::fs;
use std::mem;

const N: usize = 1000;

fn main() {
    let mut grid = [[0; N]; N];
    let input = fs::read_to_string("src/input.txt").expect("Failed to read file");
    for line in input.lines() {
        let foo: Vec<&str> = line.split(' ').collect();
        let (x1, y1) = foo[0].split_once(',').unwrap();
        let (x2, y2) = foo[2].split_once(',').unwrap();
        let mut x1: i32 = x1.parse().unwrap();
        let mut y1: i32 = y1.parse().unwrap();
        let mut x2: i32 = x2.parse().unwrap();
        let mut y2: i32 = y2.parse().unwrap();
        
        if x1 == x2 {
            if y1 > y2 {
                mem::swap(&mut y1, &mut y2);
            }
            for y in y1..y2+1 {
                grid[x1 as usize][y as usize] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
            }
            for x in x1..x2+1 {
                grid[x as usize][y1 as usize] += 1;
            }
        } else if x1-x2 == y1-y2 {
            // 10, 7 -> 11, 8 -> 12, 9 -> 13, 10 -> 14, 11 -> 15, 12
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
                mem::swap(&mut y1, &mut y2);
            }
            for i in 0..=(x2-x1) {
                grid[(x1+i) as usize][(y1+i) as usize] += 1
            }
        } else if x1-x2 == y2-y1 {
            // 10, 7 -> 9, 8 -> 8, 9 -> 7, 10 -> 6, 11
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
            }
            if y1 < y2 {
                mem::swap(&mut y1, &mut y2);
            }
            for i in 0..=(x2-x1) {
                grid[(x1+i) as usize][(y1-i) as usize] += 1
            }
        }   
    }    

    let mut count = 0;
    for i in 0..N {
        for j in 0..N {
            if grid[i][j] >= 2 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
