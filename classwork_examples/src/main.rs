use std::fs::File;
use std::io::prelude::*;

// struct Config {
//     username: String,
//     api_key: String,
//     port: u16,
// }

struct Student{
    name: String,
    major: String,
}

impl Student {
    fn from_file(path: &str) -> Student {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let major = lines.next().unwrap().to_string();


        Student { name, major }
    }
}

fn reading_from_file() {
    let student = student::from_file("student.txt"); // update this line 
    println!("Name: {}", student.name);
    println!("Major: {}", studentg.major);
   
}

fn create_and_write_to_file() {
    let mut file = File::create("student.txt").unwrap();
    writeln!(file, "Fernando").unwrap();
    writeln!(file, "Computer Science.").unwrap();
}


fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What is your major? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let major = buffer.trim().parse().unwrap();

    let person = Person { name, major };
    println!("Hi {}, you are {} years old!", student.name, student.major);
    let file = File::open("student.txt").unwrap();

    writeln!(file, "{}",name).unwrap();
    writeln!(file,"{}", major).unwrap();
    // save to a txt file

}

fn main() {
    reading_from_console();
    reading_from_file();
}