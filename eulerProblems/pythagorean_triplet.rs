fn prod() -> i64{
    for i in 1..1000{
        for j in 1..1000{
            if i as f64 + j as f64 + (((i*i + j*j) as f64).sqrt()) == 1000.0{
                return i * j * (((i*i + j*j) as f64).sqrt() as i64);
            }
        }
    }
    return 0;
}

fn main(){
    println!("the product is {}", prod());
}