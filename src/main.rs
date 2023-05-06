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
}

fn main() {
    println!("***CONNECT 4***");
    const EMPTY_PIECE: Option<Piece> = None;
    let mut board = Board {
        data: [EMPTY_PIECE; 42],
    };
    println!("You have to play with your friend bc im lazy lol\n");
    println!("Red's Move:");

    board.print_board_state();
    println!("What column do you want to put your piece into?");

    let red_move = loop {
        let mut red_move: String = String::new();
        io::stdin().read_line(&mut red_move);
        let red_move = red_move.trim().parse::<usize>();

        let Ok(red_move_int) = red_move else {
            println!("red_move is invalid");
            continue;
        };

        if red_move_int > 7 || red_move_int < 1 || board.data[red_move_int - 1] != EMPTY_PIECE {
            println!("You entered an invalid number.")
        } else {
            break red_move_int;
        }
    };
    println!("Red's move was {}.", red_move);

    for i in 0..6 {
        if board.data[red_move - 1 + (5 - i) * 7] == EMPTY_PIECE {
            board.data[red_move - 1 + (5 - i) * 7] = Some(Piece::Red);
            break;
        }
    }

    board.print_board_state();
}
