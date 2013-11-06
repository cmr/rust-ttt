use std::str;
use std::io;

use board::*;
use console_io::*;

mod board;
mod console_io;

fn main() {
    println("in main...");

    let board = ::Board::new();
    let io = ::ConsoleIO::new(io::stdin());

    println(io.printable_board(board));

    println("\n\n=========\n\n");
    let line = io.get_move();
    println("\n\n=========\n\n");

    println("preparing to print...\n");
    println(line.to_str());
    println("\nfinished printing");
    println("\n.\n.\n.\n");

    println("done");
}
