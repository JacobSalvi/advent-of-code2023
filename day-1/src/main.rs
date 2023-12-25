use std::env;
use std::fs;

fn remove_non_digit(lines: Vec<String>)-> Vec<String>{
    let mut cleaned_lines: Vec<String> = vec![];
    for l in lines{
        cleaned_lines.push(l.chars().filter(|a| a.is_digit(10)).collect::<String>());
    }
    return cleaned_lines;
}

fn replace_stringified_numbers(sn: &Vec<&str>, cl: Vec<&str>)-> Vec<String>{
    let mut new_lines: Vec<String> = vec![];
    for el in cl{
        let mut new_el = el.to_string();
        for n in sn.into_iter(){
            if(el.contains(n)){
                let index = sn.iter().position(|r| r == n).unwrap()+1;
                let rep = format!("{}{}{}",n.chars().nth(0).unwrap(), index.to_string(), n.chars().nth(n.len()-1).unwrap());
                new_el = new_el.replace(n, &rep)
            }
        }
        new_lines.push(new_el.to_string());
    }
    return new_lines;
}


fn map_to_int(cl: Vec<String>)-> Vec<i32>{
    let mut numbers: Vec<i32> = vec![];
    for n in cl{
        numbers.push(n.parse::<i32>().unwrap());
    }
    return numbers;
}

fn remove_empty_string(cl: Vec<String>)->Vec<String>{
    return cl.into_iter().filter(|el| !el.is_empty()).collect::<Vec<String>>();
}

fn take_first_and_last_char(cl: Vec<String>)->Vec<String>{
    return cl.iter().map(|el| el.chars().nth(0).unwrap().to_string()+&el.chars().last().unwrap().to_string()).collect();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let content = fs::read_to_string(&args[1]).expect("");
    let lines: Vec<&str> = content.split("\n").collect();
    // maybe an hashmap would be better
    let stringified_numbers: Vec<&str> = vec!["one", "two", "three", "four", "five", "six","seven", "eight","nine"];
    let mut cl = replace_stringified_numbers(&stringified_numbers, lines);
    for a in cl.iter(){
        println!("{}", a);
    }
    cl = remove_non_digit(cl);
    cl = remove_empty_string(cl);
    cl = take_first_and_last_char(cl);
    let numbers = map_to_int(cl);

    let total = numbers.iter().fold(0, |mut acc, el| acc+el );
    println!("{}", total);
}
