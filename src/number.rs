use std::mem::swap;
use std::ops;

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

    pub fn neg(v: &RatNum) -> RatNum {
        RatNum {
            up: (-&v.up).clone(),
            down: (&v.down).clone(),
        }
    }

    pub fn set_copy(&mut self, rhs: &RatNum) {
        self.up.set_copy(&rhs.up);
        self.down.set_copy(&rhs.down);
    }

    pub fn set_move(&mut self, rhs: RatNum) {
        self.up.set_move(rhs.up);
        self.down.set_move(rhs.down);
    }
}

impl ops::Add<&RatNum> for &RatNum {
    type Output = RatNum;

    fn add(self, rhs: &RatNum) -> Self::Output {
        RatNum::add(self, rhs)
    }
}

impl ops::AddAssign<&RatNum> for RatNum {
    fn add_assign(&mut self, rhs: &RatNum) {
        self.set_move(&*self + rhs);
    }
}

impl ops::Mul<&RatNum> for &RatNum {
    type Output = RatNum;

    fn mul(self, rhs: &RatNum) -> Self::Output {
        RatNum::mul(self, rhs)
    }
}

impl ops::MulAssign<&RatNum> for RatNum {
    fn mul_assign(&mut self, rhs: &RatNum) {
        self.set_move(&*self * rhs);
    }
}

impl ops::Neg for &RatNum {
    type Output = RatNum;

    fn neg(self) -> Self::Output {
        RatNum::neg(self)
    }
}