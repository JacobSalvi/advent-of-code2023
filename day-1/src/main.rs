use std::env;
use std::fs;

fn remove_non_digit(lines: Vec<&str>)-> Vec<String>{
    let mut cleaned_lines: Vec<String> = vec![];
    for l in lines{
        cleaned_lines.push(l.chars().filter(|a| a.is_digit(10)).collect::<String>());
    }
    return cleaned_lines;
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
    let mut cl = remove_non_digit(lines);
    cl = remove_empty_string(cl);
    cl = take_first_and_last_char(cl);
    let numbers = map_to_int(cl);

    let total = numbers.iter().fold(0, |mut acc, el| acc+el );
    println!("{}", total);
}
