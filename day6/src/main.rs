use std::collections::HashSet;

fn main() {
    let code = include_str!("in.txt");
    let mut reverse_code = code.chars().rev().collect::<String>();
    let mut four_char = "".to_string();
    for _i in 0..4 {
        let temp_char = reverse_code.pop();
        four_char.push(temp_char.unwrap());
    }
    
    for char in code.chars() {
        let mut chars = HashSet::new();
        for i in 0..4{
            chars.insert(four_char.chars().nth(i));
        }
        if chars.len() == 4 {
            println!("{}", four_char);
            break
        }
        let mut three_char = four_char.chars().skip(1).collect::<String>();
        three_char.push(char);
        four_char = three_char;
    }

    println!("{}", code.find(&four_char).unwrap() +4);




    let code = include_str!("in.txt");
    let mut reverse_code = code.chars().rev().collect::<String>();
    let mut four_char = "".to_string();
    for _i in 0..14 {
        let temp_char = reverse_code.pop();
        four_char.push(temp_char.unwrap());
    }
    
    for char in code.chars() {
        let mut chars = HashSet::new();
        for i in 0..14{
            chars.insert(four_char.chars().nth(i));
        }
        if chars.len() == 14 {
            println!("{}", four_char);
            break
        }
        let mut three_char = four_char.chars().skip(1).collect::<String>();
        three_char.push(char);
        four_char = three_char;
    }

    println!("{}", code.find(&four_char).unwrap() + 14);



    
    

}
