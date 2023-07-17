use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;

const COMMA: char = ',';
const SPACE: char = ' ';

#[derive(Debug)]
enum EmployeeType {
    Manager,
    SoftEng,
    GraphicDesighner,
}

#[derive(Debug)]
struct Pos {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Manager {
    label: EmployeeType
    name: String,
    age: u32,
    staff_list: Vec<String>,
    pos: Pos,
}

#[derive(Debug)]
struct SoftEng {
    label: EmployeeType
    name: String,
    age: u32,
    skills: Vec<String>,
    pos: Pos,
}

#[derive(Debug)]
struct GraphicDesighner {
    label: EmployeeType
    name: String,
    age: u32,
    skills: Vec<String>,
    pos: Pos,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"\tRole: {}\nName: {}\nAge: {}\nStaffList: {}\nPos: {}",
            self.label,
            self.name,
            self.age,
            self.staff_list.join(", "),
            self.pos)
    }
}

impl fmt::Display for SoftEng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"\tRole: {}\nName: {}\nAge: {}\nSkills: {}\nPos: {}",
            self.label,
            self.name,
            self.age,
            self.skills.join(", "),
            self.pos)
    }
}

impl fmt::Display for GraphicDesighner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,"\tRole: {}GraphicDesighner\nName: {}\nAge: {}\nSkills: {}\nPos: {}",
            self.label,
            self.name,
            self.age,
            self.skills.join(", "),
            self.pos)
    }
}

fn read_lines(){
    let mut f = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut lines = contents.lines();

    let role = lines.next().unwrap();
    let name = lines.next().unwrap();
    let age = lines.next().unwrap();
    let staff_list = lines.next().unwrap();
    let pos = lines.next().unwrap();
    let pos = pos.trim_start().split(SPACE).collect::<Vec<&str>>();
    let pos = Pos {
        x: pos[1].trim().parse::<f64>().unwrap(),
        y: pos[2].trim().parse::<f64>().unwrap(),
    };

    let mut staff_list = Vec::new();
    let mut skills = Vec::new();

    if role == "Manager" {
        let staff_list_str = lines.next().unwrap();
        staff_list = staff_list_str.split(COMMA).map(|s| s.to_string()).collect();
    } else if role == "SoftEng" {
        let skills_str = lines.next().unwrap();
        skills = skills_str.split(COMMA).map(|s| s.to_string()).collect();
    } else if role == "GraphicDesighner" {
        let skills_str = lines.next().unwrap();
        skills = skills_str.split(COMMA).map(|s| s.to_string()).collect();
    }

    if role == "Manager" {
        let manager = Manager {
            name: name.to_string(),
            age: age.parse().unwrap(),
            staff_list: staff_list,
            pos: pos,
            label: EmployeeType::Manager
        };
        println!("{}", manager);
    } else if role == "SoftEng" {
        let softeng = SoftEng {
            name: name.to_string(),
            age: age.parse().unwrap(),
            skills: skills,
            pos: pos,
            label: EmployeeType::SoftEng
        };
        println!("{}", softeng);
    } else if role == "GraphicDesighner" {
        let graphicdesighner = GraphicDesighner {
            name: name.to_string(),
            age: age.parse().unwrap(),
            skills: skills,
            pos: pos,
            label: EmployeeType::GraphicDesighner
        };
        println!("{}", graphicdesighner);
    }
}

fn main() {
    read_lines();
}