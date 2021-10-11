mod fraction;

use fraction::Fraction;
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
    let mut _matrix: Vec<Vec<Fraction>> = Vec::new();

    for _i in 0..n {
        let mut _vec_temp: Vec<Fraction> = Vec::new();
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
            let frac_temp = Fraction::new(_conv, 1 as f64);
            _vec_temp.push(frac_temp);
        }
        _matrix.push(_vec_temp);
        temp = String::new();
    }
    print_matrix(&_matrix);
    println!("");
    _matrix = gauss_jordan(_matrix);
    let _solution = find_solutions(_matrix);
    print_vec_solution(&_solution);
}

fn print_matrix(_matrix: &Vec<Vec<Fraction>>) {
    for i in _matrix.iter() {
        print!("\n|\t");
        for j in 0..i.len() {
            if i.len() - 1 == j {
                print!(":\t");
            }
            print!("{}/{}\t", i[j].get_numerator(), i[j].get_denominator());
        }
        println!("|");
    }
}

fn gauss_jordan(mut _matrix: Vec<Vec<Fraction>>) -> Vec<Vec<Fraction>> {
    for it in 0.._matrix.len() {
        _matrix = find_pibot(it, _matrix);
        if _matrix[it][it].value() == 0 as f64 {
            println!("It is not possible to solve");
        } else {
            for i in 0.._matrix.len() {
                if i != it {
                    let _multiplier = fraction::div(&_matrix[it][it], &_matrix[i][it]);
                    for _k in 0.._matrix[i].len() {
                        _matrix[i][_k] = fraction::dif(
                            &_matrix[it][_k],
                            &fraction::mult(&_matrix[i][_k], &_multiplier),
                        );
                    }
                }
                print_matrix(&_matrix);
                println!("\n--------------------------------------------");
            }
        }
    }
    print_matrix(&_matrix);
    _matrix
}

fn find_pibot(idx: usize, mut _matrix: Vec<Vec<Fraction>>) -> Vec<Vec<Fraction>> {
    if _matrix[idx][idx].value() == 0 as f64 {
        for i in idx.._matrix.len() {
            if _matrix[i][idx].value() == 0 as f64 {
                continue;
            } else {
                _matrix.swap(idx, i);
                break;
            }
        }
    }
    _matrix
}

fn find_solutions(_matrix: Vec<Vec<Fraction>>) -> Vec<Fraction> {
    let mut temp: Vec<Fraction> = Vec::new();
    for i in 0.._matrix.len() {
        let x_i = fraction::div(&_matrix[i][_matrix[i].len() - 1], &_matrix[i][i]);
        temp.push(x_i);
    }
    temp
}

fn print_vec_solution(_vec: &Vec<Fraction>) {
    let mut out = String::new();
    for i in 0.._vec.len() {
        out.push_str("x_");
        out.push_str(&i.to_string());
        out.push_str(" = ");
        out.push_str(&_vec[i].value().to_string());
        out.push_str("   ");
    }

    println!("[ {} ]", out);
}
