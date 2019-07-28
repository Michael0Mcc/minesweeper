mod game;

fn main() {
  clear_terminal();
  let mut board = game::Board::create(20, 15, 35);
  board.mark(7, 7);
  board.mark(12, 8);
  board.un_mark(7, 7);
  board.reveal(5, 5);
  board.print();
}

fn clear_terminal() {
  print!("{}[2J", 27 as char);
}