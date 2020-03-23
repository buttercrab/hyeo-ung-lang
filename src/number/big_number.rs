use std::cmp::{max, min, Ordering};
use std::{error, fmt, ops};

/// Error class for `BigNum`
/// Used in `BigNum::from_string()`, `BigNum::from_string_base()`, `BigNum::to_string_base()`.
///
/// # Examples
///
/// ```
/// use hyeong::number::big_number::BigNum;
/// use hyeong::number::big_number::Error;
///
/// let a = BigNum::new(1234);
///
/// assert!(matches!(BigNum::from_string(String::from("random string")), Result::Err(Error::ParseError)));
/// ```
#[derive(Debug)]
pub enum Error {
    ParseError,
    BaseSizeError(usize),
}

impl fmt::Display for Error {
    /// Formats to String
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::Error;
    ///
    /// assert_eq!("Parse Error", format!("{}", Error::ParseError));
    /// assert_eq!("Base Size Error: size = 100", format!("{}", Error::BaseSizeError(100)));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError => write!(f, "Parse Error"),
            Error::BaseSizeError(s) => write!(f, "Base Size Error: size = {}", s),
        }
    }
}

impl error::Error for Error {}

/// `BigNum` for big number handling
/// - Using `Vec<u32>` for data and using `u32::max_value()` as base of the number
/// - Can handle negative numbers
///
/// # Examples
///
/// ```
/// use hyeong::number::big_number::BigNum;
///
/// // Ways to make 10
/// let a = BigNum::new(10);
/// let b = BigNum::from_vec(vec![10]);
/// let c = BigNum::from_string(String::from("10")).unwrap();
/// let d = BigNum::from_string_base(String::from("1010"), 2).unwrap();
///
/// // Arithmetic operators
/// let e = &a + &b;
/// let f = &a - &b;
/// let g = &a * &b;
/// let h = &a / &b;
/// let i = &a % &b;
///
/// // Compare operators
/// let j = &a == &b;
/// let k = &a < &b;
///
/// assert_eq!("10", format!("{}", a));
/// assert_eq!("10", format!("{}", b));
/// assert_eq!("10", format!("{}", c));
/// assert_eq!("10", format!("{}", d));
/// ```
#[derive(Clone)]
pub struct BigNum {
    pos: bool,
    val: Vec<u32>,
}

impl BigNum {
    /// Makes new `BigNum` from the number
    /// Supports negative nubmers
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("1234", a.to_string());
    /// assert_eq!("-4321", b.to_string());
    /// ```
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

    /// Makes new `BigNum` from vector
    /// Doesn't support negative number
    /// vector is `u32::max_value()` based number
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::from_vec(vec![1234]);
    /// let b = BigNum::from_vec(vec![0, 1]);
    ///
    /// assert_eq!("1234", a.to_string());
    /// assert_eq!("4294967296", b.to_string());
    /// assert!(a.is_pos());
    /// ```
    pub fn from_vec(v: Vec<u32>) -> BigNum {
        let mut res = BigNum { pos: true, val: v };
        res.shrink_to_fit();
        res
    }

    /// Makes new zero `BigNum`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::zero();
    ///
    /// assert_eq!("0", a.to_string());
    /// ```
    pub fn zero() -> BigNum {
        BigNum {
            pos: true,
            val: vec![0],
        }
    }

    /// Makes new one `BigNum`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::one();
    ///
    /// assert_eq!("1", a.to_string());
    /// ```
    pub fn one() -> BigNum {
        BigNum {
            pos: true,
            val: vec![1],
        }
    }

    /// Check if the number is positive
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!(true, a.is_pos());
    /// assert_eq!(false, b.is_pos());
    /// ```
    pub fn is_pos(&self) -> bool {
        self.pos
    }

