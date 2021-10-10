# gauss-jordan-method

Equation solver using Gauss Jordan method for nxn variables built on Rust

### Execution

#### Using cargo:
    $ cargo run

#### Using Rustup:

go to src/ and

    $ rustc main.rs


#### Input:

<ul>
    <li>
        First line must be the n of the number of variables
    </li>
    <li>
        Subsequently enter the coefficients of the equations reparated by one space " ". <br>
        Ex: x0 x1 x2 ... xn K
    </li>
</ul
    
#### Example:

If we want to solve the equation

    4x + 6y + 7z = 3
    5x + 3y + 2z = 2
    1x + 2y + 4z = 9
    
The input should be:
    
    3
    4 6 7 3
    5 3 2 2
    1 2 4 9
 
    
    
    




