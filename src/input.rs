use colored::*;
use crate::game::Board;

fn get_user_input() -> String {
    use std::io;

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    return input;
}

pub fn init() -> Board {
  println!("{}", "Please enter board WIDTH, HEIGHT, and MINE COUNT separated by whitespace".green());
  let gui = get_user_input();
  let dimensions: Vec<&str> = gui.split_whitespace().collect();
  let x: usize = dimensions[0].parse::<usize>().unwrap();
  let y: usize = dimensions[1].parse::<usize>().unwrap();
  let mc: usize = dimensions[2].parse::<usize>().unwrap();
  clear_terminal();
  return Board::create(x, y, mc);
}

pub fn get_command(board: &mut Board) {
  println!("{}", "Enter x-y to reveal, x-y-m to mark, x-y-um to unmark".green());
  let command: String = get_user_input().to_lowercase().split_whitespace().collect();
  let commands: Vec<&str> = command.split('-').collect();
  let x: usize = commands[0].parse::<usize>().unwrap();
  let y: usize = commands[1].parse::<usize>().unwrap();
  if board.is_in_bounds(x, y) {
    clear_terminal();
    if commands.len() == 3 {
      if commands[2] == "m" {
        board.mark(x, y);
      } else if commands[2] == "um" {
        board.un_mark(x, y)
      }
    } else {
      board.reveal(x, y);
      board.print();
      get_command(board);
    }
  } else {
    println!("{}", "Values out of range".yellow());
  }
}

fn clear_terminal() {
  print!("{}[2J", 27 as char);
}