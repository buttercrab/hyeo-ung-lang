use std::{fmt, ops};
use std::cmp::{max, min, Ordering};

pub struct Num {
    pos: bool,
    val: Vec<u32>,
}

impl Num {
    pub fn new(n: isize) -> Num {
        if n >= 0 {
            Num {
                pos: true,
                val: vec![n as u32],
            }
        } else {
            Num {
                pos: true,
                val: vec![(-n) as u32],
            }
        }
    }

    pub fn from_vec(v: Vec<u32>) -> Num {
        let mut res = Num {
            pos: true,
            val: v,
        };
        res.shrink_to_fit();
        res
    }

    pub fn zero() -> Num {
        Num {
            pos: true,
            val: vec![0],
        }
    }

    pub fn one() -> Num {
        Num {
            pos: true,
            val: vec![1],
        }
    }

    pub fn clone(&self) -> Num {
        Num {
            pos: self.pos,
            val: self.val.clone(),
        }
    }

    pub fn is_pos(&self) -> bool {
        self.pos
    }

    pub fn is_zero(&self) -> bool {
        self.val == vec![0]
    }

    pub fn from_string(s: String) -> Num {
        Num::from_string_base(s, 10)
    }

    // assert: _base <= 36
    pub fn from_string_base(s: String, _base: usize) -> Num {
        let base = Num::new(_base as isize);
        let mut res = Num::new(0);
        let mut i = 0usize;
        let arr = s.chars().collect::<Vec<char>>();
        if s.starts_with('-') {
            res.pos = false;
            i = 1;
        }

        while i < arr.len() {
            let c = arr[i];
            let k = if '0' <= c && c <= '9' {
                c as isize - '0' as isize
            } else if 'A' <= c && c <= 'Z' {
                c as isize - 'A' as isize + 10
            } else {
                0
            };
            res *= &base;
            res += &Num::new(k);
            i += 1;
        }

        res
    }

    pub fn to_string(&self) -> String {
        self.to_string_base(10)
    }

    // TODO: https://en.wikipedia.org/wiki/Double_dabble
    // assert: _base <= 36
    pub fn to_string_base(&self, _base: usize) -> String {
        let base = Num::new(_base as isize);
        let mut res = String::new();
        let mut num = self.clone();
        while !num.is_zero() {
            let k = &num % &base;
            num /= &base;

            res.push(if k.val[0] < 10 {
                ('0' as u8 + k.val[0] as u8) as char
            } else {
                ('a' as u8 + k.val[0] as u8 - 10) as char
            });
        }
        if !self.pos {
            res.push('-')
        }
        res.chars().rev().collect()
    }

    fn shrink_to_fit(&mut self) {
        while match self.val.last() {
            Some(x) => *x == 0,
            None => false,
        } {
            self.val.pop();
        }
    }

    fn add_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        let mut v = vec![0; max(lhs.len(), rhs.len()) + 1];

        for i in 0..min(lhs.len(), rhs.len()) {
            let mut t = (lhs[i] as u64) + (rhs[i] as u64) + (v[i] as u64);
            if t >= 1u64 << 32 {
                v[i + 1] = 1;
                t -= 1u64 << 32;
            }
            v[i] = t as u32;
        }

        let t = if lhs.len() < rhs.len() {
            rhs
        } else {
            lhs
        };

        for i in min(lhs.len(), rhs.len())..t.len() {
            let mut s = (t[i] as u64) + (v[i] as u64);
            if s >= 1u64 << 32 {
                v[i + 1] = 1;
                s -= 1u64 << 32;
            }
            v[i] = s as u32;
        }

