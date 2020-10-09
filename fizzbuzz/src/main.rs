fn main() {

    for i in 1..101{
    let mut word = String::new();
        if i % 3 ==0 {
            word = format!("{}", "Fizz");
        }
        if i % 5 == 0 {
            word = format!("{}{}", word, "Buzz");
        }
        if word == "" {
            word = format!("{}", i);
        }

        println!("{}",word);

    }

}

