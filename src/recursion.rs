fn countdown(i: u32) {
    println!("{i:#?}");
    if i <= 1 {
        return;
    }
    countdown(i - 1);
}

fn greet(name: &str) {
    println!("hello, {name}!");
    greet2(name);
    println!("getting ready to say bye");
    bye();
}

fn greet2(name: &str) {
    println!("how are you {name}?");
}

fn bye() {
    println!("ok bye!");
}

fn factorial(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    return n * factorial(n - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(0, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(6, factorial(3));
        assert_eq!(2432902008176640000, factorial(20));
    }
}
