/*

콜라스 추측


*/

fn main() {
    let mut b:bool = true;
    let mut i:u128 = 0;
    while b {
        if how(i, 0){
            println!("{}| true", i);
            i += 1;
        }else{
            println!("{}| FALSE!!!!", i);
            b = false;
        }
    }

}

fn how(i:u128, runed:i32) -> bool{
    if runed >= 6290 { false }

    else if i == 1   { true }
    
    else if i%2 == 0 { how(i/2, runed + 1) }
    else             { how(i*3+1, runed + 1) }
}

