
/// .
fn main() {
    let mut count = 0;
    let mut count2 = 0;
    for line in include_str!("in.txt").lines() {
        count += rps(line);
        count2 += rps2(line);
    }
    println!("{}", count);
    println!("{}", count2);
}

fn rps(s: &str) -> i32{
    let split: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
    let first = &split[0];
    let second = &split[1];
    let mut count = 0;
    if second == "X" {
        count += 1;
        if first == "A"{
            count += 3;
        } else if first == "C" {
            count += 6;
        }
    } else if second == "Y" {
        count += 2;
        if first == "B"{
            count += 3;
        } else if first == "A" {
            count += 6;
        }
    } else if second == "Z" {
        count += 3;
        if first == "C"{
            count += 3;
        } else if first == "B" {
            count += 6;
        }
    }
    return count;
}

fn rps2(s: &str) -> i32{
    let split: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
    let first = &split[0];
    let second = &split[1];
    let mut count = 0;
    if second == "X" {
        if first == "A"{
            count += 3;
        } else if first == "B" {
            count += 1;
        } else if first == "C"{
            count += 2;
        }
    } else if second == "Y" {
        count += 3;
        if first == "A"{
            count += 1;
        } else if first == "B" {
            count += 2;
        } else if first == "C" {
            count += 3;
        }
    } else if second == "Z" {
        count += 6;
        if first == "A"{
            count += 2;
        } else if first == "B" {
            count += 3;
        } else if first == "C" {
            count += 1;
        }
    }
    return count;
}