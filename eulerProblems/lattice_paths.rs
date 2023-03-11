// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.


// How many such routes are there through a 20×20 grid?

fn print_mat(mat: & Vec<Vec<i64>>){
    for i in 0..20{
        for j in 0..20{
            print!("{} ", mat[i][j]);
        }
        println!();
    }
}

fn make_routes(mat: &mut Vec<Vec<i64>>){
    for _i in 0..20{
        mat.push(Vec::new());  
    }
    for i in 0..20{
        mat[0].push(i+2);
        if i != 0{
            mat[i as usize].push(i+2);
        }
    }
    
    for i in 1..20{
        for j in 1..20{
            let sum = mat[i-1][j] + mat[i][j-1];
            mat[i].push(sum); 
        }
    }
}

fn main(){
    let mut mat: Vec<Vec<i64>> = Vec::new();

    make_routes(&mut mat);  

    print_mat(&mat);

    println!("\n\nthe value you seek: {}", mat[19][19]);
}