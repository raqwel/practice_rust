use std::io;

fn main() {
    let input_num = get_num();

    if input_num < 0 {
        println!("please input number than 0");
        return;
    }

    let answer = fib(input_num);

    println!("answer is {}", answer);
}

fn get_num() -> u32 {
    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num)
        .expect("Failed to read line");
    let input_num: u32 = input_num.trim().parse()
        .expect("please type a number");
    input_num
}

fn fib(num: u32) -> u32 {
    if num <= 1 {
        num
    } else {
        fib(num - 2) + fib(num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(1), 1)
    }
}