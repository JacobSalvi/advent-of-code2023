use std::fs;
use std::env;
use std::ops::Sub;


fn get_char_at_position(lines: &Vec<&str>, i: usize, j:usize) -> Option<char>{
    if  i >= lines.len(){
        return None;
    }
    let line: &str = lines.get(i).unwrap();
    if  j >= line.len(){
        return None;
    }
    let chars: Vec<char> = line.chars().collect();
    return Some(chars[j]);
}



fn get_numbers(lines: &Vec<&str>) -> i32{
    let mut result: i32 = 0;
    for i in 0..lines.len(){
        let mut touches_symbol = false;
        let mut current: String = "".to_string();
        let line: &str = lines.get(i).unwrap();
        let chars: Vec<char> = line.chars().collect();
        for j in 0..chars.len(){
            let char: char = chars[j];
            if char.is_digit(10){
                for r in -1..=1{
                    for c in -1..=1{
                        if r == 0 && c == 0{
                            //continue;
                        }
                        let new_i = i as i32 + r;
                        let new_j = j as i32 + c;
                        let opt_char: Option<char> = get_char_at_position(&lines, new_i as usize, new_j as usize);
                        if ! opt_char.is_none(){
                            let new_char: char = opt_char.unwrap();
                            if !new_char.is_digit(10) && new_char !='.'{
                                touches_symbol = true;
                            }
                        }
                    }
                }
                current += &char.to_string();
            }else{
                if touches_symbol{
                    result += current.parse::<i32>().unwrap();
                }
                current = "".to_string();
                touches_symbol = false;
            }
        }
        if touches_symbol{
            result += current.parse::<i32>().unwrap();
        }
    }
    return result;
}

fn get_number_starting_at_position(line: &str, c: usize) -> i32{
    let mut current: String = "".to_string();
    let chars: Vec<char> = line.chars().collect();
    current += &chars[c].to_string();
    let mut i = c-1;
    while i >=0{
        let ch  = chars.get(i).unwrap_or(&'.');
        if !ch.is_digit(10){
            break;
        }
        current = ch.to_string()+&current;
        if i == 0{
            break;
        }
        i=i.sub(1);
    }
    i = c+1;
    while i <chars.len(){
        let ch  = chars.get(i).unwrap_or(&'.');
        if !ch.is_digit(10){
            break;
        }
        current = current+&ch.to_string();
        i += 1;
    }
    return current.parse::<i32>().unwrap();
}


fn get_touching_number(lines: &Vec<&str>, r: usize, c: usize) -> Vec<i32>{
    let char_left: Option<char> = get_char_at_position(lines, r, c-1); 
    let char_center: Option<char> = get_char_at_position(lines, r, c); 
    let char_right: Option<char> = get_char_at_position(lines, r, c+1);
    
    let cl: char = char_left.unwrap_or('.');
    let cr: char = char_right.unwrap_or('.');
    let cc: char = char_center.unwrap_or('.');
    let mut res: Vec<i32> = Vec::new();
    if !cc.is_digit(10){
        if cl.is_digit(10){
            res.push(get_number_starting_at_position(lines[r], c-1));
        }
        if cr.is_digit(10){
            res.push(get_number_starting_at_position(lines[r], c+1));
        }
    }else{
        res.push(get_number_starting_at_position(lines[r], c));
    }
    return res;
}

fn get_nearby_numbers(lines: &Vec<&str>, line_num: usize, char_num: usize) -> Vec<i32>{
    let mut nearby_numbers: Vec<i32> = Vec::new();
    for r in -1..=1{
        let new_l: i32 = line_num as i32 +r;
        nearby_numbers.extend(get_touching_number(lines, new_l as usize , char_num));
    }
    return nearby_numbers;
}

fn get_gear_ratio(lines: &Vec<&str>) -> i32{
    let mut result: i32 = 0;
    for i  in 0..lines.len(){
        let line: &str = lines.get(i).unwrap();
        let chars: Vec<char> = line.chars().collect();
        for j in 0..chars.len(){
            let char: char = chars[j];
            if char == '*'{
                // get nearby numbers
                let nearby_numbers: Vec<i32> = get_nearby_numbers(&lines, i, j);
                if nearby_numbers.len() == 2{
                    result += nearby_numbers[0]*nearby_numbers[1];
                }
            }
        }
    }
    return result;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let content = fs::read_to_string(&args[1]).expect("");
    let lines:Vec<&str> = content.split("\n").filter(|e| !e.is_empty()).collect();
    let res = get_numbers(&lines);
    let ratio = get_gear_ratio(&lines);

    println!("{}", res);
    println!("{}", ratio);
    println!("Hello, world!");
}
