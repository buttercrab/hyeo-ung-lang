use std::{fmt, ops};
use std::cmp::{max, min, Ordering};

pub struct BigNum {
    pos: bool,
    val: Vec<u32>,
}

impl BigNum {
    pub fn new(n: isize) -> BigNum {
        if n >= 0 {
            BigNum {
                pos: true,
                val: vec![n as u32],
            }
        } else {
            BigNum {
                pos: false,
                val: vec![(-n) as u32],
            }
        }
    }

    pub fn from_vec(v: Vec<u32>) -> BigNum {
        let mut res = BigNum {
            pos: true,
            val: v,
        };
        res.shrink_to_fit();
        res
    }

    pub fn zero() -> BigNum {
        BigNum {
            pos: true,
            val: vec![0],
        }
    }

    pub fn one() -> BigNum {
        BigNum {
            pos: true,
            val: vec![1],
        }
    }

    pub fn clone(&self) -> BigNum {
        BigNum {
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

    pub fn from_string(s: String) -> BigNum {
        BigNum::from_string_base(s, 10)
    }

    // assert: _base <= 36
    pub fn from_string_base(s: String, _base: usize) -> BigNum {
        let base = BigNum::new(_base as isize);
        let mut res = BigNum::new(0);
        let mut flip = false;

        for (i, c) in s.chars().enumerate() {
            if i == 0 && c == '-' {
                flip = true;
                continue;
            }
            let k = if '0' <= c && c <= '9' {
                c as isize - '0' as isize
            } else if 'A' <= c && c <= 'Z' {
                c as isize - 'A' as isize + 10
            } else {
                0
            };
            res *= &base;
            res += &BigNum::new(k);
        }

        if flip {
            res.pos = false;
        }
        res
    }

    pub fn to_string(&self) -> String {
        self.to_string_base(10)
    }

    // TODO: https://en.wikipedia.org/wiki/Double_dabble
    // assert: _base <= 36
    pub fn to_string_base(&self, _base: usize) -> String {
        let base = BigNum::new(_base as isize);
        let mut res = String::new();
        let mut num = self.clone();
        num.pos = true;

        while !num.is_zero() {
            let k = &num % &base;
            num /= &base;

            res.push(if k.val[0] < 10 {
                ('0' as u8 + k.val[0] as u8) as char
            } else {
                ('A' as u8 + k.val[0] as u8 - 10) as char
            });
        }

        if !self.pos {
            res.push('-')
        }
        res.chars().rev().collect()
    }

    fn shrink_to_fit(&mut self) {
        while match self.val.last() {
            Some(x) => *x == 0 && self.val.len() > 1,
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

        v
    }

    fn sub_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> (Vec<u32>, bool) {
        let mut v = vec![0; max(lhs.len(), rhs.len()) + 1];

        // a > b => a - b
        let (a, b, swapped) = if BigNum::less_core(lhs, rhs) {
            (rhs, lhs, true)
        } else {
            (lhs, rhs, false)
        };

        for i in 0..b.len() {
            let mut t = (a[i] as i64) - (b[i] as i64) - (v[i] as i64);
            if t < 0 {
                v[i + 1] = 1;
                t += 1i64 << 32;
            }
            v[i] = t as u32;
        }

        for i in b.len()..a.len() {
            let mut t = (a[i] as i64) - (v[i] as i64);
            if t < 0 {
                v[i + 1] = 1;
                t += 1i64 << 32;
            }
            v[i] = t as u32;
        }

        (v, swapped)
    }

    // TODO: https://en.wikipedia.org/wiki/Sch%C3%B6nhage%E2%80%93Strassen_algorithm
    fn mult_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        let mut v = vec![0; lhs.len() + rhs.len() + 1];

        for i in 0..lhs.len() {
            if lhs[i] == 0 {
                continue;
            }

            for j in 0..rhs.len() {
                let t = (lhs[i] as u64) * (rhs[j] as u64);
                v[i + j] += t % (1u64 << 32);
                v[i + j + 1] += t / (1u64 << 32);
                v[i + j + 1] += v[i + j] / (1u64 << 32);
                v[i + j] %= 1u64 << 32;
            }
        }

        v.iter().map(|&x| x as u32).collect()
    }

    // TODO: https://en.wikipedia.org/wiki/Division_algorithm
    fn div_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
        let mut v = vec![0; max(lhs.len(), rhs.len())];

        for i in (0..v.len()).rev() {
            for j in (0..32).rev() {
                v[i] += 1u32 << j;
                if BigNum::less_core(lhs, &BigNum::mult_core(&v, rhs)) {
                    v[i] -= 1u32 << j;
                }
            }
        }

        v
    }

    fn less_core(lhs: &Vec<u32>, rhs: &Vec<u32>) -> bool {
        let mut a = lhs.len() - 1;
        let mut b = rhs.len() - 1;

        while a > 0 && lhs[a] == 0 {
            a -= 1;
        }

        while b > 0 && rhs[b] == 0 {
            b -= 1;
        }

        if a != b {
            a < b
        } else {
            a += 1;
            for i in (0..a).rev() {
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

    pub fn add(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let mut need_flip = false;
        let mut res = BigNum::from_vec(if lhs.pos {
            if rhs.pos {
                BigNum::add_core(&lhs.val, &rhs.val)
            } else {
                let (tmp, swapped) = BigNum::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            }
        } else {
            let t = if rhs.pos {
                let (tmp, swapped) = BigNum::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            } else {
                BigNum::add_core(&lhs.val, &rhs.val)
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

    pub fn sub(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let mut need_flip = false;
        let mut res = BigNum::from_vec(if lhs.pos {
            if !rhs.pos {
                BigNum::add_core(&lhs.val, &rhs.val)
            } else {
                let (tmp, swapped) = BigNum::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            }
        } else {
            let t = if !rhs.pos {
                let (tmp, swapped) = BigNum::sub_core(&lhs.val, &rhs.val);
                need_flip ^= swapped;
                tmp
            } else {
                BigNum::add_core(&lhs.val, &rhs.val)
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

    pub fn mul(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let mut res = BigNum::from_vec(BigNum::mult_core(&lhs.val, &rhs.val));
        if lhs.pos ^ rhs.pos {
            res.minus();
        }
        res
    }

    pub fn div(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let mut res = BigNum::from_vec(BigNum::div_core(&lhs.val, &rhs.val));
        if !lhs.pos {
            res += &BigNum::one();
            res.minus();
        }
        if !rhs.pos {
            res.minus();
        }
        res
    }

    pub fn rem(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let q = BigNum::div(&lhs, &rhs);
        BigNum::sub(&lhs, &BigNum::mul(&q, &rhs))
    }

    // TODO: https://en.wikipedia.org/wiki/Lehmer%27s_GCD_algorithm
    pub fn gcd(lhs: &BigNum, rhs: &BigNum) -> BigNum {
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

    pub fn neg(v: &BigNum) -> BigNum {
        BigNum {
            pos: !v.pos,
            val: v.val.clone(),
        }
    }

    pub fn set_copy(&mut self, rhs: &BigNum) {
        self.val = rhs.val.clone();
        self.pos = rhs.pos;
    }

    pub fn set_move(&mut self, rhs: BigNum) {
        self.val = rhs.val;
        self.pos = rhs.pos;
    }
}

impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            true
        } else {
            self.pos == other.pos && self.val == other.val
        }
    }
}

impl PartialOrd for BigNum {
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
                BigNum::less_core(&self.val, &other.val)
            } else {
                true
            }
        } else {
            if other.pos {
                false
            } else {
                !BigNum::less_core(&other.val, &self.val)
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

impl fmt::Debug for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Add<&BigNum> for &BigNum {
    type Output = BigNum;

    fn add(self, rhs: &BigNum) -> Self::Output {
        BigNum::add(self, rhs)
    }
}

impl ops::AddAssign<&BigNum> for BigNum {
    fn add_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self + rhs);
    }
}

impl ops::Sub<&BigNum> for &BigNum {
    type Output = BigNum;

    fn sub(self, rhs: &BigNum) -> Self::Output {
        BigNum::sub(self, rhs)
    }
}

impl ops::SubAssign<&BigNum> for BigNum {
    fn sub_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self - rhs);
    }
}

impl ops::Mul<&BigNum> for &BigNum {
    type Output = BigNum;

    fn mul(self, rhs: &BigNum) -> Self::Output {
        BigNum::mul(self, rhs)
    }
}

impl ops::MulAssign<&BigNum> for BigNum {
    fn mul_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self * rhs);
    }
}

impl ops::Div<&BigNum> for &BigNum {
    type Output = BigNum;

    fn div(self, rhs: &BigNum) -> Self::Output {
        BigNum::div(self, rhs)
    }
}

impl ops::DivAssign<&BigNum> for BigNum {
    fn div_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self / rhs);
    }
}

impl ops::Rem<&BigNum> for &BigNum {
    type Output = BigNum;

    fn rem(self, rhs: &BigNum) -> Self::Output {
        BigNum::rem(self, rhs)
    }
}

impl ops::RemAssign<&BigNum> for BigNum {
    fn rem_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self % rhs);
    }
}

impl ops::Neg for &BigNum {
    type Output = BigNum;

    fn neg(self) -> Self::Output {
        BigNum::neg(self)
    }
}
