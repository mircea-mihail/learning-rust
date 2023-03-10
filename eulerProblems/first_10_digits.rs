// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
// (found in input.txt)
// file1.rs

use std::fs::File;
use std::io::Read;

fn input_to_mat(mat: &mut Vec<Vec<i32>>){
    // if you want to provide an input via command line
    // let first = std::env::args().nth(1).expect("provide a file to read from");
    

    let mut file: File = File::open(&"input.txt").expect("could not open file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't print to text");
    
    let mut k: i32 = 0;
    let mut row = 0;
    mat.push(Vec::new());

    for i in 0..text.len(){
        let mut digit = text.chars().nth(i).unwrap() as i32;
        if digit != '\n' as i32{
            if k == 50{
                k = 0;
                row += 1;
                mat.push(Vec::new());
                // println!("row: {}", row);
            }
                        
            digit -= '0' as i32;
            mat[row].push(digit);

            k += 1;
        }
    }
}



fn print_mat(mat: &Vec<Vec<i32>>){
    println!("lines: {}, cols: {}", mat.len(), mat[0].len());

    for i in 0..mat.len(){
        for j in 0..mat[0].len(){
            print!("{}", mat[i][j]);
        }
        println!();
    }
}

fn get_sum(mat: & Vec<Vec<i32>>){
    let mut v: Vec<i32> = Vec::new();
    let mut carry = 0;
    for j in (0..mat[0].len()).rev(){
        for i in 0..mat.len(){
            carry += mat[i][j];
        }
        v.push(carry % 10);
        carry -= carry%10;
        carry /= 10;
    }
    while carry != 0 {
        v.push(carry%10);
        carry -= carry%10;
        carry /= 10;
    }

    //values are stored in reverse in v, so it needs to be printed in reverse
    //only the first 10 digits are required
    for i in (v.len()-10..v.len()).rev(){
        print!("{}", v[i]);
    }
    println!();
}

fn main() {
    let mut mat: Vec<Vec<i32>> = Vec::new();
    // slicing is better than cloning (no need to allocate more memory)
    // input_to_mat_auxv(&mut mat);
    input_to_mat(&mut mat);
    //needs a slice
    print_mat(&mat);

    get_sum(&mat);
}
