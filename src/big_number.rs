use std::cmp::{max, min};
use std::ops;
use std::ops::Mul;
use std::panic::resume_unwind;

#[cfg(target_pointer_width = "64")]
pub struct BigNumber {
    pos: bool,
    nan: bool,
    val: Vec<u32>,
}

#[cfg(target_pointer_width = "32")]
struct BigNumber {
    pos: bool,
    nan: bool,
    val: Vec<u16>,
}

#[cfg(target_pointer_width = "64")]
fn new(n: isize) -> BigNumber {
    if n >= 0 {
        BigNumber {
            pos: true,
            nan: false,
            val: vec![n as u32],
        }
    } else {
        BigNumber {
            pos: true,
            nan: false,
            val: vec![(-n) as u32],
        }
    }
}

#[cfg(target_pointer_width = "32")]
fn new(n: isize) -> BigNumber {
    if n >= 0 {
        BigNumber {
            pos: true,
            nan: false,
            val: vec![n as u16],
        }
    } else {
        BigNumber {
            pos: true,
            nan: false,
            val: vec![(-n) as u16],
        }
    }
}

#[cfg(target_pointer_width = "64")]
fn from_vec(v: Vec<u32>) -> BigNumber {
    let mut res = BigNumber {
        pos: true,
        nan: false,
        val: v,
    };
    res.shrink_to_fit();
    res
}

#[cfg(target_pointer_width = "32")]
fn from_vec(v: Vec<u16>) -> BigNumber {
    let mut res = BigNumber {
        pos: true,
        nan: false,
        val: v,
    };
    res.shrink_to_fit();
    res
}

fn zero() -> BigNumber {
    BigNumber {
        pos: true,
        nan: false,
        val: vec![0],
    }
}

fn one() -> BigNumber {
    BigNumber {
        pos: true,
        nan: false,
        val: vec![1],
    }
}

fn nan() -> BigNumber {
    BigNumber {
        pos: true,
        nan: true,
        val: vec![],
    }
}

impl BigNumber {
    fn is_pos(&self) -> bool {
        self.pos
    }

    fn is_nan(&self) -> bool {
        self.nan
    }

    fn to_string(&self) -> String {
        self.to_string_base(10)
    }

    fn to_string_base(&self, base: usize) -> String {
        // TODO: https://en.wikipedia.org/wiki/Double_dabble
        unimplemented!()
    }

    fn shrink_to_fit(&mut self) {
        while match self.val.last() {
            Some(x) => *x == 0,
            None => false,
        } {
            self.val.pop();
        }
    }

    #[cfg(target_pointer_width = "64")]
    fn add_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        let mut v = vec![0; max(lhs.len(), rhs.len()) + 1];

        for i in 0..min(lhs.len(), rhs.len()) {
            let mut t = (lhs[i] as u64) + (rhs[i] as u64) + (v[i] as u64);
            if t > u32::max_value() as u64 {
                v[i + 1] += 1;
                t -= u32::max_value() as u64;
            }
            v[i] = t as u32;
        }

        let t = if lhs.len() < rhs.len() {
            rhs
        } else {
            lhs
        };

        for i in min(lhs.len(), rhs.len())..t.len() {
            v[i] = t[i];
        }

        v
    }

    #[cfg(target_pointer_width = "32")]
    fn add_core(lhs: &Vec<u16>, rhs: &Vec<u16>) -> Vec<u16> {
        let mut v = vec![0; max(lhs.len(), rhs.len()) + 1];

        for i in 0..min(lhs.len(), rhs.len()) {
            let mut t = (lhs[i] as u32) + (rhs[i] as u32) + (v[i] as u32);
            if t > u16::max_value() as u32 {
                v[i + 1] += 1;
                t -= u16::max_value() as u32;
            }
            v[i] = t as u16;
        }

        let t = if lhs.len() < rhs.len() {
            rhs
        } else {
            lhs
        };

        for i in min(lhs.len(), rhs.len())..t.len() {
            v[i] = t[i];
        }

        v
    }

    #[cfg(target_pointer_width = "64")]
    fn sub_core(&self, rhs: &BigNumber) -> BigNumber {
        one()
    }

    #[cfg(target_pointer_width = "32")]
    fn sub_core(&self, rhs: &BigNumber) -> BigNumber {
        one()
    }

    #[cfg(target_pointer_width = "64")]
    fn mult_core(&self, rhs: &BigNumber) -> BigNumber {
        one()
    }

    #[cfg(target_pointer_width = "32")]
    fn mult_core(&self, rhs: &BigNumber) -> BigNumber {
        one()
    }

    fn minus(&mut self) {
        if self.pos {
            self.pos = false;
        } else {
            self.pos = true;
        }
    }
}

impl ops::Add<BigNumber> for BigNumber {
    type Output = BigNumber;

    fn add(self, rhs: BigNumber) -> Self::Output {
        if self.nan || rhs.nan {
            return nan();
        }

        if self.pos {
            if rhs.pos {
                from_vec(BigNumber::add_core(&self.val, &rhs.val))
            } else {
                self.sub_core(&rhs)
            }
        } else {
            let mut res = if rhs.pos {
                self.sub_core(&rhs)
            } else {
                from_vec(BigNumber::add_core(&self.val, &rhs.val))
            };
            res.minus();
            res
        }
    }
}

impl ops::Sub<BigNumber> for BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: BigNumber) -> Self::Output {
        if self.nan || rhs.nan {
            return nan();
        }

        if self.pos {
            if rhs.pos {
                self.sub_core(&rhs)
            } else {
                self.add_core(&rhs)
            }
        } else {
            let mut res = if rhs.pos {
                self.add_core(&rhs)
            } else {
                self.sub_core(&rhs)
            };
            res.minus();
            res
        }
    }
}

impl ops::Mul<BigNumber> for BigNumber {
    type Output = BigNumber;

    fn mul(self, rhs: BigNumber) -> Self::Output {
        if self.nan || rhs.nan {
            return nan();
        }

        let mut res = self.mult_core(&rhs);
        if self.pos ^ rhs.pos {
            res.minus();
        }
        res
    }
}