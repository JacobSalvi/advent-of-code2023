use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Extraction {
    blue: i32,
    red: i32,
    green: i32
}

fn get_extraction(input: &str) -> Extraction{
    let elements: Vec<&str> = input.split(",").collect();
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;
    for el in elements{
        let a: Vec<&str> = el.trim().split(" ").collect();
        let val = a[0].parse().unwrap();
        if(el.contains("blue")){
            blue = val;
        }else if (el.contains("red")) {
            red = val;
        }else if(el.contains("green")){
            green = val;
        }
    }
    return Extraction{blue, red, green};
}

fn get_extractions(line: &str) -> Vec<Extraction>{
    let mut extractions: Vec<Extraction> = vec![];
    let encoded_ex: Vec<&str> = line.split(";").collect();
    for ex in encoded_ex{
        let ext: Extraction = get_extraction(ex);
        extractions.push(ext);
    }
    return extractions;
}


fn get_games(lines: Vec<&str>) -> HashMap<i32, Vec<Extraction>>{
    let mut game_id_to_extractions = HashMap::new();
   for l in lines{
        let a: Vec<&str> = l.split(":").collect();
        println!("{}", l);
        let game_id: i32 = a[0].replace("Game ", "").parse().unwrap();
        let exs: Vec<Extraction> = get_extractions(a[1]);
        game_id_to_extractions.insert(game_id, exs);
    }
    return game_id_to_extractions;
}

fn is_game_possible(exs: &Vec<Extraction>)-> bool{
    const n_red: i32 = 12;
    const n_green: i32 = 13;
    const n_blue: i32 = 14;
    for ex in exs{
        if ex.blue > n_blue || ex.red > n_red || ex.green > n_green{
            return false;
        }
    }
    return true;
}

fn find_minimum_possible_cubes(exs: Vec<Extraction>)->Extraction{
    let mut min_b: i32 = 0;
    let mut min_g: i32 = 0;
    let mut min_r: i32 = 0;
    for ex in exs{
        if ex.blue > min_b{
            min_b = ex.blue;
        }

        if ex.red > min_r{
            min_r = ex.red;
        }

        if ex.green > min_g{
            min_g = ex.green;
        }
    }
    return Extraction{blue: min_b, red: min_r, green: min_g};
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let content = fs::read_to_string(&args[1]).expect("");
    let lines:Vec<&str> = content.split("\n").filter(|e| !e.is_empty()).collect();
    let games = get_games(lines);
    let mut res = 0;
    let mut cube_sum = 0;
    for (key, val) in games.into_iter(){
        if is_game_possible(&val){
            res += key;
        }
        let min_ex: Extraction = find_minimum_possible_cubes(val);
        cube_sum += min_ex.red*min_ex.green*min_ex.blue; 

    }
    println!("{}", res);
    println!("{}", cube_sum);
    println!("Hello, world!");
}
