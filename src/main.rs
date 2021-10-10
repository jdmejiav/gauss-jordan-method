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
    print_vec_solution(_solution);
}

fn print_matrix(_matrix: &Vec<Vec<Fraction>>) {
    for _i in _matrix.iter() {
        print!("\n|\t");
        for _j in 0.._i.len() {
            if _i.len() - 1 == _j {
                print!(":\t");
            }
            print!("{}/{}\t", _i[_j].get_numerator(), _i[_j].get_denominator());
        }
        println!("|");
    }
}

fn gauss_jordan(mut _matrix: Vec<Vec<Fraction>>) -> Vec<Vec<Fraction>> {
    for _it in 0.._matrix.len() {
        _matrix = find_pibot(_it, _matrix);
        if _matrix[_it][_it].value() == 0 as f64 {
            println!("It is not possible to solve");
        } else {
            for _i in _it.._matrix.len() {
                if _i != _it {
                    let _multiplier = fraction::div(&_matrix[_it][_it], &_matrix[_i][_it]);
                    for _k in 0.._matrix[_i].len() {
                        _matrix[_i][_k] = fraction::dif(
                            &_matrix[_it][_k],
                            &fraction::mult(&_matrix[_i][_k], &_multiplier),
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

fn find_pibot(_idx: usize, mut _matrix: Vec<Vec<Fraction>>) -> Vec<Vec<Fraction>> {
    if _matrix[_idx][_idx].value() == 0 as f64 {
        for _i in _idx.._matrix.len() {
            if _matrix[_i][_idx].value() == 0 as f64 {
                continue;
            } else {
                _matrix.swap(_idx, _i);
                break;
            }
        }
    }
    _matrix
}

fn find_solutions(_matrix: Vec<Vec<Fraction>>) -> Vec<Fraction> {
    let mut temp: Vec<Fraction> = Vec::new();
    for _i in 0.._matrix.len() {
        let mut temp_frac = Fraction::new(0.0, 1.0);
        println!("Inicia it");
        for _j in _matrix.len() - _i - 1.._matrix.len() {
            if _j == _matrix.len() - _i - 1 {
                println!("Esto es lo que suma {}/{}",temp_frac.get_numerator(),temp_frac.get_denominator());
                temp_frac = fraction::sum(&temp_frac, &_matrix[_matrix.len()-_i-1][_j]);
            }else {
                temp_frac = fraction::sum(&temp_frac, &fraction::mult(&temp[_matrix.len()-1-_j],&_matrix[_matrix.len()-_i-1][_j]));
                println!("Esto es lo que suma {}/{}",temp_frac.get_numerator(),temp_frac.get_denominator());
            }
            
        }
        println!("Fin IT");
        println!("Esto es lo que divide num {}/{}",temp_frac.get_numerator(),temp_frac.get_denominator());
        println!("Esto es lo que divide deno {}/{}",&_matrix[_matrix.len()-_i-1][_matrix[_i].len() - 1].get_numerator(),&_matrix[_matrix.len()-_i-1][_matrix[_i].len() - 1].get_denominator());
        let x_i = fraction::div(&_matrix[_matrix.len()-_i-1][_matrix[_i].len() - 1],&temp_frac);
        println!("lo que se guarda {}/{}",x_i.get_numerator(),x_i.get_denominator());
        temp.push(x_i);
    }
    temp
}

fn print_vec_solution(_vec: Vec<Fraction>) {
    let mut out = String::new();
    for _i in 0.._vec.len() {
        out.push_str("x_");
        out.push_str(&_i.to_string());
        out.push_str("=");
        out.push_str(&_vec[_i].value().to_string());
        out.push_str(" ");
    }
    println!("[ {} ]", out);
}
