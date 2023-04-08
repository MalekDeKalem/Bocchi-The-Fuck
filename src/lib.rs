use std::io;
use wasm_bindgen::prelude::*;


const DATASIZE: u16 = 65535;


// Words to use :
/*
bocchi  =    ぼち       : u307c | u3061         // For incrementation
gotoh   =    ごと       : u3054 | u3068         // For decrementation
wakaru  =    わかる     : u308f | u304b | u308b // For output
ikuyo   =    いくよ     : u3044 | u304f | u3088 // For input
kita    =    きた       : u304d | u305f         // Open Bracket
gita    =    ぎた       : u304e | u305f         // Close Bracket
*/

// Parses input string from a bocchifuck programm to a brainfuck programm and returns it
#[wasm_bindgen]
pub fn bocchifuck(programm: &str) -> String {
    let mut output: String = String::new();
    let mut index = 0;
    let trim: String = programm.chars().filter(|c| !c.is_whitespace()).collect();

    while index < trim.len() {
        if trim.chars().nth(index) != Option::None {

            match trim.chars().nth(index).unwrap() {
                'ぼ' => {
                    if trim.chars().nth(index + 1).unwrap() == 'ち' {
                        output.push('+');
                        index += 1;
                    }
                },
                'ご' => {
                    if trim.chars().nth(index+ 1).unwrap() == 'と' {
                        output.push('-');
                        index += 1;
                    }
                },
                'わ' => {
                    if trim.chars().nth(index + 1).unwrap() == 'か' && trim.chars().nth(index + 2).unwrap() == 'る' {
                        output.push('.');
                        index += 2;
                    }
                },
                'い' => {
                    if trim.chars().nth(index + 1).unwrap() == 'く' && trim.chars().nth(index + 2).unwrap() == 'よ' {
                        output.push(',');
                        index += 2;
                    }
                },
                'き' => {
                    if trim.chars().nth(index + 1).unwrap() == 'た' {
                        output.push('[');
                        index += 1;
                    }
                },
                'ぎ' => {
                    if trim.chars().nth(index + 1).unwrap() == 'た' {
                        output.push(']');
                        index += 1;
                    }
                },
                _ => println!("Something went wrong"),
            };

        }

        index += 1;
    }
    output = brainfuck(&output);
    output
}

fn brainfuck(programm: &str) -> String {
    let mut data_stream: [u8; DATASIZE as usize] = [0; DATASIZE as usize];
    let mut data_pointer: u16 = 0;
    let mut output: String = String::new();
    let mut line = String::new();

    let mut count = 0;
    let mut index = 0;

    while index < programm.len() {

        if programm.chars().nth(index) != Option::None {
            match programm.chars().nth(index).unwrap() {
                '>' => {
                    if data_pointer == DATASIZE - 1 {
                        data_pointer = 0;
                    } else {
                        data_pointer += 1;
                    }
                },
                '<' => {
                    if data_pointer == 0 {
                        data_pointer = DATASIZE - 1;
                    } else {
                        data_pointer -= 1;
                    }
                },
                '+' => {
                    if data_stream[data_pointer as usize] == 255 {
                        data_stream[data_pointer as usize] = 0;
                    } else {
                        data_stream[data_pointer as usize] += 1;
                    }
                },
                '-' => {
                    if data_stream[data_pointer as usize] == 0 {
                        data_stream[data_pointer as usize] = 255;
                    } else {
                        data_stream[data_pointer as usize] -= 1;
                    }
                },
                '.' => {
                    output.push(data_stream[data_pointer as usize] as char);
                },
                ',' => {
                    let input = io::stdin().read_line(&mut line).expect("Failed to read");
                    data_stream[data_pointer as usize] = input as u8;
                },
                '[' => {
                    if data_stream[data_pointer as usize] == 0 {
                        count += 1;
                        while programm.chars().nth(index as usize).unwrap() != ']' || count != 0 {
                            index += 1;
                            if programm.chars().nth(index).unwrap() == '[' {
                                count += 1;
                            } else if programm.chars().nth(index).unwrap() == ']' {
                                count -= 1;
                            }
                        }
                    }
                },
                ']' => {
                    if data_stream[data_pointer as usize] != 0 {
                        count += 1;
                        while programm.chars().nth(index).unwrap() != '[' || count != 0 {
                            index -= 1;
                            if programm.chars().nth(index).unwrap() == ']' {
                                count += 1;
                            } else if programm.chars().nth(index).unwrap() == '[' {
                                count -= 1;
                            }
                        }
                    }
                },
                _ => println!("Invalid symbol at char number {} uwu", index),
            };
        }
        index += 1;
    }
    output
}


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}




#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}