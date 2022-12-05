fn main() {
    let mut total = Vec::new();
    for line in include_str!("in.txt").lines() {
        let length = line.chars().count();
        let words = line.split_at(length/2);
        
        let mut counter = 0;
        for character in words.0.chars() {
            if words.1.contains(&character.to_string()){
                counter += convert(character);
                break 
            }
        }
        total.push(counter);
    }
    let mut calc = 0;
    for i in total {
        calc += i;
    }
    println!("{}", calc);

    let ruck: Vec<String> = include_str!("in.txt").lines().map(String::from).collect(); 
    total = Vec::new();
    for i in 0..(ruck.len()/3) {
        for character in ruck[3*i].chars() {
            if ruck[3*i+1].contains(&character.to_string()) && ruck[3*i+2].contains(&character.to_string()) {
                println!("{}", i);
                total.push(convert(character));
                break 
            }
        }
    }
    calc = 0;
    for i in total {
        calc += i;
    }
    println!("{}", calc);
}

fn convert(c: char) -> u32 {
    return (c as u32 - 38) % 58;
}
