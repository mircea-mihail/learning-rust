//not re-sizeable!
//to pas an array into a function you need its slice (similar to c pointers, but they know their size)
// this is a slice of i32
fn sum(v: &mut [i32]) -> i32{
    let mut sum = 0;
    for i in 0..v.len(){
        sum += v[i];
    }
    //in order to add this line, the array, the function arguments and the function call had to be made mutable 
    v[0] = 100;
    return sum;
}

fn printv(v: &[i32]){
    for i in 0..v.len(){
        print!("{} ", v[i]);
    }
    print!("\n");
}

fn main(){
    // arrays are not used so much in rust -> they're fixed in size.
    let mut v = [1, 4, 2, 6];
    printv(&v);
    let slice = &v[1..];
    printv(&slice);
    //the compiler paniks at run time when using slice[slice.len() + 1]
    print!("the first slice element is {:?}\nand the one after the last is {:?}\n", slice[0], slice.get(slice.len() + 1));
    //some and none: none unwrapped is an error message
    println!("pozitie valida: is some {}, is none {}", slice.get(0).is_some(), slice.get(0).is_none());
    println!("pozitie invalida: is some {}, is none {}", slice.get(slice.len() + 1).is_some(), slice.get(slice.len() + 1).is_none());
    
    println!();
    println!();
    let maybe_last = slice.get(100);
    let last = if maybe_last.is_some() {*maybe_last.unwrap()} else {-1};
    println!("last: {}", last);

    //key thing: this is not passing the address of..
    //this is *borrowing* a slice of the vector
    let res = sum(&mut v);
    println!("\nthe sum of v is {}", res);
    println!("just checking, v is {:?}", v);//debug purposes
}