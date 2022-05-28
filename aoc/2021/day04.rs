use std::fs;

type Board = [[(bool, i32); 5]; 5];

fn parse_input(input: &str) -> (Vec<Board>, Vec<i32>) {
    let mut lines = input.lines();

    let numsStr = lines.next().expect("Error with input");
    let nums = numsStr.split(',')
        .map(|n| { n.parse().unwrap() })
        .collect::<Vec<i32>>();

    println!("{:?}", nums);
    
    let mut boards = Vec::new();
    
    while let Some(l) = lines.next() {
        let mut board: Board = [[(false, 0); 5]; 5];
        if l.is_empty() {
           for i in 0..5 {
              let l = lines.next().unwrap();
              let nums = l.split(' ')
                  .filter(|s| !s.is_empty())
                  .map(|n| { n.parse().unwrap() })
                  .collect::<Vec<i32>>();
              for j in 0..5 {
                 board[i][j] = (false, nums[j]);
              }
           }
          boards.push(board);
        }
    }
  
    (boards, nums)
}

fn mark(board: &mut Board, num: i32) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j].1 == num {
                board[i][j].0 = true;
            }
        }
    }
}

fn is_winner(board: &Board) -> bool {
    // row
    for i in 0..5 {
        let mut win = true;
        for j in 0..5 {
            if board[i][j].0 == false {
                win = false;
            }
        }
        if win {
            return true;
        }
    }
    // column
    for i in 0..5 {
        let mut win = true;
        for j in 0..5 {
            if board[j][i].0 == false {
                win = false;
            }
        }
        if win {
            return true;
        }
    }
    false
}

fn sum_unselected(board: &Board) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !board[i][j].0 {
                sum += board[i][j].1;
            }
        }
    }
    sum
}

fn answer(input: &str) -> i32 {
    let (mut boards, nums) = parse_input(input);
    for num in nums {
        for mut board in boards.iter_mut() {
            mark(&mut board, num);
            if is_winner(&board) {
                return sum_unselected(&board) * num;
            }
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error with input");
    println!("{}", answer(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn basic() {
        let input_string = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let (boards,callNums) = parse_input(input_string);
        println!("{:?}", boards);
        assert_eq!(answer(input_string), 4512);
    }
}