    /// Check if the number is zero
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::zero();
    ///
    /// assert_eq!(false, a.is_zero());
    /// assert_eq!(true, b.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.val == vec![0]
    }

    /// Returns to `u32` type
    ///
    /// # Assertions
    ///
    /// - the value is same when the value is under `u32::max_value()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    ///
    /// assert_eq!(1234, a.to_int());
    /// ```
    pub fn to_int(&self) -> u32 {
        self.val[0]
    }

    /// Make new `BigNum` from string (10 based)
    /// It won't make error when the format is not right.
    /// But it would make unexpected number.
    /// Negative numbers are supported.
    ///
    /// # Time Complexity
    ///
    /// `O(n^2)` where `n := s.len()`
    ///
    /// # Errors
    ///
    /// - `Error::ParseError`: when parse failed
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::from_string(String::from("12345678987654321")).unwrap();
    /// let b = BigNum::from_string(String::from("-98765432123456789")).unwrap();
    ///
    /// assert_eq!("12345678987654321", a.to_string());
    /// assert_eq!("-98765432123456789", b.to_string());
    /// ```
    pub fn from_string(s: String) -> Result<BigNum, Error> {
        BigNum::from_string_base(s, 10)
    }

    /// Make new `BigNum` from string
    /// It won't make error when the format is not right.
    /// But it would make unexpected number.
    /// Negative numbers are supported.
    ///
    /// # Time Complexity
    ///
    /// `O(n^2)` where `n := s.len()`
    ///
    /// # Assertions
    ///
    /// - `0 < _base <= 36`
    ///
    /// # Errors
    ///
    /// - `Error::BaseSizeError(base)`: when base size is not in range
    /// - `Error::ParseError`: when parse failed
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::from_string_base(String::from("A"), 16).unwrap();
    /// let b = BigNum::from_string_base(String::from("-1010"), 2).unwrap();
    ///
    /// assert_eq!("10", a.to_string());
    /// assert_eq!("-10", b.to_string());
    /// ```
    pub fn from_string_base(s: String, _base: usize) -> Result<BigNum, Error> {
        if _base > 36 || _base < 1 {
            return Result::Err(Error::BaseSizeError(_base));
        }

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
                return Result::Err(Error::ParseError);
            };
            res *= &base;
            res += &BigNum::new(k);
        }

        if flip {
            res.pos = false;
        }
        Result::Ok(res)
    }

    /// Make string from itself (10 based)
    /// Negative numbers are supported
    ///
    /// # Time Complexity
    ///
    /// `O(n^2)` where `n := res.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("1234", a.to_string());
    /// assert_eq!("-4321", b.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        self.to_string_base(10).unwrap()
    }

    /// Make string from itself
    /// Negative numbers are supported
    ///
    /// # Time Complexity
    ///
    /// `O(n^2)` where `n := res.len()`
    ///
    /// # Assertions
    ///
    /// - `0 < _base <= 36`
    ///
    /// # Errors
    ///
    /// - `Error::BaseSizeError(base)`: when base size is not in range
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(10);
    /// let b = BigNum::new(-10);
    ///
    /// assert_eq!("A", a.to_string_base(16).unwrap());
    /// assert_eq!("-1010", b.to_string_base(2).unwrap());
    /// ```
    ///
    /// # TODO
    ///
    /// - [Better Algorithm](https://en.wikipedia.org/wiki/Double_dabble)
    pub fn to_string_base(&self, _base: usize) -> Result<String, Error> {
        if _base > 36 || _base < 1 {
            return Result::Err(Error::BaseSizeError(_base));
        }

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

        if res.is_empty() {
            res = String::from("0");
        }

        if !self.pos {
            res.push('-')
        }
        Result::Ok(res.chars().rev().collect())
    }

    /// Private function for removing leading zero in data.
    /// However, if the value is 0, it would leave one zero.
    ///
    /// # Time Complexity
    ///
    /// `O(n)` where `n := self.val.len()`
    fn shrink_to_fit(&mut self) {
        while match self.val.last() {
            Some(x) => *x == 0 && self.val.len() > 1,
            None => false,
        } {
            self.val.pop();
        }
    }

    /// Private function for adding two numbers. (Core function)
    /// Gets two vectors of data and returns new vector of result.
    /// It assumes two value are positive.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.len()` and `m := rhs.len()`
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

        let t = if lhs.len() < rhs.len() { rhs } else { lhs };

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

    /// Private function for subtracting two numbers. (Core function)
    /// Gets two vectors of data and returns new vector of result.
    /// It assumes two value are positive.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.len()` and `m := rhs.len()`
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

    /// Private function for multiplying two numbers. (Core function)
    /// Gets two vectors of data and returns new vector of result.
    /// It assumes two value are positive.
    ///
    /// # Time Complexity
    ///
    /// `O(n + m * k)` where `n := lhs.len()`, `m := non-zero values in lhs` and `k := rhs.len()`
    ///
    /// # TODO
    ///
    /// - [Better Algorithm](https://en.wikipedia.org/wiki/Sch%C3%B6nhage%E2%80%93Strassen_algorithm)
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

    /// Private function for dividing two numbers. (Core function)
    /// Gets two vectors of data and returns new vector of result.
    /// It assumes two value are positive.
    ///
    /// # Time Complexity
    ///
    /// `O(n * max(n, m))` where `n := lhs.len()` and `m := rhs.len()`
    ///
    /// # TODO
    ///
    /// - [Better Algorithm](https://en.wikipedia.org/wiki/Division_algorithm)
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

    /// Private function for comparing two numbers. (Core function)
    /// Gets two vectors of data and returns the result.
    /// It assumes two values are positive.
    ///
    /// # Time complexity
    ///
    /// `O(max(n, m))` where `n := lhs.len()` and `m := rhs.len()`
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

    /// Make its sign flip
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let mut a = BigNum::new(1234);
    /// let mut b = BigNum::new(-4321);
    ///
    /// a.minus();
    /// b.minus();
    ///
    /// assert_eq!("-1234", a.to_string());
    /// assert_eq!("4321", b.to_string());
    /// ```
    pub fn minus(&mut self) {
        if !self.is_zero() {
            self.pos = !self.pos;
        }
    }

    /// Adds two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("-3087", BigNum::add(&a, &b).to_string());
    /// ```
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

    /// Subtracts two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("5555", BigNum::sub(&a, &b).to_string());
    /// ```
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

    /// Multiplies two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(n + m * k)` where `n := lhs.val.len()`, `m := non-zero values in lhs.val` and `k := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("-5332114", BigNum::mul(&a, &b).to_string());
    /// ```
    pub fn mul(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let mut res = BigNum::from_vec(BigNum::mult_core(&lhs.val, &rhs.val));

        if lhs.pos ^ rhs.pos {
            res.minus();
        }

        res
    }

    /// Divides two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(n * max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Warning
    ///
    /// In Python `-1234 / 31 == -40` (same calculation as below).
    /// But in this lib, it works like this:
    /// always `|a| / |b| == |q|` (rounding to zero)
    /// Rust works same like this lib.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(-1234);
    /// let b = BigNum::new(31);
    ///
    /// assert_eq!("-39", BigNum::div(&a, &b).to_string());
    /// ```
    pub fn div(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let mut res = BigNum::from_vec(BigNum::div_core(&lhs.val, &rhs.val));

        if lhs.pos ^ rhs.pos {
            res.minus();
        }

        res
    }

    /// Get remainder of two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(div(lhs, rhs) + mul(q, rhs))` where `q := lhs / rhs`
    ///
    /// # Warning
    ///
    /// In Python `-1234 / 31 == -40` (same calculation as below).
    /// But in this lib, it works like this:
    /// always `|a| / |b| == |q|` (rounding to zero)
    /// Rust works same like this lib.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(-1234);
    /// let b = BigNum::new(31);
    ///
    /// assert_eq!("-25", BigNum::rem(&a, &b).to_string());
    /// ```
    pub fn rem(lhs: &BigNum, rhs: &BigNum) -> BigNum {
        let q = BigNum::div(&lhs, &rhs);
        BigNum::sub(&lhs, &BigNum::mul(&q, &rhs))
    }

    /// Get greatest common value of two number and make new `BigNum` as result
    ///
    /// # Time Complexity
    ///
    /// `O(log(n + m))` where `n := lhs as value` and `m := rhs as value`
    ///
    /// # Example
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(18);
    /// let b = BigNum::new(24);
    ///
    /// assert_eq!("6", BigNum::gcd(&a, &b).to_string());
    /// ```
    ///
    /// # TODO
    ///
    /// - [Better Algorithm](https://en.wikipedia.org/wiki/Lehmer%27s_GCD_algorithm)
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

    /// Returns new `BigNum` that minus is applied.
    ///
    /// # Example
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::neg(&a);
    ///
    /// assert_eq!("-1234", b.to_string());
    /// ```
    pub fn neg(v: &BigNum) -> BigNum {
        BigNum {
            pos: if !v.is_zero() { !v.pos } else { v.pos },
            val: v.val.clone(),
        }
    }

    /// Makes `self` same value as `rhs` but copying the value
    ///
    /// # Example
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let mut b = BigNum::zero();
    ///
    /// b.set_copy(&a);
    ///
    /// assert_eq!("1234", b.to_string());
    /// ```
    pub fn set_copy(&mut self, rhs: &BigNum) {
        self.val = rhs.val.clone();
        self.pos = rhs.pos;
    }

    /// Makes `self` same value as `rhs` but moving the value
    ///
    /// # Example
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let mut b = BigNum::zero();
    ///
    /// b.set_move(a);
    /// //         ^ `a` moved here
    ///
    /// assert_eq!("1234", b.to_string());
    /// ```
    pub fn set_move(&mut self, rhs: BigNum) {
        self.val = rhs.val;
        self.pos = rhs.pos;
    }
}

