// String

// fn main() {
//     let name: String = String::from("Joy Majumdar");
//     println!("Name : {}\nFirst word is : {}", name,name.chars().nth(0).unwrap());
// }

// &str means string slices

// fn take_slice (slice : &str) {
//     println!("Got : {:?}",slice);
// }

// fn main() {
//     let s : String = String::from("Hello");
//     take_slice(&s);

// }

// Indexing

// fn main(){

// }

// Conditionals, loops

// for loop
// fn main() {
//     let a = 84;
//     let b = 46;
//     if a == b {
//         print!("Helo");
//     } else {
//         print!("Hi");
//     }
// }

// loops

// fn main() {
//     for i in 0..144 {
//         print!("{} ", i);
//     }
// }
// depth loops concept
// fn main() {
//     // array, maps, strings
//     let sentence = String::from("Joy Majumdar");
//     let first_word: String = get_first_word(sentence);
//     println!("First word is : {}", first_word);

//     for _i in 0..10 {
//         println!("Hello Joeeeeeeee")
//     }
// }
// // ownership part this is

// fn get_first_word(sentence: String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// function
// fn main() {
//     let a = 32;
//     let b = 12;
//     let sum = do_sum(a, b);
//     println!("Sum of {} and {} is {}", a, b, sum);
// }
// fn do_sum(a: i32, b: i32) -> i32 {
//     return a + b;
// }

// Mutability

// fn main() {
//     let a: i32 = 23;
//     a = 23; // Errorr because immutable variables cant change
//     println!("{:?}", a);
// }

// fn main() {
//     let mut a: i32 = 23;
//     a = 23; // no error
//     println!("{:?}", a);
// }

// use std::io;

// input
// use std::io;
// fn main() {
//     let mut input = String::new(); // create a str var
//     io::stdin() // get the standard input stream
//         .read_line(&mut input)
//         .expect("Unable to read");
//     println!("{:?}",input);
// }
// fn main() {
// println!("Hello World");
// let a : i8 = 69;
// let  b : i8= 60;
// if a>b || b>a {
//     println!("Hello : {}",a-b);
// } else {
//     println!("Ups sorry!");
// }
// let a = String::from("Joy Majumdar");
// println!("{}\n",a);
// println!("{}",a.chars().nth(4).unwrap());
// let is_even = false;
// if is_even {
//     println!("Hello Even");
// } else {
//     println!("Hello Odd");
// }
// for i in 1..11 {
//     print!("{} ",i);
// }
// let mut n = String::new();
// // println!("Enter Something here : ");
// io::stdin().read_line(&mut n).expect("Error here");
// let n1: i32 = n.trim().parse().expect("Not a valid number!");
// for n1 in 1..101 {

// }
// if n1==1 {
//     println!("I hate it");
// } else if n1==2 {
//     println!("I hate that I love it");
// } else if n1==3 {
//     println!("I hate that I love that I hate it");
// }
// print!("Typed here : {}",input);
// let mut count = 0;

// loop {
//     println!("Count = {}", count);
//     count += 1;

//     if count == 5 {
//         break;  // stops the loop
//     }
// }
// }

// fn main() {
//     let sentence = String::from("Hello World");
//     let first_word = get_first_word(sentence);
//     println!("First word is : {}", first_word);
// }

// pub fn get_first_word(s: String) -> String {
//     let mut ans = String::new();
//     for char in s.chars() {
//         if char == ' ' {
//             break;
//         }
//         ans.push(char);
//     }
//     ans
// }

// fn main() {
//     // let a = 10;
//     // let b = 20;
//     // let sum = do_sum(a, b);
//     // println!("Sum of {} and {} is {}", a, b, sum);
//     let mut a : i32 = 25;
//     a =35;
//     println!("{:?}",a);
// }