        v.shrink_to_fit();
        v
    }

    fn sub_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> (Vec<u32>, bool) {
        let mut v = vec![0; max(lhs.len(), rhs.len()) + 1];

        let (b, a, swapped) = if Num::less_core(lhs, rhs) {
            (lhs, rhs, true)
        } else {
            (rhs, lhs, false)
        };

        for i in 0..a.len() {
            let mut t = if i < b.len() {
                (a[i] as i64) - (b[i] as i64) - (v[i] as i64)
            } else {
                (a[i] as i64) - (v[i] as i64)
            };
            while t < 0 {
                v[i + 1] += 1;
                t += 1i64 << 32;
            }
            v[i] = t as u32;
        }

        v.shrink_to_fit();
        (v, swapped)
    }

    // TODO: https://en.wikipedia.org/wiki/Sch%C3%B6nhage%E2%80%93Strassen_algorithm
    fn mult_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        let mut v = vec![0; lhs.len() + rhs.len() + 1];

        for i in 0..(lhs.len() + rhs.len()) {
            for j in max(0, i - rhs.len())..min(i, lhs.len()) {
                let t = (lhs[j] as u64) * (rhs[j] as u64);
                v[i] += t % (1u64 << 32);
                v[i + 1] += v[i] / (1u64 << 32);
                v[i] %= (1u64 << 32);
                v[i + 1] += t / (1u64 << 32);
                v[i + 2] += v[i + 1] / (1u64 << 32);
                v[i + 1] %= (1u64 << 32);
            }
        }

        let mut res: Vec<u32> = vec![0; v.len()];
        for i in 0..v.len() {
            res[i] = v[i] as u32;
        }

        res.shrink_to_fit();
        res
    }

    // TODO: https://en.wikipedia.org/wiki/Division_algorithm
    fn div_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        unimplemented!()
    }

    fn less_core<T: PartialOrd>(lhs: &Vec<T>, rhs: &Vec<T>) -> bool {
        if lhs.len() != rhs.len() {
            lhs.len() < rhs.len()
        } else {
            for i in (0..lhs.len()).rev() {
                if lhs[i] != rhs[i] {
                    return lhs[i] < rhs[i];
                }
            }
            false
        }
    }

    pub fn minus(&mut self) {
        self.pos = !self.pos;
    }

    pub fn add(lhs: &Num, rhs: &Num) -> Num {
        let mut need_flip = false;
        let mut res = Num::from_vec(if lhs.pos {
            if rhs.pos {
                Num::add_core(&lhs.val, &rhs.val)
            } else {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            }
        } else {
            let t = if rhs.pos {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            } else {
                Num::add_core(&lhs.val, &rhs.val)
            };
            need_flip ^= true;
            t
        });
        if need_flip {
            res.minus();
        }
        res.shrink_to_fit();
        res
    }

    pub fn sub(lhs: &Num, rhs: &Num) -> Num {
        let mut need_flip = false;
        let mut res = Num::from_vec(if lhs.pos {
            if !rhs.pos {
                Num::add_core(&lhs.val, &rhs.val)
            } else {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            }
        } else {
            let t = if !rhs.pos {
                let (tmp, swapped) = Num::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            } else {
                Num::add_core(&lhs.val, &rhs.val)
            };
            need_flip ^= true;
            t
        });
        if need_flip {
            res.minus();
        }
        res.shrink_to_fit();
        res
    }

    pub fn mul(lhs: &Num, rhs: &Num) -> Num {
        let mut res = Num::from_vec(Num::mult_core(&lhs.val, &rhs.val));
        if lhs.pos ^ rhs.pos {
            res.minus();
        }
        res.shrink_to_fit();
        res
    }

    pub fn div(lhs: &Num, rhs: &Num) -> Num {
        let mut res = Num::from_vec(Num::div_core(&lhs.val, &rhs.val));
        if !lhs.pos {
            res = Num::add(&res, &Num::one());
        }
        if !rhs.pos {
            res.minus()
        }
        res.shrink_to_fit();
        res
    }

    pub fn rem(lhs: &Num, rhs: &Num) -> Num {
        let q = Num::div(&lhs, &rhs);
        Num::sub(&lhs, &Num::mul(&q, &rhs))
    }

    // TODO: https://en.wikipedia.org/wiki/Lehmer%27s_GCD_algorithm
    pub fn gcd(lhs: &Num, rhs: &Num) -> Num {
        let mut a = lhs.clone();
        let mut b = rhs.clone();
        while !b.is_zero() {
            let d = &a % &b;
            let c = b;
            a = c;
            b = d;
        }
        a
    }

    pub fn neg(v: &Num) -> Num {
        Num {
            pos: !v.pos,
            val: v.val.clone(),
        }
    }

    pub fn set_copy(&mut self, rhs: &Num) {
        self.val = rhs.val.clone();
        self.pos = rhs.pos;
    }

    pub fn set_move(&mut self, rhs: Num) {
        self.val = rhs.val;
        self.pos = rhs.pos;
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            true
        } else {
            self.pos == other.pos && self.val == other.val
        }
    }
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Option::Some(Ordering::Less)
        } else {
            if self > other {
                Option::Some(Ordering::Greater)
            } else {
                Option::Some(Ordering::Equal)
            }
        }
    }

    fn lt(&self, other: &Self) -> bool {
        if self.pos {
            if other.pos {
                Num::less_core(&self.val, &other.val)
            } else {
                true
            }
        } else {
            if other.pos {
                false
            } else {
                !Num::less_core(&other.val, &self.val)
            }
        }
    }

    fn le(&self, other: &Self) -> bool {
        !(other < self)
    }

    fn gt(&self, other: &Self) -> bool {
        other < self
    }

    fn ge(&self, other: &Self) -> bool {
        !(self < other)
    }
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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

impl ops::Sub<&Num> for &Num {
    type Output = Num;

    fn sub(self, rhs: &Num) -> Self::Output {
        Num::sub(self, rhs)
    }
}

impl ops::SubAssign<&Num> for Num {
    fn sub_assign(&mut self, rhs: &Num) {
        self.set_move(&*self - rhs);
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

impl ops::Div<&Num> for &Num {
    type Output = Num;

    fn div(self, rhs: &Num) -> Self::Output {
        Num::div(self, rhs)
    }
}

impl ops::DivAssign<&Num> for Num {
    fn div_assign(&mut self, rhs: &Num) {
        self.set_move(&*self / rhs);
    }
}

impl ops::Rem<&Num> for &Num {
    type Output = Num;

    fn rem(self, rhs: &Num) -> Self::Output {
        Num::rem(self, rhs)
    }
}

impl ops::RemAssign<&Num> for Num {
    fn rem_assign(&mut self, rhs: &Num) {
        self.set_move(&*self % rhs);
    }
}

impl ops::Neg for &Num {
    type Output = Num;

    fn neg(self) -> Self::Output {
        Num::neg(self)
    }
}
