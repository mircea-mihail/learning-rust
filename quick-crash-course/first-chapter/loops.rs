const RANGE:i32 = 6;

fn c_style_even_odd (){
    for i in 0..RANGE{
        if i%2 == 0{
            println!("even,\t{}", i);
        }
        else{
            println!("odd,\t{}", i);
        }
    }
}

fn odd_even_odd(){
    println!();
    for i in 0..RANGE{
        let even_odd = if i%2 == 0 {"even"} else {"odd"};
        println!("{},\t{}", even_odd, i);
    }
}

fn main(){
    c_style_even_odd();
    odd_even_odd();
}