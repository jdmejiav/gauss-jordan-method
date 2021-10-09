use std::io::{self, Write};

fn main() {
    let mut temp = String::new();
    print!("Enter the number n of variables: ");
    let mut n: usize;
    loop {
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut temp).ok().expect("error");
        n = temp.trim().parse().unwrap();
        if n > 1 {
            break;
        } else {
            println!("n must be higher than 1");
        }
    }
    println!("Enter one by one {} equations", n);
    temp = String::new();
    let mut _matrix: Vec<Vec<f64>> = Vec::new();

    for _i in 0..n {
        let mut _vec_temp: Vec<f64> = Vec::new();
        loop {
            io::stdin().read_line(&mut temp).ok().expect("error");
            if temp.split(" ").collect::<Vec<&str>>().len() > n {
                break;
            } else {
                println!("Is not a valid equation");
                println!("Hint: x0 x1 x2 ... xn K");
                temp = String::new();
            }
        }
        for j in temp.split(" ").collect::<Vec<&str>>() {
            let _conv: f64 = j.trim().parse().unwrap();
            _vec_temp.push(_conv);
        }
        _matrix.push(_vec_temp);
        temp = String::new();
    }
    print_matrix(&_matrix);
    println!("");
    _matrix = gauss_jordan(_matrix);
    let _solution = find_solutions(_matrix);
    print_vec_solution(_solution);
}

fn print_matrix(_matrix: &Vec<Vec<f64>>) {
    for _i in _matrix.iter() {
        print!("\n|\t");
        for _j in 0.._i.len() {
            if _i.len() - 1 == _j {
                print!(":\t");
            }
            print!("{}\t", _i[_j]);
        }
        println!("|");
    }
}

fn gauss_jordan(mut _matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    for _it in 0.._matrix.len() {
        _matrix = find_pibot(_it, _matrix);
        if _matrix[_it][_it] == 0 as f64 {
            println!("It is not possible to solve");
        } else {
            for _i in 0.._matrix.len() {
                if _i != _it {
                    let _multiplier = _matrix[_it][_it] / _matrix[_i][_it];
                    for _k in 0.._matrix[_i].len() {
                        _matrix[_i][_k] = _matrix[_it][_k] - _matrix[_i][_k]*_multiplier;
                    }
                }
            }
        }
    }
    print_matrix(&_matrix);
    _matrix
}

fn find_pibot(_idx: usize, mut _matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    if _matrix[_idx][_idx] == 0 as f64 {
        for _i in _idx.._matrix.len() {
            if _matrix[_i][_idx] == 0 as f64 {
                continue;
            } else {
                _matrix.swap(_idx, _i);
                break;
            }
        }
    }
    _matrix
}

fn find_solutions (_matrix: Vec<Vec<f64>>) ->Vec<f64>{
    let mut temp: Vec<f64> = Vec::new();

    for _i in 0.._matrix.len(){
        println!("{}",_matrix[_i][_matrix.len()-1]);
        let x_i = _matrix[_i][_i] / _matrix[_i][_matrix[_i].len()-1];
        temp.push(x_i);
    }
    for _i in temp.iter(){
        print!("{} ",_i);
    }
    println!("");
    temp
}


fn print_vec_solution (_vec: Vec<f64>){
    let mut out = String::new();
    for _i in 0.._vec.len(){
        out.push_str("x_");
        out.push_str(&_i.to_string());
        out.push_str("=");
        out.push_str(&_vec[_i].to_string());
        out.push_str(", ");
    }
    println!("[ {} ]",out);
}