impl PartialEq for BigNum {
    /// Equal function of two `BigNum`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(1234);
    /// let c = BigNum::new(-1234);
    ///
    /// assert_eq!(true, a == b);
    /// assert_eq!(false, a == c);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            true
        } else {
            self.pos == other.pos && self.val == other.val
        }
    }
}

impl PartialOrd for BigNum {
    /// Compare function of two `BigNum`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    /// use std::cmp::Ordering;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(4321);
    ///
    /// assert_eq!(Ordering::Less, if let Some(t) = a.partial_cmp(&b) {
    ///     t
    /// } else {
    ///     unreachable!();
    /// });
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Option::Some(Ordering::Equal)
        } else if if self.pos {
            if other.pos {
                BigNum::less_core(&self.val, &other.val)
            } else {
                false
            }
        } else {
            if other.pos {
                true
            } else {
                BigNum::less_core(&other.val, &self.val)
            }
        } {
            Option::Some(Ordering::Less)
        } else {
            Option::Some(Ordering::Greater)
        }
    }
}

impl fmt::Debug for BigNum {
    /// Printing feature of `BigNum`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    ///
    /// assert_eq!("1234", format!("{:?}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for BigNum {
    /// Printing feature of `BigNum`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    ///
    /// assert_eq!("1234", format!("{}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Add<&BigNum> for &BigNum {
    type Output = BigNum;

