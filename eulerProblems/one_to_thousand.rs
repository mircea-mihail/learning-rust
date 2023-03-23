/*
    If the numbers 1 to 5 are written out in words: one, two, three,
four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

    If all the numbers from 1 to 1000 (one thousand) inclusive were
written out in words, how many letters would be used?


    NOTE: Do not count spaces or hyphens. For example, 342 
(three hundred and forty-two) contains 23 letters and 115 (one hundred
and fifteen) contains 20 letters. The use of "and" when writing out
numbers is in compliance with British usage.
 */

static ONE:i32 = 3;
static TWO:i32 = 3;
static THREE:i32 = 5;
static FOUR:i32 = 4;
static FIVE:i32 = 4;
static SIX:i32 = 3;
static SEVEN:i32 = 5;
static EIGHT:i32 = 5;
static NINE:i32 = 4;



fn one_to_nine() -> i64{
    (ONE + TWO + THREE + FOUR + FIVE + SIX + SEVEN + EIGHT + NINE) as i64
}

static TEN:i32 = 3;
static ELEVEN:i32 = 6;
static TWELVE:i32 = 6;
static THIRTEEN:i32 = 8;
static FOURTEEN:i32 = 8;
static FIFTEEN:i32 = 7;
static SIXTEEN:i32 = 7;
static SEVENTEEN:i32 = 9;
static EIGHTEEN:i32 = 8;
static NINETEEN:i32 = 8;


fn ten_to_nineteen() -> i64{
    (TEN+ELEVEN+TWELVE+THIRTEEN+FOURTEEN+FIFTEEN+SIXTEEN+SEVENTEEN+EIGHTEEN+NINETEEN) as i64
}   

static TWENTY:i32= 6;
static THIRTY:i32= 6;
static FORTY:i32= 5;
static FIFTY:i32= 5;
static SIXTY:i32= 5;
static SEVENTY:i32= 7;
static EIGHTY:i32= 6;
static NINETY:i32= 6;


fn twenty_to_99() -> i64{
    let mut sum:i64 = 0;
    sum += one_to_nine() * 8;
    for i in 2..10{
        match i {
            2 => sum += (TWENTY * 10) as i64,
            3 => sum += (THIRTY * 10) as i64,
            4 => sum += (FORTY * 10) as i64,
            5 => sum += (FIFTY * 10) as i64,
            6 => sum += (SIXTY * 10) as i64,
            7 => sum += (SEVENTY * 10) as i64,
            8 => sum += (EIGHTY * 10) as i64,
            9 => sum += (NINETY * 10) as i64,
            _ => sum += 100000000
        }
    }
    return sum;
}

fn one_to_99() -> i64{
    one_to_nine() + ten_to_nineteen() + twenty_to_99()
}

static HUNDRED: i32 = 7;
static AND: i32 = 3;
static ONE_THOUSAND: i32 = 11;

// X       X         X            X
//one   hundred     and     one_to_hundred
//six   hundred     and     one_to_hundred

fn hundred_to_thousand() -> i64{
    let mut sum:i64 = 0;

    sum += (HUNDRED * (999 - 100 + 1)) as i64; 
    sum += (AND * (999 - 100 + 1 - 9)) as i64; // all numbers - one hundred/ six hundred    

    //one hundred.. two (hundred)... etc
    for i in 1..10{
        match i {
            1 => sum += (ONE * 100) as i64,
            2 => sum += (TWO * 100) as i64,
            3 => sum += (THREE * 100) as i64,
            4 => sum += (FOUR * 100) as i64,
            5 => sum += (FIVE * 100) as i64,
            6 => sum += (SIX * 100) as i64,
            7 => sum += (SEVEN * 100) as i64,
            8 => sum += (EIGHT * 100) as i64,
            9 => sum += (NINE * 100) as i64,
            _ => sum += 100000000
        }
    }

    sum += one_to_99() * 9; 

    sum += (ONE_THOUSAND) as i64;

    return sum;
}

fn main(){
    let sum:i64 = one_to_99() + hundred_to_thousand();
    println!("{} ", sum);
}







