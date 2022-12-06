fn main(){
    let mut highest_sum = 0;
    let mut second_sum: i32 = 0;
    let mut third_sum: i32 = 0;
    let mut sum_counter: i32 = 0;
    for line in include_str!("in1.txt").lines() {
        if line != ""{
            sum_counter += line.parse::<i32>().unwrap();
        } else if sum_counter > highest_sum {
            third_sum = second_sum;
            second_sum = highest_sum;
            highest_sum = sum_counter;
            sum_counter = 0;
        } else if sum_counter > second_sum {
            third_sum = second_sum;
            second_sum = sum_counter;
            sum_counter = 0;
        } else if sum_counter > third_sum {
            third_sum = sum_counter;
            sum_counter = 0;
        } else {
            sum_counter = 0;
        }
  }
  println!("{}", highest_sum);
  let top_three_sum = highest_sum + second_sum + third_sum;
  println!("{}", top_three_sum);
}
