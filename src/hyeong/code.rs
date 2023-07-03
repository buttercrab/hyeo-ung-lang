use derive_more::{Constructor, Display};

use crate::hyeong::area::Area;
use crate::hyeong::parse::Span;

/// Code Types
#[derive(Debug, Copy, Clone, Display, Eq, PartialEq, Hash)]
pub enum HangulType {
    #[display(fmt = "형")]
    Hyeong,
    #[display(fmt = "항")]
    Hang,
    #[display(fmt = "핫")]
    Hat,
    #[display(fmt = "흣")]
    Heut,
    #[display(fmt = "흡")]
    Heup,
    #[display(fmt = "흑")]
    Heuk,
}

/// Code trait
///
/// It defines methods that code structure needs.
/// [UnOptCode](struct.UnOptCode.html) and [OptCode](struct.OptCode.html) use this trait.
pub trait Code {
    fn hangul_type(&self) -> HangulType;

    fn hangul_count(&self) -> usize;

    fn dot_count(&self) -> usize;

    fn area(&self) -> &Area;

    fn area_count(&self) -> usize;
}

/// structure for optimized code
///
/// It contains a single command.
/// It can be used for level 1, 2 optimization.
///
/// # Examples
///
/// ```
/// use hyeong::core::code::{OptCode, Code};
/// use hyeong::core::area::Area;
///
/// let a = OptCode::new(
///     0,
///     10,
///     10,
///     20,
///     Area::new(3)
/// );
///
/// assert_eq!(0, a.get_type());
/// assert_eq!(10, a.get_hangul_count());
/// assert_eq!(10, a.get_dot_count());
/// assert_eq!(20, a.get_area_count());
/// ```
#[derive(Debug, Clone, Constructor)]
pub struct OptCode {
    hangul_type: HangulType,
    hangul_count: usize,
    dot_count: usize,
    area_count: usize,
    area: Area,
}

impl Code for OptCode {
    /// Return type of code
    fn hangul_type(&self) -> HangulType {
        self.hangul_type
    }

    /// Return hangul count of code
    fn hangul_count(&self) -> usize {
        self.hangul_count
    }

    /// Return dot count of code
    fn dot_count(&self) -> usize {
        self.dot_count
    }

    /// Return Area of code
    fn area(&self) -> &Area {
        &self.area
    }

    /// Return area count of code
    fn area_count(&self) -> usize {
        self.area_count
    }
}

/// structure for optimized code
///
/// # Examples
///
/// ```
/// use hyeong::core::code::UnOptCode;
/// use hyeong::core::area::Area;
///
/// let a = UnOptCode::new(0, 1, 2, (1, 2), Area::Nil, String::from("형.."));
/// assert_eq!("type: 0, cnt1: 1, cnt2: 2, area: \"_\"", format!("{:?}", a));
/// ```
#[derive(Debug, Clone, Constructor)]
pub struct UnOptCode<'a> {
    hangul_type: HangulType,
    hangul_count: usize,
    dot_count: usize,
    start: Span<'a>,
    end: Span<'a>,
    area: Area,
}

impl<'a> UnOptCode<'a> {
    /// Return location
    pub fn start_span(&self) -> Span<'a> {
        self.start
    }

    /// Return raw code
    pub fn raw(&self) -> &str {
        &self.start.extra[self.start.location_offset()..self.end.location_offset()]
    }
}

impl Code for UnOptCode<'_> {
    /// Return type of code
    fn hangul_type(&self) -> HangulType {
        self.hangul_type
    }

    /// Return hangul count of code
    fn hangul_count(&self) -> usize {
        self.hangul_count
    }

    /// Return dot count of code
    fn dot_count(&self) -> usize {
        self.dot_count
    }

    /// Return Area of code
    fn area(&self) -> &Area {
        &self.area
    }

    /// Return area count of code
    fn area_count(&self) -> usize {
        self.hangul_count * self.dot_count
    }
}