// fn do_sum(a: i32,  b: i32)-> i32 {
//     let n = 12;
//     for i in 0..n {
//         if a+b == 40 {
//             return a + b;
//         } else {
//             println!("Not equal to 40");
//         }
//     }
//     return 0;
// }

// fn main() {
//     let s = String::from("Hello");
//     let s2 = sname(s);
//     println!("{}", s2);

// }

// fn sname(some_new: String) {
//     println!("Inside sname function : {}", some_new);
// println!("{}", s);
// // s.push_str(", world!"); // push_str() appends a literal to a String
// println!("{}", s);

// for _ in 0..100 {
//     s.push_str(", world!");
//     println!("Capacity : {} Length is : {} Pointer address : {:p}", s.capacity(),s.len(),s.as_ptr());
// }
// println!("{}", s);
// }

// Ownership

// fn main(){
//     let s = String::from("Hello, world!");
//     take_ownership(s.clone());
//     println!("Hello Ex im back : {}", s);
// }

// fn take_ownership(some_string: String)  {
//     println!("Firstly printed New One : Inside takeOwnership function : {}", some_string);
//     // return some_string;
// }

// borrowing

// fn main() {
//     // let s = String::from("Hello");
//     // let s2 = &s;
//     // println!("s2 is : {}", s2);
//     // println!("s is : {}", s);
//     // let my_string = String::from("Hello, world!");
//     // borrow(&my_string);
//     // println!("Back in main : {}", my_string);
//     let mut s = String::from("Hello, world!");
//     borrow(&mut s);
//     println!("Back in main : {}", s);
// }

// fn borrow(some_string: &mut String)  {
//     some_string.push_str("\nWelcome to Rust World!");
// }

// Struct

// use std::io;

// struct User {
//     active: bool,
//     name: String,
//     email: String,
//     sign_in_count: u64,
//     age: u8,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         name: String::from("Joy Majumdar"),
//         email: String::from("a@gmail.com"),
//         age: 21,
//         sign_in_count: 1,
//     };
//     if user1.active {
//         let _n: i32;
//         let mut input = String::new();

//         print!("Enter a number: ");
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         let n: i32 = input.trim().parse().expect("Please enter a valid number");
//         for i in 0..n {
//             println!(
//             "User is active\nName : {}\nEmail : {}\nAge : {}\nSign In : {:?}",
//             user1.name, user1.email, user1.age, user1.sign_in_count+i as u64
//         );
//         println!(" ")
//         }
//         // println!(
//         //     "User is active\nName : {}\nEmail : {}\nAge : {}\nSign In : {}",
//         //     user1.name, user1.email, user1.age, user1.sign_in_count
//         // );
//     } else {
//         println!("User is not active");
//     }
// }

// struct UserDetails {
//     active: bool,
//     name: String,
//     email: String,
//     age: u8,
//     class: String,
// }
// struct UserDetails {
//     age : u8,
//     id : u8,
// }
// impl UserDetails {
//     fn UserD(&self)-> u32 {
//         self.age as u32 + self.id as u32
//     }
// }
// fn main() {
//     let user_d = UserDetails {
//         age: 24,
//         id: 12,
//     };
//     let total = user_d.UserD();
//     println!("Total is : {}", total);
// }
// fn main() {
//     let mut user_d = UserDetails {
//         active: true,
//         name: String::from("Joy"),
//         age: 24,
//         email: String::from("Aadjsdja@gmail.com"),
//         class: String::from("BSCSE"),
//     };
//     for i in 0..5  {
//         if user_d.active {
//             let new_age = user_d.age + i;
//             if user_d.name == "Joy" {
//                 (user_d.name).push_str(" Majumdar");
//             } else if user_d.name == "Joy Majumdar" {
//                 user_d.name.push_str("Joy M.");
//             }
//             println!(
//                 "User is active\nName : {}\nEmail : {}\nAge : {}\nClass : {}",
//                 user_d.name, user_d.email, new_age, user_d.class
//             );
//         }
//         println!(" ");
//     }
// }

