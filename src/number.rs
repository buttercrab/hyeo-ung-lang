use std::cmp::Ordering;
use std::mem::swap;
use std::{fmt, ops};

use crate::big_number::BigNum;

/// `Num` for rational number handling
/// - Using two `BigNum` for denominator and numerator.
/// - Can handle negative numbers
/// - Can handle NaN
///
/// # Examples
///
/// ```
/// use hyeong::number::Num;
///
/// let a = Num::from_num(10);
/// let mut  b = Num::from_num(3);
/// b.flip();
///
/// let c = &a * &b;
///
/// assert_eq!("10/3", c.to_string());
/// ```
#[derive(PartialEq, Clone)]
pub struct Num {
    up: BigNum,
    down: BigNum,
}

impl Num {
    /// Makes new `Num` from the denominator and numerator.
    /// Supports negative numbers (only denominator)
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::new(-10, 6);
    ///
    /// assert_eq!("10/3", a.to_string());
    /// assert_eq!("-5/3", b.to_string());
    /// ```
    pub fn new(up: isize, down: usize) -> Num {
        let mut res = Num {
            up: BigNum::new(up),
            down: BigNum::new(down as isize),
        };
        res.optimize();
        res
    }

    /// Makes new `Num`
    /// Support negative numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::from_num(1234);
    ///
    /// assert_eq!("1234", a.to_string());
    /// ```
    pub fn from_num(n: isize) -> Num {
        Num {
            up: BigNum::new(n),
            down: BigNum::one(),
        }
    }

    /// Makes new `Num` from `BigNum`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    /// use hyeong::big_number::BigNum;
    ///
    /// let a = BigNum::new(1000);
    /// let b = BigNum::new(14);
    ///
    /// assert_eq!("500/7", Num::from_big_num(a, b).to_string());
    /// ```
    pub fn from_big_num(up: BigNum, down: BigNum) -> Num {
        let mut res = Num { up, down };
        res.optimize();
        res
    }

    /// Makes new zero `Num`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::zero();
    ///
    /// assert_eq!("0", a.to_string());
    /// ```
    pub fn zero() -> Num {
        Num {
            up: BigNum::zero(),
            down: BigNum::one(),
        }
    }

    /// Makes new one `Num`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::one();
    ///
    /// assert_eq!("1", a.to_string());
    /// ```
    pub fn one() -> Num {
        Num {
            up: BigNum::one(),
            down: BigNum::one(),
        }
    }

    /// Makes new NaN `Num`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::nan();
    ///
    /// assert_eq!("너무 커엇...", a.to_string());
    /// ```
    pub fn nan() -> Num {
        Num {
            up: BigNum::one(),
            down: BigNum::zero(),
        }
    }

    /// Floor function for `Num`
    /// Works for positive values.
    ///
    /// # Assertions
    ///
    /// - `self` is not NaN
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(7, 3);
    ///
    /// assert_eq!("2", a.floor().to_string());
    /// ```
    pub fn floor(&self) -> BigNum {
        if self.down == BigNum::one() {
            self.up.clone()
        } else {
            &self.up / &self.down
        }
    }

    /// Check if the number is positive
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::from_num(10);
    ///
    /// assert_eq!(true, a.is_pos());
    /// ```
    pub fn is_pos(&self) -> bool {
        self.up.is_pos() && !self.is_nan()
    }

    /// Check if the number is NaN
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::nan();
    /// let b = Num::new(10, 3);
    ///
    /// assert_eq!(true, a.is_nan());
    /// assert_eq!(false, b.is_nan());
    /// ```
    pub fn is_nan(&self) -> bool {
        self.down.is_zero()
    }

