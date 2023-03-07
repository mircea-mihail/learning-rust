// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn smallest_multiple(m: i64, n:i64) -> i64{
    let mut m1 = m;
    let mut n1 = n;
    while m1 != n1 {
        if m1 > n1 {
            n1 += n;
        }
        else{ 
            m1 += m;
        }
    }
    return m1;
}

fn smallest_number_div() -> i64{
    let mut smallest = 1;
    for i in 1..20 + 1{
        smallest = smallest_multiple(i, smallest);
    } 
    return smallest;
}

fn main(){
    
    println!("{}", smallest_number_div());
}
