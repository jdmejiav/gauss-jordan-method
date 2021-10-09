pub struct Fraction {
    numerator: f64,
    denominator: f64,
}

impl Fraction {
    fn new(numerator: f64, denominator: f64) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }
}
