use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
enum Language {Thai, Hiragana, Katakana, English, Chinese, Burmese, Korean, LatinExtended, None} 

#[derive(Debug, Clone)]
struct LangStruct {
    lang: Language,
    range: (u64, u64),
    name: String,
    count: u64,
}

fn given_unicode(c: char, table: &Vec<LangStruct>) -> Language{
    let mut lang:Language = Language::None;
    for i in table{
        if u64::from(c as u32) >= i.range.0 && u64::from(c as u32) <= i.range.1{
            lang = i.lang.clone();
            break;
        }
    }
    return lang;
}

fn given_lang(table: &Vec<LangStruct>, lang: Language) -> String{
    let mut string: String = "".to_string();
    for i in table{
        if lang == i.lang{
            string = i.name.clone();
            break
        }
    }
    return string;
}

fn read_file(filename: &str, mut int_table: Vec<u64>) -> Vec<u64> {
    // let mut content = String::new();
    let mut file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for c in line.expect("can't read").chars(){
            if int_table.contains(&u64::from(c as u32)) {
                continue;
            }
            else{
                // let new_uni = c as u32;
                // println!("{:x}", new_uni);

                // let mut new_uni = c.escape_unicode().to_string();
                // new_uni = "0x".to_owned() + &(&new_uni[3..new_uni.len()-1]).to_string();
                // println!("{}", new_uni);
                
                let mut new_uni = c as u32;
                int_table.push(new_uni as u64);
            }
        }
    }
    return int_table;

    // file.read_to_string(&mut content).expect("Unable to read string");
    // return content;
}

fn produce_report(filename: &str, int_tab: Vec<u64>, table: &mut Vec<LangStruct>) {
    let mut file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut line_no = 1;
    
    for line in reader.lines() {
        // let mut count: u64 = 0;
        for j in 0..=N{
            table[j].count = 0;
        }
        let mut unknown = 0;
        println!("Line {}: ", line_no);
        
        for c in line.expect("can't read").chars(){
            for i in 0..=N{
                if u64::from(c as u32) >= table[i].range.0 && u64::from(c as u32) <= table[i].range.1 {
                    table[i].count += 1;
                    // print!("{:?} ", c);
                    break;
                }
                else {
                    continue;
                }
            }
            // if int_tab.contains(&u64::from(c as u32)) {
            //     unknown += 1;
            //     continue;
            // }
        }

        for i in 0..=N{
            if table[i].count != 0{
            println!(">> {} ({}) \n", table[i].name, table[i].count);
            }
        }
        // if unknown != 0{
        //     println!(">> Unknown ({}) \n", unknown);
        // }
        line_no += 1;
    }
}

const N: usize = 7;
fn main() {
    let mut table: Vec<LangStruct> = vec![
        LangStruct{lang:Language:: Thai, range: (0x0E00, 0x0E7F), name: "Thai".to_string(), count: 0},
        LangStruct{lang:Language:: Hiragana, range: (0x3040, 0x309F), name: "Hiragana".to_string(), count: 0},
        LangStruct{lang:Language:: Katakana, range: (0x30A0, 0x30FF), name: "Katakana".to_string(), count: 0},
        LangStruct{lang:Language:: English, range: (0x0041, 0x007A), name: "English".to_string(), count: 0},
        LangStruct{lang:Language:: Chinese, range: (0x4E00, 0x9FFF), name: "Chinese".to_string(), count: 0},
        LangStruct{lang:Language:: Burmese, range: (0x1000, 0x109F), name: "Burmese".to_string(), count: 0},
        LangStruct{lang:Language:: Korean, range: (0xAC00, 0xD7A3), name: "Korean".to_string(), count: 0},
        LangStruct{lang:Language:: LatinExtended, range: (0x00A0, 0x00FF), name: "Latin Extended".to_string(), count: 0},
    ];

    for i in 0..=N{
    println!("{:?}", table[i]);
    }

    // let mut int_table: Vec<(u64,u64)> = vec![
    //     (0x0000, 0x007F), // ASCII
    //     (0x0E00, 0x0E7F) // THAI
    // ];

    let mut int_table: Vec<u64> = vec![];
    for i in 0x0000..=0x007F{ // ASCII
        int_table.push(i);
    }
    for i in 0x0E00..=0x0E7F{ // THAI
        int_table.push(i);
    }

    println!("{:?}", given_lang(&table, given_unicode('운', &table)));
    println!("{:?}", given_lang(&table, given_unicode('é', &table)));
    println!("{:?}", given_lang(&table, given_unicode('၍', &table)));
    println!("{:?}", given_lang(&table, given_unicode('ก', &table)));
    println!("{:?}", given_lang(&table, given_unicode('p', &table)));
    println!("{:?}", given_lang(&table, given_unicode('的', &table)));

    let int_table = read_file("input.txt", int_table.clone());
    println!("{:?}", int_table);

    produce_report("input.txt", int_table, &mut table);
    
}