// struct User{
//     age: u8,
//     active: bool,
//     name: String,
//     email: String,
//     // ÷: u16,
// }

// fn main() {
//     let user = User{
//         age: 21,
//         active: true,
//         name: String::from("Joy Majumdar"),
//         email: String::from("Ag@g.com"),
//     };
//     println!("User Details:\nName: {}\nEmail: {}\nAge: {}\nActive: {}", user.name, user.email, user.age, user.active);
// }

// implimenting struct

// struct React{
//     width: u32,
//     height: u32,
// }

// impl React{
//     fn area(&self) -> u32 {
//        return self.width * self.height;
//     }
//     fn sum(&self) -> u32 {
//         return self.width + self.height;
//     }
// }
// fn main() {
//     let rect1 = React{
//         width: 30,
//         height: 50,
//     };
//     println!("Sum is {}",rect1.sum());
//     println!("Area is : {}", rect1.area());
// }

// structs

// struct User{
//     active: bool,
//     username: String,
//     email: String,
//     login_count: i32,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("Joy Majumder"),
//         email: String::from("gopal@g.com"),
//         login_count: 1,
//     };
//     if user1.active==true {
//         for i in 0..5  {
//             print!("Username:{:?}\nEmail:{:?}\nCount:{:?}",user1.username,user1.email,user1.login_count);
//             print!("\n");
//         }
//     } else {
//         print!("DB Error");
//     }
// }

// #[warn(non_camel_case_types)]

// #[allow(dead_code)]

// mod random_info;
// use random_info::*;
// struct dtata {
//     some_bool: bool,
//     some_int: i32,
//     some_float: f32,
//     randamD: random,
// }
// fn main() {
//     let mut data = dtata{
//         some_bool: true,
//         some_float: 3.232,
//         some_int:232,
//         randamD: random::new(true),
//     };
//     data.some_int = 232;
//     let data2 = dtata{
//         some_int: 456789,
//         ..data // ..data because just changed some_int var value and other variable values are same as first so here it is!
//     };
//     println!("{:?}, {:?} {:?} {:?}",data2.some_bool,data2.some_int,data2.some_float,data2.randamD);
// }

// impl struct

// struct rect{
//     width: u32,
//     heigh: u32,
// }

// impl rect{
//     fn area(&self) -> u32 {
//         self.width*self.heigh
//     }
// }

// #[allow(unused_variables)]
// fn main(){
//     let rect = rect{
//         width:30,
//         heigh:20,
//     };
//     println!("Area of its is : {:?}",rect.area());
// }
// enums

// enum direc {
//     north,
//     south,
//     east,
//     west,
// }

// fn main() {
//     let myDic = direc::north;

// }

// Patter matching

// const PI: f64 = 3.1416;
// #[allow(dead_code)]
// enum shape {
//     circle(f64),
//     square(f64, f64),
//     rect(f64),
// }

// fn cal_shape(shape: shape) -> f64 {
//     match shape {
//         shape::circle(radius) => PI * (radius * radius),
//         shape::rect(side_vale) => side_vale * side_vale,
//         shape::square(width, heigth) => width * heigth,
//     }
// }

// fn main() {
//     let cir1 = shape::circle(4.00);
//     let sq1 = shape::square(4.00, 5.000);
//     let recr1 = shape::rect(3.00);

//     println!(
//         "Circle is : {:?}\nRectangle is : {:?}\nSquare is : {:?}",
//         cal_shape(cir1),
//         cal_shape(sq1),
//         cal_shape(recr1)
//     );
// }

// We can also print the length of space
// fn main(){
//     lang();
// }

// fn lang(){
//     let space = "   ";
//     println!("{}",space.len()); // here it prints the value of its length is 3
// }

// Functions
// fn main(){
//     let a:i32=15;
//     let b:i32 = 32;
//     let sum = do_sum(a, b);
//     println!("The sum of {a} and {b} is : {sum}");
// }

