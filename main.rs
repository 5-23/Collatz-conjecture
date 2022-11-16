/*

콜라스 추측


*/

fn main() {
    let mut b:bool = true;
    let mut i:u128 = 0;
    while b {
        if how(i) == 1{
            println!("{}| true", i);
            i += 1;
        }else{
            println!("{}| FALSE!!!!", i);
            b = false;
        }
    }

}

fn how(i:u128) -> u128{
    if i == 1         { 1 }
    else if i%2 == 0  { how(i/2) }
    else              { how(i*3+1) }
}
