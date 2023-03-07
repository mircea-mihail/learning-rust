// A palindromic number reads the same both ways.
//The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

//Find the largest palindrome made from the product of two 3-digit numbers.

fn printv(v: & Vec<i32>){
    for i in 0..v.len(){
        print!("{} ", v[i]);
    }
    println!();
}

fn is_palindrome(n: i32) -> bool{
    let mut v = Vec::new();
    let mut aux_n = n;
    while aux_n/10 != 0{
        v.insert(0, aux_n % 10);
        aux_n /= 10;
    }
    v.insert(0, aux_n);

    let mut begin = 0;
    let mut end = v.len() - 1;

    while begin < end{
        if v[begin] != v[end]{
            return false;
        }
        begin += 1;
        end -= 1;
    }

    return true;
} 

fn is_3_digit_product(n: i32) -> bool{
    for i in (100..999).rev(){
        if n%i == 0 && n/i < 999 && n/i > 100 {
            print!("{} * {} = {}\n", i, n/i, n);
            return true;
        }
    }
    return false;
}

fn largest_palindrome() -> i32{
    for i in (0..999*999).rev(){
        if is_palindrome(i) && is_3_digit_product(i){
            return i;
        }
    }
    return -1;
}

fn main(){
    println!("the largest palindrome is {}", largest_palindrome());
}