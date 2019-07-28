mod game;
mod input;

fn main() {
  let mut board = input::init();
  let _board = &mut board;
  _board.print();
  input::get_command(_board);
}