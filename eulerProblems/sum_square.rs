// The sum of the squares of the first ten natural numbers is, 1^2 + ... + 10^2 = 385

// The square of the sum of the first ten natural numbers is, (1 + 2 + .. + 10)^2 = 3025 

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is .

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn square_sum() -> i64{ 
    let mut sum = 0;
    for i in 1..100 + 1{
        sum += i;
    }
    return sum * sum;
}

fn sum_square() -> i64{
    let mut sum = 0;
    for i in 1..100 + 1{
        sum += i * i;
    }
    return sum;
}

fn main(){
    println!("|{} - {}| = {}", sum_square(), square_sum(), (sum_square() - square_sum()).abs());
}