fn fizzbuzz(i: u32) -> String {
    let divisible = (i % 3, i % 5);
    match divisible {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => format!("{}", i),
    }
}

fn main() {
    for i in 1..100 {
        println!(" {}: {}", i, fizzbuzz(i));
    }
}