// fn do_sum(a:i32,b:i32) -> i32{ // if we dont define return type then it wont work so we 
//                                // we always should use return type.
//     return a+b;
// }

// use simple_user_input::get_input;

// fn main(){
//     let input: String = get_input("Please type something Birooo");
//     println!("{}",input);
// }

// mod simple_user_input {
//     use std::io;
//     pub fn get_input(prompt: &str) -> String{
//         println!("{}",prompt);
//         let mut input = String::new();
//         match io::stdin().read_line(&mut input) {
//             Ok(_goes_into_input_above) => {},
//             Err(_no_updates_is_fine) => {},
//         }
//         input.trim().to_string()
//     }
// }

// fn main(){
//     let mut x = String::from("Hello ");
//     x.push_str("Joy");
//     println!("{x}");
// }

// fn main(){
//     stack_fn();
//     heap_fn();
//     update_fn();
// }

// fn stack_fn(){
//     let a = 10;
//     let b = 20;
//     let c = a+b;
//     println!("Stack function : the sum of {a} and {b} is : {c}");
// }

// fn heap_fn(){
//     let s1 = String::from("Hello");
//     let s2 = String::from("Joy");

//     let combined = format!("{s1} {s2}");
//     println!("The combined string is : {combined}");
// }

// fn update_fn(){
//     let mut s = String::from("Hello intial String");
//     println!("Before update : {s} also pointer is : {:p} length{}",s.as_ptr(), s.len());

//     // append some text to the string
//     s.push_str(" And we welcome you Joy G. Majumdar");
//     println!("Here is updated version of string : {s}");
// }


// OwnerShip

// fn main(){
//     // let x =1; // created on stack
//     // let y = 3;
//     // {
//     //     let y = 2; // scoping variable in the same fn, created on stack also we can not access y outer of {} so yea
//     //     println!("{y}");
//     // }
//     // println!("{x} {y}");
//     let s1 = String::from("Hello");
//     //let s2 = s1; when do this s1 got invalid and its value got assign to s2 so if wee need s1 also then we shpould do like this  make a clone of s1 so s1.clone();
//     // let s2 = s1.clone();
//     let s2 = &s1; // it means we borrow it from main str,
//     println!("{s2}");
//     //println!("{s1}"); // when we assign our s1 var to s2 it always done something like it wont work i mean by this when we do s22=s1 then s1 got invalid and s1's value got assigned to s2 so yayy
//     println!("{s1}");
// }

// Borrowing 

// use std::io::{self, Read};

// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     println!("Typed : {input}");
//     // take_ownership(input);
//     borrow(&input);
//     update(&mut input);
//     print!("Updated version : {input}");
// }
// // &s1 means reference
// fn take_ownership(some_string:String) -> String{
//     println!("{}",some_string);
//     return some_string;
// }
// fn borrow(some_string:&String) -> &String{
//     println!("{}",some_string);
//     return some_string;
// }
// fn update(input: &mut String){
//     input.push_str(" Word");
// }

// Structs -> structures data
// #[derive(Debug)]
// struct user{
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u32,
// }

// fn main(){
//     let user1 = user{
//         active: true,
//         username: String::from("ada@asda.com"),
//         email: String::from("ada"),
//         sign_in_count:1,
//     };
//     for _i in 0..10{
//         println!("The name is : {:?} and email : {:?}",user1.username,user1.email);
//     };
// }

// Impl(Impliment) Structs

// struct Rect{
//     width : u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&mut self) -> u32{
//         return self.width * self.height
//     }
//     fn perameter(&self) -> u32 {
//         self.width * self.width
//     }
// }

// fn main(){
//     let mut rect1 = Rect{
//         width : 3,
//         height:2,
//     };
//     let mut power:i128 = 232;
   
//     println!("the area of the rect is : {}\nthe perameter of this is : {}",rect1.area(),rect1.perameter());
// }


