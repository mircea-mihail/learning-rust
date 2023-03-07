fn main(){
    let mut v:Vec<i64> = Vec::new();
    v.push(12);
    v.push(41);
    v.push(0);

    for i in 0..v.len(){
        print!("{} ", v[i]);
    }
    println!();

    let arr = [12, 4, 7, 4, 6, 88];

    for i in arr.iter(){
        print!("{} ", i);
    }

    let sum: i64 = arr.iter().sum();
    println!("\nthe sum of the array is {}", sum);

    println!();

}