// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.


fn is_prime(n: i64) -> bool{
    //good enough for large numbers (does not work on 4 and not sure if 1 is prime)
    for i in 2..((n as f64).sqrt()+1.0)as i64{
        if n % i == 0{
            return false
        }
    }
    return true;
}

fn sum_prime() -> i64{
    let mut counter = 0;
    let mut iterator = 1;
    let mut sum: i64 = 0;
    while iterator < 2000000{
        iterator += 1;
        if is_prime(iterator){
            counter += 1;
            sum += iterator;
        }
        if counter % 1000 == 0{
            println!("{}: {}", counter, iterator);
        }
    }
    return sum;

}

fn main(){
    println!("{}", sum_prime());
}