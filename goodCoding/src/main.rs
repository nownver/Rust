

// const UC_zero : String = "0x40".to_string();
// const A : String = "10".to_string();
// const B : String = "11".to_string();
// const C : String = "12".to_string();
// const D : String = "13".to_string();
// const E : String = "14".to_string();
// const F : String = "15".to_string();

const UC_eng[char;16] = ['0', '1', ... ,'A','F'];
const UC_jap[char;16] = ['0', '1', ... ,'あ'];

// if character == UC_jap[i]:
//     character == UC_eng[i]

// 07あ2え
fn main() {
    let hexadecimal = "07あ2え".to_string();
    let mut binary = "".to_string();

    for character in hexadecimal.clone().into_bytes(){
        if character == UC_jap[i]{
            character == UC_eng[i]
        }
        binary += &format!("0{:b}", character);
    }

    println!("{} into binary is {}", hexadecimal, binary);

}

