//The prime factors of 13195 are 5, 7, 13 and 29.

//What is the largest prime factor of the number 600 851 475 143 ?

fn is_prime(n: i64) -> bool{
    //good enough for large numbers (does not work on 4 and not sure if 1 is prime)
    for i in 2..((n as f64).sqrt()+1.0)as i64{
        if n % i == 0{
            return false
        }
    }
    return true;
}

fn largest_prime(n: i64) -> i64{
    for i in (0..(n)/2 + 1).rev(){
        //faster condition to satisfy
        if n%i == 0{
            if is_prime(i){
                return i;
            }
        }
        if i%100000000 == 0{
            println!("{}", i);
        }
    }
    return -1;
}

//in about an hour this printed the result, 6857, but in the meantime i realised that 
//I could have solved the problem far faster:
//thousands of times faster (and probably more correct)
fn faster_prime(n: i64) -> i64{
    let mut max = 0;
    let mut aux_n = n;
    print!("the prime factors of {} are:\n", n);
    while aux_n != 1{
        let mut i = 2;
        while aux_n % i != 0 || !is_prime(i){
            i += 1;
        }
        if i > max{
            max = i;
        }
        aux_n /= i;
        println!("factor: {}\nn: {}", i, aux_n);
    }
    return max;

}

fn main(){
    let n: i64 = 600851475143;
    println!("{}", faster_prime(n));
}