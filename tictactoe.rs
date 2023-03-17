use std::io;

const BOARD_SIZE: usize = 9;

fn main() {
    println!("Welcome to Tic Tac Toe!");
    let mut board = ['-'; BOARD_SIZE];
    let mut empty_cells = BOARD_SIZE;
    let mut player = 'X';

    loop {
        display_board(&board);
        let index = get_player_move(&player, &board);
        board[index] = player;
        empty_cells -= 1;
        if is_winner(&board, &player) {
            println!("Player {} wins!", player);
            break;
        }
        if empty_cells == 0 {
            println!("The game is a tie!");
            break;
        }
        player = get_next_player(&player);
    }
}

fn display_board(board: &[char; BOARD_SIZE]) {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("-----------");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("-----------");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

fn get_player_move(player: &char, board: &[char; BOARD_SIZE]) -> usize {
    loop {
        println!("Player {}, enter your move (1-9):", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        let index = match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= BOARD_SIZE && board[num - 1] == '-' => num - 1,
            _ => {
                println!("Invalid move. Please try again.");
                continue;
            }
        };
        return index;
    }
}

fn is_winner(board: &[char; BOARD_SIZE], player: &char) -> bool {
    let wins = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for win in &wins {
        if board[win[0]] == *player && board[win[1]] == *player && board[win[2]] == *player {
            return true;
        }
    }
    return false;
}

fn get_next_player(current_player: &char) -> char {
    if *current_player == 'X' {
        return 'O';
    } else {
        return 'X';
    }
}
