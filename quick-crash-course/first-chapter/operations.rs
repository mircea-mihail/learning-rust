fn sqr(i: f64) -> f64{
    i*i
} 

fn max(a: f64, b: f64) -> f64{
    if a > b{
        a
    }
    else{
        b
    }
}

fn factorial(a: i32) -> i32{
    if a == 1{
        a
    }
    else{
        a * factorial(a - 1)
    }
}

fn increment(a: &mut i32){
    *a = *a + 1;
}

fn main(){
    let mut sum = 0.0;
    for i in 0..5{
        sum += sqr(i as f64);
    }
    println!("the sum is {}", sum);
    let a = 5.6;
    let b = 7 as f64;
    let mut c : i32 = 5;
    println!("between {} and {}, {} is bigger", a, b, max(a, b));
    println!("{} factorial is {}", c, factorial(c));
    print!("{} incremented is", c); 
    increment(&mut c);
    println!(" {}", c);
}