// Enums 

// enum Direction{
//     North,
//     East,
//     South,
//     West,
// }

// fn EnumSHow(){
//     let mydirection = Direction::North;
//     // let new_direction = mydirection; // No error, Because Direction is Copy
//     move_around(mydirection);
// }
// fn move_around(direction: Direction){
//     // impl logic to move a character around

// }
// fn main(){
//     EnumSHow();
// }

// pattern matching

// const PI: f64 = 3.1416;

// #[allow(dead_code)]
// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
//     Triangle(f64,f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => PI * radius * radius,
//         Shape::Square(side) => side * side,
//         Shape::Rectangle(width, height) => width * height,
//         Shape::Triangle(base,height) => 0.5 * base * height,
//     }
    // we can do it using if else also but it makes verbose our code
    // if let Shape::Circle(radius) = shape {
    //     println!("Circle radius: {}", radius);
    //     PI * radius * radius
    // } else if let Shape::Square(side) = shape {
    //     println!("Square side: {}", side);
    //     side * side
    // } else if let Shape::Rectangle(width, height) = shape {
    //     println!("Rectangle width: {} height: {}", width, height);
    //     width * height
    // } else {
    //     0.0
    // }
// }

// fn main() {
//     // create instances of different shapes
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rect = Shape::Rectangle(1.0, 2.0);
//     let triangle = Shape::Triangle(2.00, 3.00);

//     println!("The area of circle is: {}", calculate_area(circle));
//     println!("The area of square is: {}", calculate_area(square));
//     println!("The area of rectangle is: {}", calculate_area(rect));
//     println!("The area of Triangle is: {}", calculate_area(triangle));
// }


// Error Handling

// struct Point<T>{ // Here T is generics Type
//     x: T,
//     y: T,
// }

// use std::{fmt::Error, fs, task::Context};

// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }

// fn main(){
//     // let intValue = Point {x:2 , y:3};
//     // let floatValue = Point {x : 1.00 , y :2.00};
//     // println!("The value of sum of int is : {}",intValue.x+intValue.y);
//     // println!("The value of sum of float is : {}",floatValue.x+floatValue.y);
//     // there is a fn that can err out/stop the thread
//     let ans: Result<Vec<u8>, std::io::Error> = fs::read("Err.txt");
//     // let ans = read_from_file_unsafe("Err1.txt".to_string());
//     match ans {
//         Ok(content) => {
//             println!("File content : {:?}",content);
//         },
//         Err(err) => {
//             println!("Error: {:?}",err);
//         }
//     }
//     println!("Hello world");
// }



// fn read_from_file_unsafe(file_content:String) -> String{ // we will get this err and it gets crash so we dont choose this 
// // thread 'main' (114612) panicked at main.rs:795:16:
// // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
// // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//     let ans = fs::read_to_string("Err1.txt");
//     // return ans.unwrap();

// use std::ops::Index;

//     // btw we can do this :
//     match ans{
    //         Ok(content:String)=> Ok(content),
    //         Err(_)=> Err("Error to read the file"),
    //     }
// }
 
    // Option Enum 
// fn find_first(s: String) -> Option<usize>{
//     for (index,character) in s.chars().enumerate() {
//         if character == 'o'{
//             return Some(index);
//         }
//     }
//     return None;
// }

// fn main() {
//     let my_string = String::from("Joy");
//     match find_first(my_string) {
//         Some(index) => println!("The letter a is found at idenx : {index}"),
//         None => println!("The letter a isnt found sooo saddd babe"),
//     }
// }


// #[derive(Debug)]
// enum IPAddKind {
//     V4,
//     v6,
// }

// #[derive(Debug)]
// struct ipKind{
//     kind : IPAddKind,
//     address: String,
// }

// fn main() {
//     let home = ipKind{
//         kind : IPAddKind::V4,
//         address : String::from("127.0.0.1"),
//     };
//     println!("The value is : {:?}",home);
    
