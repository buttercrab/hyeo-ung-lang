use std::mem::swap;
use std::ops;

use crate::big_number::BigNum;

pub struct Num {
    up: BigNum,
    down: BigNum,
}

impl Num {
    pub fn new(n: isize) -> Num {
        Num {
            up: BigNum::new(n),
            down: BigNum::one(),
        }
    }

    pub fn zero() -> Num {
        Num {
            up: BigNum::zero(),
            down: BigNum::one(),
        }
    }

    pub fn one() -> Num {
        Num {
            up: BigNum::one(),
            down: BigNum::one(),
        }
    }

    pub fn nan() -> Num {
        Num {
            up: BigNum::one(),
            down: BigNum::zero(),
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
        let g = BigNum::gcd(&self.up, &self.down);
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

    pub fn add(lhs: &Num, rhs: &Num) -> Num {
        if lhs.is_nan() || rhs.is_nan() {
            return Num::nan();
        }

        let mut res = Num {
            up: &(&lhs.up * &rhs.down) + &(&lhs.down * &rhs.up),
            down: &lhs.down * &rhs.down,
        };
        res.optimize();
        res
    }

    pub fn mul(lhs: &Num, rhs: &Num) -> Num {
        if lhs.is_nan() || rhs.is_nan() {
            return Num::nan();
        }

        let mut res = Num {
            up: &lhs.up * &rhs.up,
            down: &lhs.down * &rhs.down,
        };
        res.optimize();
        res
    }

    pub fn neg(v: &Num) -> Num {
        Num {
            up: (-&v.up).clone(),
            down: (&v.down).clone(),
        }
    }

    pub fn set_copy(&mut self, rhs: &Num) {
        self.up.set_copy(&rhs.up);
        self.down.set_copy(&rhs.down);
    }

    pub fn set_move(&mut self, rhs: Num) {
        self.up.set_move(rhs.up);
        self.down.set_move(rhs.down);
    }
}

impl ops::Add<&Num> for &Num {
    type Output = Num;

    fn add(self, rhs: &Num) -> Self::Output {
        Num::add(self, rhs)
    }
}

impl ops::AddAssign<&Num> for Num {
    fn add_assign(&mut self, rhs: &Num) {
        self.set_move(&*self + rhs);
    }
}

impl ops::Mul<&Num> for &Num {
    type Output = Num;

    fn mul(self, rhs: &Num) -> Self::Output {
        Num::mul(self, rhs)
    }
}

impl ops::MulAssign<&Num> for Num {
    fn mul_assign(&mut self, rhs: &Num) {
        self.set_move(&*self * rhs);
    }
}

impl ops::Neg for &Num {
    type Output = Num;

    fn neg(self) -> Self::Output {
        Num::neg(self)
    }
}