fn fibonacci(num: u32) -> u32 {
    if num < 2 {
        num
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::fibonacci;

    #[test]
    fn fibo_test() {
        let array = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        let mut index: u32 = 0;
        for num in array {
            assert_eq!(fibonacci(index), num);
            index += 1;
        }
    }
}
