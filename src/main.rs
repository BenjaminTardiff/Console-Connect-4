use std::io::{self, Read};
use std::ops::Deref;

#[derive(PartialEq, Debug)]
enum Piece {
    Red,
    Yellow,
}
#[derive(Debug)]
struct Board {
    data: [Option<Piece>; 42],
}

impl Board {
    fn print_board_state(&self) {
        println!(" 1   2   3   4   5   6   7");
        for i in 0..6 {
            let mut s = String::new();
            for j in 0..7 {
                match &self.data[j + i * 7] {
                    Some(piece) => match piece {
                        Piece::Red => s = s + "[r] ",
                        Piece::Yellow => s = s + "[y] ",
                    },
                    None => s = s + "[-] ",
                }
            }
            println!("{}", s);
        }
    }

    fn check_for_row(&self, start_index: usize) -> bool {
        let mut color:Option<Piece>;
        let mut row_counter: u8;
        if start_index < 3 || {6 < start_index && start_index < 10} || {13 < start_index && start_index < 17} || {20 < start_index && start_index < 24} || {27 < start_index && start_index < 31} || {34 < start_index && start_index < 38} {} 
        else {
            for i in 0..2 {
                row_counter = 0;
                if i == 0 {
                    color = Some(Piece::Red);
                } else {
                    color = Some(Piece::Yellow);
                }
                for j in 0..4 {
                    if self.data[start_index - j] == color {
                        row_counter += 1;
                    }
                    if row_counter == 4 {
                        return true
                    }
                }
            }
        }
        return false;
    }

    fn check_for_column(&self, start_index: usize) -> bool {
        let mut color:Option<Piece>;
        let mut column_counter: u8;
        if start_index > 20 {
            for i in 0..2 {
                column_counter = 0;
                if i == 0 {
                    color = Some(Piece::Red)
                } else {
                    color = Some(Piece::Yellow)
                }
                for j in 0..4 {
                    if self.data[start_index - j * 7] == color {
                        column_counter += 1;
                    }
                    if column_counter == 4 {
                        return true
                    }
                }
            }
        }
        return false
    }

    fn check_for_diagonal_positive(&self, start_index: usize) -> bool {
        let mut color:Option<Piece>;
        let mut diagonal_counter:u8;
        if start_index > 20 && {start_index < 4 || start_index > 6} && {start_index < 11 || start_index > 13} && {start_index < 18 || start_index > 20} && {start_index < 25 || start_index > 27} && {start_index < 32 || start_index > 34} && start_index < 39  {
            for i in 0..2 {
                diagonal_counter = 0;
                if i == 0 {
                    color = Some(Piece::Red)
                } else {
                    color = Some(Piece::Yellow)
                }
                for j in 0..4 {
                    if self.data[start_index - 6 * j] == color {
                        diagonal_counter += 1;
                    }
                    if diagonal_counter == 4 {
                        return true
                    }
                }
            }
        }
        return false
    }

    fn check_for_win(&self) -> bool {
        for i in 0..42 {
            if self.check_for_column(i) || self.check_for_row(i) || self.check_for_diagonal_positive(i) {
                return true
            }
        }
        return false;
    }

    fn make_move(&mut self, player: &str) {
        println!("\n{player}'s Move:");
        self.print_board_state();
        println!("What column do you want to put your piece into?");

        let player_move = loop {
            let mut player_move: String = String::new();
            io::stdin().read_line(&mut player_move);
            let player_move = player_move.trim().parse::<usize>();
    
            let Ok(red_move_int) = player_move else {
                println!("player_move is invalid");
                continue;
            };
    
            if red_move_int > 7 || red_move_int < 1 || self.data[red_move_int - 1] != None {
                println!("You entered an invalid number.")
            } else {
                break red_move_int;
            };
        };
        println!("{}'s move was {}.", player, player_move);
    
        for i in 0..6 {
            if self.data[player_move - 1 + (5 - i) * 7] == None {
                if player == "Red" {
                    self.data[player_move - 1 + (5 - i) * 7] = Some(Piece::Red);
                }   else if player == "Yellow" {
                    self.data[player_move - 1 + (5 - i) * 7] = Some(Piece::Yellow);
                };
                break;
            };
        };
        self.print_board_state();
    }
}

fn main() {
    println!("***CONNECT 4***");
    const EMPTY_PIECE: Option<Piece> = None;
    let mut board = Board {
        data: [EMPTY_PIECE; 42],
    };
    println!("You have to play with your friend bc im lazy lol");

    loop {
        board.make_move("Red");
        if board.check_for_win() {
            println!("Red Won!");
            break;
        }

        board.make_move("Yellow");
        if board.check_for_win() {
            println!("Yellow Won");
            break;
        }
    }
}