    /// Adds two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("-3087", (&a + &b).to_string());
    /// ```
    fn add(self, rhs: &BigNum) -> Self::Output {
        BigNum::add(self, rhs)
    }
}

impl ops::AddAssign<&BigNum> for BigNum {
    /// Adds two number and move the value to `self`
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let mut a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    /// a += &b;
    ///
    /// assert_eq!("-3087", a.to_string());
    /// ```
    fn add_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self + rhs);
    }
}

impl ops::Sub<&BigNum> for &BigNum {
    type Output = BigNum;

    /// Subtracts two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("5555", (&a - &b).to_string());
    /// ```
    fn sub(self, rhs: &BigNum) -> Self::Output {
        BigNum::sub(self, rhs)
    }
}

impl ops::SubAssign<&BigNum> for BigNum {
    /// Subtracts two number and move the value to `self`
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let mut a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    /// a -= &b;
    ///
    /// assert_eq!("5555", a.to_string());
    /// ```
    fn sub_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self - rhs);
    }
}

impl ops::Mul<&BigNum> for &BigNum {
    type Output = BigNum;

    /// Multiplies two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(n + m * k)` where `n := lhs.val.len()`, `m := non-zero values in lhs.val` and `k := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    ///
    /// assert_eq!("-5332114", (&a * &b).to_string());
    /// ```
    fn mul(self, rhs: &BigNum) -> Self::Output {
        BigNum::mul(self, rhs)
    }
}

