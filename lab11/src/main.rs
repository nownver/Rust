
use std::io;
// const country:String
#[derive(Debug)]
enum TypePhone {
    Local,
    International,
    LikelyScammer,
    Special,
}

#[derive(Debug)]
struct PhoneNumber {
    phonetype: TypePhone,
    prefix: String,
    number: String,
    label: String,
}

fn analyse_number(mut num: String) -> PhoneNumber{
    num = num.trim().to_string();
    let mut struct_num = PhoneNumber{phonetype:TypePhone::Local, prefix: "66".to_string(), number: "0".to_string(), label:"".to_string()};
    let mut len_num:usize = num.to_string().len();

    if len_num ==
    let mut index = 0;
    for i in num[1..].chars(){
        if !i.is_numeric(){
            struct_num.phonetype = TypePhone::Special;
            num.drain(index..);
            break;
        }
        index += 1;
    }

    if len_num == 12 || len_num == 11{
        struct_num.prefix = num[1..3].to_string();
        if struct_num.prefix == "66"{
            struct_num.phonetype = TypePhone::Local;
            struct_num.number = num[3..].to_string();
        }
        else{
            struct_num.phonetype = TypePhone::International;
            struct_num.number = num[3..].to_string();
        
    }
    if num.chars().nth(0).unwrap() == '0' && len_num == 10 {
        struct_num.phonetype = TypePhone::Local;
        struct_num.number = num[0..].to_string();
    }
    if num.chars().nth(0).unwrap() != '0' && len_num == 9 {
        struct_num.phonetype = TypePhone::Local;
        struct_num.number = num[0..].to_string();
    }
    }
    if num[1..4].to_string() == "657".to_string() && len_num == 13{
        struct_num.phonetype = TypePhone::LikelyScammer;
        struct_num.number = num[4..].to_string();
        struct_num.prefix = num[1..4].to_string();
    }

    struct_num
}
struct TelNo { dummy: u64, }

fn print_telNo( tn: TelNo ) { 
  print!("Tel number {}", tn.dummy );
  println!(" is valid");
}

fn read_text_line() -> String {
  eprint!("Enter a telephone number ");
  let mut buffer = String::new();
  let result = io::stdin().read_line(&mut buffer);
  eprintln!("Buffer read ({}) [{}]", buffer.len(), buffer );
  buffer
}

// Some other useful functions, eg
// fn find_next_number( s: String ) -> u64 { .. }

fn proc_text_line( s: &String ) -> TelNo {
  let next_string: String = String::new();
  let mut nc:u64 = 0;
  let tn = TelNo{ dummy : 99 };
  // Ignore all irrelevant digits!!!
  for ch in s.chars() {
    // Do somehting useful here
    nc += 1;
  }
  eprintln!("Char processed {}", nc );
  tn
}

fn main() {
  println!("Telephone numbers");
  let mut cnt: u64 = 0;
 
  // Check each phone number
  loop {
    let mut text = read_text_line();
    eprintln!("Text [{}]", text );
    if text.len() < 3 { break; }
    else {
      let nc = proc_text_line( &text );
      cnt += 1;
    }
    let mut check = analyse_number(text);
    println!("{:?}", check);
  }
  eprintln!("\n{} tel nos processed", cnt );
  // Check credit card numbers
  // ...
 
}
