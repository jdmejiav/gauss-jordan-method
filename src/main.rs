
use std::io::{self,Write};





fn main() {
    let mut temp = String::new();
    println!("Ingresa el número n de variables");
    let _ = io::stdout().flush();

    io::stdin().read_line(&mut temp).ok().expect("error");

    let n:u32 = temp.trim().parse().unwrap();
    temp =String::new();
    let mut _matrix:Vec<Vec<f64>>= Vec::new() ;


    println!("{}",n);

    for _i in 0..n {
        io::stdin().read_line(&mut temp).ok().expect("error");
        let mut _vec_temp:Vec<f64> = Vec::new();
        for j in temp.split(" ").collect::<Vec<&str>>(){
            let _conv:f64 = j.trim().parse().unwrap();
            _vec_temp.push(_conv);
        }
        _matrix.push(_vec_temp);
        temp =String::new();
    }
    println!("");
    for _i in _matrix.iter(){
        for _j in _i.iter(){
            print!("{} ",_j);
        }
        println!();

    }


}