// }

// use core::num;
// use std::io;

// //Q1
// fn main() {
//   let mut input = String::new();
//   let msg = String::from("Error to read");
//   io::stdin().read_line(&mut input).unwrap();
//   let input :i32 = input.trim().parse().expect(&msg);

// //   let input_number = match input.trim().parse(){
// //     Ok(num) => num,
// //     Err(_)=>{
// //         println!("Enter a vlid number Birooo");
// //         return;
// //     }
// //   };

//   if input >= 12 {
//     println!("Yes");
//   } else {
//     println!("No");
//   }
// }
// fn main() {
//     let mut input = String::new();
//     let msg = String::from("Error to read");
//     println!("Enter a number : ");
//     let ans = io::stdin().read_line(&mut input).expect(&msg);
//     println!("Ans is : {ans}");

// }
 

// struct Point { x: i32, y: i32 }

// fn main() {
//     let p = Point { x: 0, y: 1 };

// match p {
//     Point { x: 0, y } => println!("On Y-axis at {y}"),
//     Point { x, y: 0 } => println!("On X-axis at {x}"),
//     Point { x, y } => println!("At ({x}, {y})"),
// }
// let x = 12121;

// match x {
//     0 => println!("zero"),
//     1..=10 => println!("small"),
//     n if n % 2 == 0 => println!("even number {n}"), // guard
//     _ => println!("other"),
// }
// }


// Option enum 

// use std::char;

// fn main(){
//     let index = String::from("Jasdkasndkasoy");
//     match find_first_o(index) {
//         Some(value)=> println!("Found {value}"),
//         None => println!("Sad bt value cant found"),
//     }
// }

// fn find_first_o(s: String) -> Option<i32> {
//     for (index, c) in s.chars().enumerate() { // ← clear name, proper open
//         if c == 'o' {
//             return Some(index as i32);
//         } // ← closes `if`
//     } // ← closes `for`
//     None // ← idiomatic trailing expression (no `return` needed)
// } // ← closes `fn`


// pakage management
// use chrono::{Local, Utc}; // for time

// fn main() { 
//     let now = Utc::now();
//     println!("Current time rn is : {now}");

//     // we will do format here :
//     let formated = now.format("%Y-%m-%d %H:%M:%s");
//     println!("Time formated here is : {formated}");

//     // here we will print out local time brooo
//     let local = Local::now();
//     println!("Current local. time is : {local}");
// }


use std::fmt::Debug;
// practice here 
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
    Keys,
    Numbers,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn debug() -> i32 {
        1
    }
}

#[warn(unreachable_patterns)]
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
                KeyCode::KeypadBegin => Direction::Keys,
                KeyCode::NumLock => Direction::Numbers,
                _ => continue,
            };
            match dir {
                Direction::Up => println!("Yay bro! Moving Up\r"),
                Direction::Down => println!("Yay bro! Moving Down\r"),
                Direction::Left => println!("Yay bro! Moving Left\r"),
                Direction::Right => println!("Yay bro! Moving Right\r"),
                Direction::Keys => println!("Yay bro! You typed Key\r"),
                Direction::Numbers => println!("Yay bro! You Typed nums\r"),
                Direction::Esc => break, // Exit the loop
                // _- => println!("Enter valid btns broc"),

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
    
    move_func();
    
    if choice.trim().parse::<bool>().unwrap_or(false) {
        println!("Area: {}", rect.area());
        println!("Debug: {}", Rect::debug());
    }
    
    if rect.area() >= 100 && rect.area() <= 2500{
        print!("Enter the points u're want to earn from by playing this game? : ");
        let mut points = String::new();
        io::stdin().read_line(&mut points).expect(msg);
        let point :i32 = h.trim().parse().unwrap_or(0);
        println!("Bro You want to play {point} round");

    }
}