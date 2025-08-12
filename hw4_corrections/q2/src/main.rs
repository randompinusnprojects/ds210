use std::io;
use std::io::Write;

fn main() {
    let mut size = String::new();
    print!("enter size: ");
    io::stdout().flush().expect("Error flushing");
    io::stdin().read_line(&mut size);
    let size = size.trim().parse::<usize>().unwrap();

    let mut iter = String::new();
    print!("enter number of iterations to watch: ");
    io::stdout().flush().expect("Error flushing");
    io::stdin().read_line(&mut iter);
    let iter = iter.trim().parse::<usize>().unwrap();
    
    let init = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    let mut board = vec![vec![0;size];size];

    

    
    for &(x, y) in &init {
        board[x][y] = 1;
    }

    for n in 0..iter {
        println!("\n{}th iteration: ", n+1);
        display(&board);
        board = update(&board);
    }

}


fn update(old: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let mut new = old.clone();
    let size = old[0].len();

    for i in 0..size {
        for j in 0..size {
            new[i][j] = isitalive(old, i, j);
        }
    }

    return new;
    
}

fn isitalive(old: &Vec<Vec<usize>>, i:usize, j:usize) -> usize {
    let mut sum = 0;
    let size = old[0].len();
    let diff:[isize;3] = [-1, 0, 1];

    for x_diff in diff {
        for y_diff in diff {
            if x_diff == 0 && y_diff == 0 { // exclude the point itself
                continue;
            }

            else {
                let nx = ((i as isize + size as isize + x_diff) % size as isize) as usize;
                let ny = ((j as isize + size as isize + y_diff) % size as isize) as usize;
                sum += old[nx][ny];
            }
        }
    }

    if sum == 3 {
        return 1;
    }

    else {
        if sum != 2 {
            return 0;
        }

        else { return old[i][j]; }
    }
}

fn display(board: &Vec<Vec<usize>>) {
    for row in board {
        println!("{:?}", row);
    }
}

#[test]
fn testme() { // test with 3x3 matrix with 3 ones -> everything becomes ones
    let mut testboard = vec![vec![0;3];3];
    testboard[0][0] = 1;
    testboard[1][1] = 1;
    testboard[2][1] = 1;

    let updated_testboard = update(&testboard);

    let mut answerkey = vec![vec![1;3];3];

    assert_eq!(updated_testboard, answerkey, "Something is wrong");

}