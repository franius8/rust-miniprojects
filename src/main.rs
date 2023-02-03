use std::io::stdin;

fn main() {
    let mut board = vec![" ".to_owned(); 9];
    let mut marker = "X".to_owned();
    print_welcome_message();
    loop {
        print_board(&board);
        place_marker(&mut board, &marker);
        if check_win(&board, &marker) {
            println!("Game ended, {} won!", marker);
            return;
        }
        marker = if marker == "X" { "O".to_owned() } else { "X".to_owned() };
    }
}

fn place_marker<'a>(board: &'a mut Vec<String>, marker: &'a String) -> &'a Vec<String> {
    let place = collect_move(board.clone());
    board[place] = marker.to_string();
    return board;
}

fn print_welcome_message() {
    println!("-------------");
    println!("|Tic-Tac-Toe|");
    println!("-------------");
}

fn print_board(board: &Vec<String>) {
    println!("-------------");
    println!("| {} | {} | {} |", board[0], board[1], board[2]);
    println!("-------------");
    println!("| {} | {} | {} |", board[3], board[4], board[5]);
    println!("-------------");
    println!("| {} | {} | {} |", board[6], board[7], board[8]);
    println!("-------------");
}

fn collect_move(board: Vec<String>) -> usize {
    println!("Enter the number of the field to place your marker in:");
    loop {
        let input = stdin();
        let mut input_string = String::new();
        input.read_line(&mut input_string).expect("Cannot read from input.");
        let trimmed_string = input_string.trim();
        match trimmed_string.parse::<usize>() {
            Ok(i) => {
                match board.get(i - 1) {
                    Some(text) => {
                        if text == " " {
                            return i - 1;
                        } else {
                            println!("Space already taken. Try again.")
                        }
                    },
                    None => println!("Invalid position entered. Try again.")
                }
            },
            Err(..) => println!("Input is not an integer. Try again."),
        }
    }
}

fn check_win(board: &Vec<String>, marker: &String) -> bool {
    let winning_combinations:[[usize; 3]; 8] = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6],
        [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]];
    for x in winning_combinations {
        let mut counter = 0;
        for position in x {
            if board[position] == marker.to_string() {
                counter += 1;
            }
        }
        if counter == 3 {
            return true;
        }
    }
    return false;
}
