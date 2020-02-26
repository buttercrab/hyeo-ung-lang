use std::mem::swap;

use crate::big_number::Num;

pub struct RatNum {
    up: Num,
    down: Num,
}

impl RatNum {
    pub fn new(n: isize) -> RatNum {
        RatNum {
            up: Num::new(n),
            down: Num::one(),
        }
    }

    pub fn zero() -> RatNum {
        RatNum {
            up: Num::zero(),
            down: Num::one(),
        }
    }

    pub fn one() -> RatNum {
        RatNum {
            up: Num::one(),
            down: Num::one(),
        }
    }

    pub fn nan() -> RatNum {
        RatNum {
            up: Num::one(),
            down: Num::zero(),
        }
    }

    pub fn is_nan(&self) -> bool {
        self.down.is_zero()
    }

    pub fn to_string(&self) -> String {
        if self.is_nan() {
            format!("너무 커엇...")
        } else {
            format!("{}/{}", self.up.to_string(), self.down.to_string())
        }
    }

    fn optimize(&mut self) {
        let g = Num::gcd(&self.up, &self.down);
        self.up /= &g;
        self.down /= &g;
        if !self.down.is_pos() {
            self.down.minus();
            self.up.minus();
        }
    }

    pub fn minus(&mut self) {
        self.up.minus();
    }

    pub fn flip(&mut self) {
        if !self.is_nan() {
            swap(&mut self.up, &mut self.down);
            if !self.down.is_pos() {
                self.down.minus();
                self.up.minus();
            }
        }
    }

    pub fn add(lhs: &RatNum, rhs: &RatNum) -> RatNum {
        if lhs.is_nan() || rhs.is_nan() {
            return RatNum::nan();
        }

        let mut res = RatNum {
            up: &(&lhs.up * &rhs.down) + &(&lhs.down * &rhs.up),
            down: &lhs.down * &rhs.down,
        };
        res.optimize();
        res
    }

    pub fn mul(lhs: &RatNum, rhs: &RatNum) -> RatNum {
        if lhs.is_nan() || rhs.is_nan() {
            return RatNum::nan();
        }

        let mut res = RatNum {
            up: &lhs.up * &rhs.up,
            down: &lhs.down * &rhs.down,
        };
        res.optimize();
        res
    }
}
