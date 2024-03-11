fn main() {
    count(10);
    count_down(10);
}

fn count(num: i32){
    for index in 0..=num{
        println!("{}", index);
    }
}
fn count_down(mut num: i32){
    while num > 0 {
        println!("{}", num);
        num -= 1
    }
}
