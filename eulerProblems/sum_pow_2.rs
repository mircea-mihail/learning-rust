// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 2^1000?

//vector to store individual digits


//
fn double(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32>{
    let mut v = Vec::new();
    let mut carry = 0;
    for i in (0..v1.len()).rev(){
        carry += v1[i] + v2[i];
        v.insert(0, carry % 10);
        carry -= carry%10;
        carry /= 10;
    }
    while carry != 0 {
        v.insert(0, carry%10);
        carry -= carry%10;
        carry /= 10;
    }

    //values are stored in reverse in v, so it needs to be printed in reverse
    //only the first 10 digits are required

    return v;
}

fn get_sum(v: &Vec<i32>) -> i32{
    let mut sum = 0;
    for i in 0..v.len(){
        sum += v[i];
    }
    return sum;
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    for _i in 0..1000{
        v = double(&v, &v);
    }

    println!("{}", get_sum(&v));
}
