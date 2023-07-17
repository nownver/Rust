use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
// use std::io::BufReader;
// use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Pos {
    x:f64,
    y:f64,
}

#[derive(Debug)]
struct Manager {
    name: String,
    age: u32,
    staff_list: Vec<String>,
    pos:Pos,
}

#[derive(Debug)]
struct SoftEng {
    name: String,
    age: u32,
    skills: Vec<String>,
    pos:Pos,
}

#[derive(Debug)]
struct GraphicDesighner {
    name: String,
    age: u32,
    skills: Vec<String>,
    pos:Pos,
}

trait Employee {
    fn get_role(&self) -> String;
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
    fn get_location(&self) -> &Pos;
    // fn get_staff_list(&self) -> Vec<String>;
    // fn get_skills(&self) -> Vec<String>;
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lat = 'E';
        if self.x < 0.0 {
            lat = 'W';
        }
        let mut long = 'N';
        if self.y < 0.0 {
            long = 'S';
        }
        write!(f, "({}{}, {}{})", self.x, lat, self.y, long)
    }
}

impl Employee for Manager{
    fn get_name(&self) -> &String{
        let mut name = &self.name;
        name
    }
    fn get_age(&self) -> u32{
        let mut age = self.age;
        age
    }
    fn get_location(&self)-> &Pos{
        let mut pos = &self.pos;
        pos
    }
    fn get_role(&self) -> String{
        let mut role = "Manager".to_string();
        role
    }
}

impl Employee for SoftEng{
    fn get_name(&self) -> &String{
        let mut name = &self.name;
        name
    }
    fn get_age(&self) -> u32{
        let mut age = self.age;
        age
    }
    fn get_location(&self)-> &Pos{
        let mut pos = &self.pos;
        pos
    }
    fn get_role(&self) -> String{
        let mut role = "Software engineer".to_string();
        role
    }
}

impl Employee for GraphicDesighner{
    fn get_name(&self) -> &String{
        let mut name = &self.name;
        name
    }
    fn get_age(&self) -> u32{
        let mut age = self.age;
        age
    }
    fn get_location(&self)-> &Pos{
        let mut pos = &self.pos;
        pos
    }
    fn get_role(&self) -> String{
        let mut role = "Graphic Desighner".to_string();
        role
    }
}

impl fmt::Display for Manager {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut role = self.get_role();
        let mut name = self.get_name();
        let mut location = self.get_location();
        let mut age = self.get_age();
        write!(f, "Role: {}\nName: {}\nAge: {}\nLocation: {}\nStaff list {:?}", role, name, age, location, self.staff_list)
    }
}

impl fmt::Display for SoftEng {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut role = self.get_role();
        let mut name = self.get_name();
        let mut location = self.get_location();
        let mut age = self.get_age();
        write!(f, "Role: {}\nName: {}\nAge: {}\nLocation: {}\nSkills: {:?}", role, name, age, location, self.skills)
    }
}

impl fmt::Display for GraphicDesighner {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut role = self.get_role();
        let mut name = self.get_name();
        let mut location = self.get_location();
        let mut age = self.get_age();
        write!(f, "Role: {}\nName: {}\nAge: {}\nLocation: {}\nSkills: {:?}", role, name, age, location, self.skills)
    }
}
 
fn main() {
    let mut input = String::new();
	File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
	println!("{}", input);

    // read_file();
    // let yok = Manager{
    //     name: "Urawee".to_string(),
    //     age: 19,
    //     staff_list: vec!["pao1".to_string(), "pao2".to_string()],
    //     pos:Pos{x:-1.0, y:2.0},
    // };

    // let jeff = SoftEng{
    //     name: "Phurin".to_string(),
    //     age: 18,
    //     skills: vec!["Rust".to_string(), "Python".to_string()],
    //     pos:Pos{x:-90.0, y:8.0},
    // };

    // let pete = GraphicDesighner{
    //     name: "Schnat".to_string(),
    //     age: 18,
    //     skills: vec!["Painting".to_string(), "Logo design".to_string()],
    //     pos:Pos{x:-90.0, y:8.0},
    // };

    // println!("{}", yok);
    // println!();
    // println!("{}", jeff);
    // println!();
    // println!("{}", pete);    
}
const COLON: char = ':';
const NEW_LINE: char = '\n';

// fn read_file(){
//     let mut input = String::new();
// 	File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
	
//     let mut word_by_line : Vec<Vec<&str>> = vec!();
//     let lines: Vec<&str> = input.split(NEW_LINE).collect();
//     let n_lines = lines.len();
//     for j in 0..n_lines {
//         let t_set: Vec<&str> = lines[j].trim_start().split(COLON).collect();
//         // println!("{:?}", t_set);
//         word_by_line.push(t_set);
//     }
//     println!("{:?}", word_by_line);
//     // word_by_line
// }

// fn read_file(){
//     let file = File::open("input.txt").expect("Unable to open file");
//     let reader = BufReader::new(file);

//     let mut people = Vec::new();
//     for line in reader.lines() {
//         let line = line.expect("Unable to read line");
//         let mut parts = line.split(',');
//         let name = parts.next().expect("Unable to read name");
//         let age = parts.next().expect("Unable to read age");
//         let age = age.parse().expect("Unable to parse age");
//         let person = Person { name: name.to_string(), age ,pos,staff_list};
//         people.push(person);
//     }

//     for person in people {
//         println!("{} is {} years old", person.name, person.age);
//     }

// }

// fn check_word(all_word:Vec<Vec<&str>>){
//     for i in 0..all_word.len(){
//         for j in i{
//             if j == 'Role' || j == 'role'{
//                 let mut role = i[j+1]
//             }
//         }
//     }
// }
