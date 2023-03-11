// The following iterative sequence is defined for the set of positive integers:

// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

fn collatz(a: i64) -> i64{
    if a % 2 == 0{
        a / 2
    }
    else{
        a * 3 + 1
    }
}

fn longest_chain(){
    let mut max = 0;
    let mut max_counter = 0;
    for i in 1..1000000{
        let mut n: i64 = i;
        let mut counter = 0;
        while n != 1{
            n = collatz(n);
            counter += 1;
        }
        if counter > max_counter{
            max_counter = counter;
            max = i;
        }
    }
    println!("the longest string belongs to {}", max);
}

fn main(){
    longest_chain();
}