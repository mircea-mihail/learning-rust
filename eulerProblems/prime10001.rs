// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

fn is_prime(n: i64) -> bool{
    //good enough for large numbers (does not work on 4 and not sure if 1 is prime)
    for i in 2..((n as f64).sqrt()+1.0)as i64{
        if n % i == 0{
            return false
        }
    }
    return true;
}

fn last_prime() -> i64{
    let mut counter = 0;
    let mut iterator = 1;
    while counter != 10001{
        iterator += 1;
        if is_prime(iterator){
            counter += 1;
            //println!("{}: {}", counter, iterator);
        }
    }
    return iterator;

}

fn main(){
    println!("{}", last_prime());
}