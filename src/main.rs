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
            }
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
            }
        }
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


    board.make_move("Red");

    board.make_move("Yellow")
}
