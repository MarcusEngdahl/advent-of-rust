fn main() {
    let mut counter = 0;
    for line in include_str!("in.txt").lines() { 
        let mut char_vec :Vec<String> = Vec::new();
        let mut temp_c = String::new();
        for c in line.chars() {
            if c == ',' || c == '-' {
                char_vec.push(temp_c);
                temp_c = "".to_string();
            } else {
                temp_c.push(c);
            }
        }
        char_vec.push(temp_c);
        if char_vec[0].parse::<i32>().unwrap() <= char_vec[2].parse::<i32>().unwrap()
            && char_vec[1].parse::<i32>().unwrap() >= char_vec[3].parse::<i32>().unwrap(){
                counter += 1;
        } else if char_vec[0].parse::<i32>().unwrap() >= char_vec[2].parse::<i32>().unwrap()
            && char_vec[1].parse::<i32>().unwrap() <= char_vec[3].parse::<i32>().unwrap(){
                counter += 1;
        } 
    }    
    println!("{}", counter);

    counter = 0;
    for line in include_str!("in.txt").lines() { 
        let mut char_vec :Vec<String> = Vec::new();
        let mut temp_c = String::new();
        for c in line.chars() {
            if c == ',' || c == '-' {
                char_vec.push(temp_c);
                temp_c = "".to_string();
            } else {
                temp_c.push(c);
            }
        }
        char_vec.push(temp_c);
        if char_vec[2].parse::<i32>().unwrap() <= char_vec[1].parse::<i32>().unwrap()
            && char_vec[3].parse::<i32>().unwrap() >= char_vec[0].parse::<i32>().unwrap(){
                counter += 1;
        }
    }
    println!("{}", counter);
}