impl ops::MulAssign<&BigNum> for BigNum {
    /// Multiplies two number and move the value to `self`
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(n + m * k)` where `n := lhs.val.len()`, `m := non-zero values in lhs.val` and `k := rhs.val.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let mut a = BigNum::new(1234);
    /// let b = BigNum::new(-4321);
    /// a *= &b;
    ///
    /// assert_eq!("-5332114", a.to_string());
    /// ```
    fn mul_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self * rhs);
    }
}

impl ops::Div<&BigNum> for &BigNum {
    type Output = BigNum;

    /// Divides two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(n * max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Warning
    ///
    /// In Python `-1234 / 31 == -40` (same calculation as below).
    /// But in this lib, it works like this:
    /// always `|a| / |b| == |q|` (rounding to zero)
    /// Rust works same like this lib.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(-1234);
    /// let b = BigNum::new(31);
    ///
    /// assert_eq!("-39", (&a / &b).to_string());
    /// ```
    fn div(self, rhs: &BigNum) -> Self::Output {
        BigNum::div(self, rhs)
    }
}

impl ops::DivAssign<&BigNum> for BigNum {
    /// Divides two number and move to `self`
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(n * max(n, m))` where `n := lhs.val.len()` and `m := rhs.val.len()`
    ///
    /// # Warning
    ///
    /// In Python `-1234 / 31 == -40` (same calculation as below).
    /// But in this lib, it works like this:
    /// always `|a| / |b| == |q|` (rounding to zero)
    /// Rust works same like this lib.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let mut a = BigNum::new(-1234);
    /// let b = BigNum::new(31);
    /// a /= &b;
    ///
    /// assert_eq!("-39", a.to_string());
    /// ```
    fn div_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self / rhs);
    }
}

impl ops::Rem<&BigNum> for &BigNum {
    type Output = BigNum;

    /// Get remainder of two number and make new `BigNum` as result
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(div(lhs, rhs) + mul(q, rhs))` where `q := lhs / rhs`
    ///
    /// # Warning
    ///
    /// In Python `-1234 / 31 == -40` (same calculation as below).
    /// But in this lib, it works like this:
    /// always `|a| / |b| == |q|` (rounding to zero)
    /// Rust works same like this lib.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(-1234);
    /// let b = BigNum::new(31);
    ///
    /// assert_eq!("-25", (&a % &b).to_string());
    /// ```
    fn rem(self, rhs: &BigNum) -> Self::Output {
        BigNum::rem(self, rhs)
    }
}

impl ops::RemAssign<&BigNum> for BigNum {
    /// Get remainder of two number and move to `self`
    /// Support all sign types.
    ///
    /// # Time Complexity
    ///
    /// `O(div(lhs, rhs) + mul(q, rhs))` where `q := lhs / rhs`
    ///
    /// # Warning
    ///
    /// In Python `-1234 / 31 == -40` (same calculation as below).
    /// But in this lib, it works like this:
    /// always `|a| / |b| == |q|` (rounding to zero)
    /// Rust works same like this lib.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let mut a = BigNum::new(-1234);
    /// let b = BigNum::new(31);
    /// a %= &b;
    ///
    /// assert_eq!("-25", a.to_string());
    /// ```
    fn rem_assign(&mut self, rhs: &BigNum) {
        self.set_move(&*self % rhs);
    }
}

impl ops::Neg for &BigNum {
    type Output = BigNum;

    /// Returns new `BigNum` that minus is applied.
    ///
    /// # Example
    ///
    /// ```
    /// use hyeong::number::big_number::BigNum;
    ///
    /// let a = BigNum::new(1234);
    ///
    /// assert_eq!("-1234", (-&a).to_string());
    /// ```
    fn neg(self) -> Self::Output {
        BigNum::neg(self)
    }
}
