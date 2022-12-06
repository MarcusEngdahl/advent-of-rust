fn main() {
    let mut string_vec = Vec::new();
    for _i in 0..9 {
        string_vec.push("".to_string());
    }
    for line in include_str!("in.txt").lines().take(8) {
        for i in 0..9 {
            if line.chars().nth(1+i*4).unwrap() != ' ' {
                string_vec[i] = line.chars().nth(1+i*4).unwrap().to_string() + &string_vec[i]; 
            }
        }
    }
    for line in include_str!("in.txt").lines().skip(10){
        let moves :Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        for _i in 0..moves[1].parse().unwrap(){
            let temp_char = string_vec[(moves[3].parse::<i32>().unwrap() -1) as usize].pop().unwrap();
            string_vec[(moves[5].parse::<i32>().unwrap() - 1) as usize].push(temp_char);
        }
    }
    for i in 0..9 {
        print!("{}", string_vec[i].pop().unwrap());
    }
    println!("{}", "");


    let mut string_vec = Vec::new();
    for _i in 0..9 {
        string_vec.push("".to_string());
    }
    for line in include_str!("in.txt").lines().take(8) {
        for i in 0..9 {
            if line.chars().nth(1+i*4).unwrap() != ' ' {
                string_vec[i] = line.chars().nth(1+i*4).unwrap().to_string() + &string_vec[i]; 
            }
        }
    }
    for line in include_str!("in.txt").lines().skip(10){
        let moves :Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        let mut temp_vec = "".to_string();
        for _i in 0..moves[1].parse().unwrap(){
            temp_vec.push(string_vec[(moves[3].parse::<i32>().unwrap() -1) as usize].pop().unwrap());
            
        }
        for letter in temp_vec.chars().rev() {
            string_vec[(moves[5].parse::<i32>().unwrap() - 1) as usize].push(letter);
        }
    }
    for i in 0..9 {
        print!("{}", string_vec[i].pop().unwrap());
    }
    println!("{}", "");
    
}