    /// Make string from itself (10 based)
    /// Negative numbers and NaN are supported
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::nan();
    /// let c = Num::from_num(-12);
    ///
    /// assert_eq!("10/3", a.to_string());
    /// assert_eq!("너무 커엇...", b.to_string());
    /// assert_eq!("-12", c.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        if self.is_nan() {
            format!("너무 커엇...")
        } else {
            if self.down == BigNum::one() {
                format!("{}", self.up.to_string())
            } else {
                format!("{}/{}", self.up.to_string(), self.down.to_string())
            }
        }
    }

    /// Optimize (abbreviation) the number
    /// Also, it makes that only denominator is positive.
    fn optimize(&mut self) {
        let g = BigNum::gcd(&self.up, &self.down);
        self.up /= &g;
        self.down /= &g;
    }

    /// Make itself change the sign
    /// If the value is Nan, the result is NaN.
    ///
    /// # Example
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let mut a = Num::new(10, 3);
    /// a.minus();
    ///
    /// assert_eq!("-10/3", a.to_string());
    /// ```
    pub fn minus(&mut self) {
        self.up.minus();
    }

    /// Flip its denominator and numerator
    /// (`n` to `1/n`)
    /// if NaN, it does nothing
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let mut a = Num::new(-10, 3);
    /// a.flip();
    ///
    /// assert_eq!("-3/10", a.to_string());
    /// ```
    pub fn flip(&mut self) {
        if !self.is_nan() {
            swap(&mut self.up, &mut self.down);
            if !self.down.is_pos() {
                self.down.minus();
                self.up.minus();
            }
        }
    }

    /// Add two `Num` and returns new `Num` as result
    /// If any of the value is Nan, the result is NaN.
    ///
    /// # Time Complexity
    ///
    /// `O(a * d + b * c + c * d)` where `a / b := lhs` and `c / d := rhs`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::new(7, 5);
    /// let c = Num::add(&a, &b);
    ///
    /// assert_eq!("71/15", c.to_string());
    /// ```
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

    /// Multiply two `Num` and returns new `Num` as result
    /// If any of the value is Nan, the result is NaN.
    ///
    /// # Time Complexity
    ///
    /// `O(a * b + c * d)` where `a / b := lhs` and `c / d := rhs`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::new(7, 5);
    /// let c = Num::mul(&a, &b);
    ///
    /// assert_eq!("14/3", c.to_string());
    /// ```
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

    /// Returns new `Num` that minus is applied.
    /// If the value is Nan, the result is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::neg(&a);
    ///
    /// assert_eq!("-10/3", b.to_string());
    /// ```
    pub fn neg(v: &Num) -> Num {
        Num {
            up: (-&v.up).clone(),
            down: (&v.down).clone(),
        }
    }

    /// Makes `self` same value as `rhs` but copying the values
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let mut b = Num::zero();
    ///
    /// b.set_copy(&a);
    ///
    /// assert_eq!("10/3", b.to_string());
    /// ```
    pub fn set_copy(&mut self, rhs: &Num) {
        self.up.set_copy(&rhs.up);
        self.down.set_copy(&rhs.down);
    }

    /// Makes `self` same value as `rhs` but copying the values
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let mut b = Num::zero();
    ///
    /// b.set_move(a);
    /// //         ^ `a` moved here
    ///
    /// assert_eq!("10/3", b.to_string());
    /// ```
    pub fn set_move(&mut self, rhs: Num) {
        self.up.set_move(rhs.up);
        self.down.set_move(rhs.down);
    }
}

impl PartialOrd for Num {
    /// Compare function of two `Num`
    /// If any of the value is Nan, the result is None.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    /// use std::cmp::Ordering;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::new(7, 4);
    ///
    /// assert_eq!(Ordering::Greater, if let Some(t) = a.partial_cmp(&b) {
    ///     t
    /// } else {
    ///     unreachable!()
    /// })
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            Option::None
        } else {
            if self == other {
                Option::Some(Ordering::Equal)
            } else if &self.up * &other.down < &self.down * &other.down {
                Option::Some(Ordering::Less)
            } else {
                Option::Some(Ordering::Greater)
            }
        }
    }
}

impl fmt::Debug for Num {
    /// Printing function of `Num`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    ///
    /// assert_eq!("10/3", format!("{:?}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Num {
    /// Printing function of `Num`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    ///
    /// assert_eq!("10/3", format!("{}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Add<&Num> for &Num {
    type Output = Num;

    /// Add two `Num` and returns new `Num` as result
    /// If any of the value is Nan, the result is NaN.
    ///
    /// # Time Complexity
    ///
    /// `O(a * d + b * c + c * d)` where `a / b := lhs` and `c / d := rhs`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::new(7, 5);
    /// let c = &a + &b;
    ///
    /// assert_eq!("71/15", c.to_string());
    /// ```
    fn add(self, rhs: &Num) -> Self::Output {
        Num::add(self, rhs)
    }
}

impl ops::AddAssign<&Num> for Num {
    /// Add two `Num` and moves the value to `self`
    /// If any of the value is Nan, the result is NaN.
    ///
    /// # Time Complexity
    ///
    /// `O(a * d + b * c + c * d)` where `a / b := lhs` and `c / d := rhs`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let mut a = Num::new(10, 3);
    /// let b = Num::new(7, 5);
    /// a += &b;
    ///
    /// assert_eq!("71/15", a.to_string());
    /// ```
    fn add_assign(&mut self, rhs: &Num) {
        self.set_move(&*self + rhs);
    }
}

impl ops::Mul<&Num> for &Num {
    type Output = Num;

    /// Multiply two `Num` and returns new `Num` as result
    /// If any of the value is Nan, the result is NaN.
    ///
    /// # Time Complexity
    ///
    /// `O(a * b + c * d)` where `a / b := lhs` and `c / d := rhs`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = Num::new(7, 5);
    /// let c = &a * &b;
    ///
    /// assert_eq!("14/3", c.to_string());
    /// ```
    fn mul(self, rhs: &Num) -> Self::Output {
        Num::mul(self, rhs)
    }
}

impl ops::MulAssign<&Num> for Num {
    /// Multiply two `Num` and moves the value to `self`
    /// If any of the value is Nan, the result is NaN.
    ///
    /// # Time Complexity
    ///
    /// `O(a * b + c * d)` where `a / b := lhs` and `c / d := rhs`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let mut a = Num::new(10, 3);
    /// let b = Num::new(7, 5);
    /// a *= &b;
    ///
    /// assert_eq!("14/3", a.to_string());
    /// ```
    fn mul_assign(&mut self, rhs: &Num) {
        self.set_move(&*self * rhs);
    }
}

impl ops::Neg for &Num {
    type Output = Num;

    /// Returns new `Num` that minus is applied.
    /// If the value is Nan, the result is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::number::Num;
    ///
    /// let a = Num::new(10, 3);
    /// let b = -&a;
    ///
    /// assert_eq!("-10/3", b.to_string());
    /// ```
    fn neg(self) -> Self::Output {
        Num::neg(self)
    }
}
