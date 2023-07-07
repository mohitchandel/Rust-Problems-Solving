fn is_prime(numb: u128) -> bool {
    for i in 2..numb {
        if numb % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let numbers: [u128; 5] = [12, 7, 17, 9, 3];

    for number in numbers {
        if is_prime(number) {
            println!("{} is a prime number", number);
        } else {
            println!("{} is not a prime number", number)
        }
    }
}
