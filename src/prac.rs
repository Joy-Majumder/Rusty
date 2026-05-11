use std::io;

struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
    fn debug() {
        return ();
    }
}

fn main() {
    let msg: String = String::from("Please enter correct number here brooo");
    println!("Enter the value of A : ");
    let mut w: String = String::new();
    io::stdin().read_line(&mut w).expect(&msg);
    let inti_a: i32 = match w.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter valid number brooo ");
            return;
        }
    };

    println!("Enter the value of B : ");

    let mut h: String = String::new();
    io::stdin().read_line(&mut h).expect(&msg);

    let inti_b: i32 = match h.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter valid number brooo ");
            return;
        }
    };

    let rect = Rect {
        width: inti_a,
        height: inti_b,
    };
    let mut active = String::new();
    io::stdin().read_line(&mut active).expect(&msg);
    let boolean: bool = match active.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter true or false here brooo ");
            return;
        }
    };
    if boolean {
        println!("The area of this is : {:?}", rect.area());
        println!("The Debug of this is : {:?}", Rect::debug());
    }
}
