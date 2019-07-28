use colored::*;

pub struct Board {
  pub field: Vec<Vec<&'static str>>,
  pub display: Vec<Vec<&'static str>>,
}

impl Board {
  pub fn create(width: usize, height: usize, mine_count: usize) -> Board {
    let mut field: Vec<Vec<&'static str>> = vec![vec!["0"; width]; height];
    
    use rand;
    for _i in 0..mine_count {
      let x = (rand::random::<f32>() * width as f32) as usize;
      let y = (rand::random::<f32>() * height as f32) as usize;
      field[y][x] = "B";
    }

    for y in 0..height {
      for x in 0 ..width {
        if field[y][x] != "B" {
          let mut count = 0;
          if y > 0 && field[y -1][x] == "B" { count += 1; }
          if y < height -1 && field[y +1][x] == "B" { count += 1; }
          if x > 0 && field[y][x -1] == "B" { count += 1; }
          if x < width -1 && field[y][x +1] == "B" { count += 1; }
          if y > 0 && x > 0 && field[y -1][x -1] == "B" { count += 1; }
          if y > 0 && x < width -1 && field[y -1][x +1] == "B" { count += 1; }
          if y < height -1 && x < width -1 && field[y +1][x +1] == "B" { count += 1; }
          if y < height -1 && x > 0 && field[y +1][x -1] == "B" { count += 1; }

          match count {
            1 => field[y][x] = "1",
            2 => field[y][x] = "2",
            3 => field[y][x] = "3",
            4 => field[y][x] = "4",
            5 => field[y][x] = "5",
            6 => field[y][x] = "6",
            7 => field[y][x] = "7",
            8 => field[y][x] = "8",
            _ => field[y][x] = "0"
          }
        }
      }
    }

    return Board {
      field,
      display: vec![vec![" "; width]; height]
    };
  }

  pub fn reveal(&mut self, x: usize, y: usize) {
    if self.display[y][x] == " " {
      match self.field[y][x] {
        "0" => {
          self.display[y][x] = "0";
          if y > 0 { self.reveal(x, y -1); }
          if y < self.display.len() -1 { self.reveal(x, y +1); }
          if x > 0 { self.reveal(x -1, y); }
          if x < self.display[y].len() -1 { self.reveal(x +1, y); }
          if y > 0 && x > 0 { self.reveal(x -1, y -1); }
          if y > 0 && x < self.display[y].len() -1 { self.reveal(x +1, y -1); }
          if y < self.display.len() -1 && x < self.display[y].len() -1 { self.reveal(x +1, y +1); }
          if y < self.display.len() -1 && x > 0 { self.reveal(x -1, y +1); }
        },
        "B" => self.print_all(),
        _ => self.display[y][x] = self.field[y][x],
      }
    }
  }

  pub fn mark(&mut self, x: usize, y: usize) {
    if self.display[y][x] == " " {
      self.display[y][x] = "M";
    }
  }

  pub fn un_mark(&mut self, x: usize, y: usize) {
    if self.display[y][x] == "M" {
      self.display[y][x] = " ";
    }
  }

  pub fn print(&self) {
    print!("   |");
    for i in 0..self.display[0].len() {
      if i < 10 { print!(" {} |", i.to_string().green()); }
      else if i < 100 { print!(" {}|", i.to_string().green()); }
      else  { print!("{}|", i.to_string().green()); }
    }
    println!();
    for _i in 0..self.display[0].len() {
       print!("----");
    }
    print!("----");
    println!();
    for y in 0..self.display.len() {
      if y < 10 { print!("{}  | ", y.to_string().green()); }
      else if y < 100 { print!("{} | ", y.to_string().green()); }
      else { print!("{}| ", y.to_string().green()); }
      
      for x in 0..self.display[y].len() {
        if self.display[y][x] != "B" {
          print!("{} | ", self.display[y][x].cyan().bold());
        } else {
          print!("{} | ", self.display[y][x].red().bold());
        }
      }
      println!();
      for _i in 0..self.display[y].len() {
        print!("----");
      }
      print!("----");
      println!();
    }
  }

  fn print_all(&self) {
    print!("   |");
    for i in 0..self.field[0].len() {
      if i < 10 { print!(" {} |", i.to_string().green()); }
      else if i < 100 { print!(" {}|", i.to_string().green()); }
      else  { print!("{}|", i.to_string().green()); }
    }
    println!();
    for _i in 0..self.field[0].len() {
       print!("----");
    }
    print!("----");
    println!();
    for y in 0..self.field.len() {
      if y < 10 { print!("{}  | ", y.to_string().green()); }
      else if y < 100 { print!("{} | ", y.to_string().green()); }
      else { print!("{}| ", y.to_string().green()); }
      
      for x in 0..self.field[y].len() {
        if self.field[y][x] != "B" {
          print!("{} | ", self.field[y][x].cyan().bold());
        } else {
          print!("{} | ", self.field[y][x].red().bold());
        }
      }
      println!();
      for _i in 0..self.field[y].len() {
        print!("----");
      }
      print!("----");
      println!();
    }
  }
}