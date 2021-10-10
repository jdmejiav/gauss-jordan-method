pub struct Fraction {
    numerator: f64,
    denominator: f64,
}

impl Fraction {
    
    pub fn new(numerator: f64, denominator: f64) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn value(&self) -> f64 {
        self.numerator / self.denominator
    }

    pub fn get_numerator(&self) -> f64 {
        self.numerator
    }

    pub fn get_denominator(&self) -> f64 {
        self.denominator
    }
    
    #[warn(dead_code)]
    pub fn _set_numerator(&mut self, numerator: f64) {
        self.numerator = numerator;
    }


    #[warn(dead_code)]
    pub fn _set_denominator(&mut self, denominator: f64) {
        self.numerator = denominator;
    }
    
}

#[warn(dead_code)]
pub fn _sum(f1: &Fraction, f2: &Fraction) -> Fraction {
    let numerator = (f1.get_numerator()*f2.get_denominator())+(f2.get_numerator()*f1.get_denominator());
    let denominator = f1.get_denominator()*f2.get_denominator();
    Fraction::new(numerator, denominator)
}

pub fn dif(f1: &Fraction, f2: &Fraction) -> Fraction {
    let numerator = f1.get_numerator()*f2.get_denominator()-f2.get_numerator()*f1.get_denominator();
    let denominator = f1.get_denominator()*f2.get_denominator();
    Fraction::new(numerator, denominator)
}

pub fn mult(f1: &Fraction, f2: &Fraction) -> Fraction {
    let numerator = f1.get_numerator()*f2.get_numerator();
    let denominator = f1.get_denominator()*f2.get_denominator();
    Fraction::new(numerator, denominator)
}

pub fn div(f1: &Fraction, f2: &Fraction) -> Fraction {
    let numerator = f1.get_numerator()*f2.get_denominator();
    let denominator = f1.get_denominator()*f2.get_numerator();
    Fraction::new(numerator, denominator)
}
