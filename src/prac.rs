use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

struct Rect {
    width: i32,
    height: i32,
}

// Added 'derive' so we can compare and debug print the enum easily
#[derive(Debug, PartialEq)]
enum Direction { // we use enums for fixed things like btns and directions etc  
    Up,
    Down,
    Right,
    Left,
    Esc,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn debug() -> i32 {
        1
    }
}


fn move_func() {
    enable_raw_mode().unwrap();
    // In Raw Mode, we use \r\n to ensure the cursor starts at the left
    println!("Control the Rect! Press Arrow Keys (Esc to quit)\r");

    loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            
            let dir = match code {
                KeyCode::Up => Direction::Up,
                KeyCode::Down => Direction::Down,
                KeyCode::Left => Direction::Left,
                KeyCode::Right => Direction::Right,
                KeyCode::Esc => Direction::Esc,
                _ => continue,
            };
            match dir {
                Direction::Up => println!("Yay bro! Moving Up\r"),
                Direction::Down => println!("Yay bro! Moving Down\r"),
                Direction::Left => println!("Yay bro! Moving Left\r"),
                Direction::Right => println!("Yay bro! Moving Right\r"),
                Direction::Esc => break, // Exit the loop
            }
        }
    }
    disable_raw_mode().unwrap();
}

fn main() {
    let msg = "Please enter a valid number broo : !";
    
    println!("Enter Width:");
    let mut w = String::new();
    io::stdin().read_line(&mut w).expect(msg);
    let width: i32 = w.trim().parse().unwrap_or(0);

    println!("Enter Height:");
    let mut h = String::new();
    io::stdin().read_line(&mut h).expect(msg);
    let height: i32 = h.trim().parse().unwrap_or(0);

    let rect = Rect { width, height };

    println!("Show stats? (true/false):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect(msg);
    
    if choice.trim().parse::<bool>().unwrap_or(false) {
        println!("Area: {}", rect.area());
        println!("Debug: {}", Rect::debug());
    }

    move_func();
}