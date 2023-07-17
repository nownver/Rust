use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::io::{BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    Thai,
    Arabic,
    Tagalog,
    Chinese,
    Japanese,
    Burmese,
    Korea,
    English,
    Latin,
    Lao,
    ASCII,
    Other,
}

#[derive(Debug, PartialEq)]
struct EeachLang {
    lang: Language,
    range: Vec<u32>,
    name: String,
}

#[derive(Debug, PartialEq)]
struct Count {
    lang: Language,
    count: u64,
}

fn main() {
    let mut lang_vec = vec![
        EeachLang {
            lang: Language::Thai,
            range: vec![0x0E00, 0x0E7F],
            name: "Thai".to_string(),
        },
        EeachLang {
            lang: Language::Arabic,
            range: vec![0x0600, 0x06FF],
            name: "Arabic".to_string(),
        },
        EeachLang {
            lang: Language::Tagalog,
            range: vec![0x1700, 0x171F],
            name: "Tagalog".to_string(),
        },
        EeachLang {
            lang: Language::Chinese,
            range: vec![0x4E00, 0x9FFF],
            name: "Chinese".to_string(),
        },
        EeachLang {
            lang: Language::Japanese,
            range: vec![0x3040, 0x309F],
            name: "Japanese".to_string(),
        },
        EeachLang {
            lang: Language::Burmese,
            range: vec![0x1000, 0x109F],
            name: "Burmese".to_string(),
        },
        EeachLang {
            lang: Language::Korea,
            range: vec![0x3131, 0xD7A3],
            name: "Korea".to_string(),
        },
        EeachLang {
            lang: Language::English,
            range: vec![0x0041, 0x007A],
            name: "English".to_string(),
        },
        EeachLang {
            lang: Language::Latin,
            range: vec![0x20, 0xFF],
            name: "Latin".to_string(),
        }, 
        EeachLang {
            lang: Language::Lao,
            range: vec![0x0E80, 0x0EFF],
            name: "Lao".to_string(),
        },     
        EeachLang {
            lang: Language::ASCII,
            range: vec![0x00, 0x7F],
            name: "ASCII".to_string(),
        },   
        EeachLang {
            lang: Language::Other,
            range: vec![0x00, 0x10FFFF],
            name: "Other".to_string(),
        },
    ];

    let mut count_vec = vec![
        Count {
            lang: Language::Thai,
            count: 0,
        },
        Count {
            lang: Language::Arabic,
            count: 0,
        },
        Count {
            lang: Language::Tagalog,
            count: 0,
        },
        Count {
            lang: Language::Chinese,
            count: 0,
        },
        Count {
            lang: Language::Japanese,
            count: 0,
        },
        Count {
            lang: Language::Burmese,
            count: 0,
        },
        Count {
            lang: Language::Korea,
            count: 0,
        },
        Count {
            lang: Language::English,
            count: 0,
        },
        Count {
            lang: Language::Latin,
            count: 0,
        },
        Count {
            lang: Language::ASCII,
            count: 0,
        },
        Count {
            lang: Language::Other,
            count: 0,
        },
    ];
    count_lines();
    count_lang(&mut count_vec, &mut lang_vec);
    println!("");
}


fn count_lang(vec_count: &mut Vec<Count>, vec_lang: &mut Vec<EeachLang>){
    if let Ok(lines) = read_lines("input.txt") {
        for (line_number, line) in lines.enumerate() {
            if let Ok(ref ip) = line {
                let mut text = ip;
                for i in text.chars().into_iter() {
                    let mut a: Language = Language::Thai;
                    let mut c = i;
                    a = find_lang(c, &vec_lang);

                    for k in 0..vec_lang.len() {
                        if a == vec_lang[k].lang {
                            for j in 0..vec_count.len() - 1 {
                                if a == vec_count[j].lang {
                                    vec_count[j].count += 1;
                                }
                            }
                        }
                    }
                }
                print_report_line(line_number.try_into().unwrap(),vec_count);
                println!();
            }
        }
    }
}


fn print_report_line(line: u64, vec_struct:&mut Vec<Count>){
    print!("Line {:?} : ", line + 1);
    for i in 0..vec_struct.len() - 1 {
        if vec_struct[i].count > 0 {
            print!("{:?} ({:?}) ", vec_struct[i].lang, vec_struct[i].count);
            // println!();
        }
    }
    for i in 0..vec_struct.len() - 1 {
        vec_struct[i].count = 0;
        }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn find_lang(c: char, table: &Vec<EeachLang>) -> Language {
    let mut char_lang: Language = Language::Thai;
    for i in table {
        if i.range.len() == 2 {
            if c as u32 >= i.range[0] && c as u32 <= i.range[1] {
                char_lang = i.lang;
                break;
            }
        } else {
            for j in 0..i.range.len() {
                if c as u32 == i.range[j] {
                    //println!("{}", i.range[j]);
                    char_lang = i.lang;
                    break;
                }
            }
        }
    }
    char_lang
}

// no need
fn tell_lang(each_lang: &Language, table: &Vec<EeachLang>) ->String{
    let mut name_lang = "".to_string();
    for i in table{
        if each_lang == &i.lang{
            name_lang = i.name.clone();
            break
        }
    }
    name_lang
}

fn count_lines(){
    let file = BufReader::new(File::open("input.txt").expect("Unable to open file"));
    let mut cnt  = 0;
    for _ in file.lines() {
        cnt = cnt + 1;
    }
    println!("");
    println!("Total lines are: {}",cnt);
    println!("");
}
    