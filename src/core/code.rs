use crate::core::area;
use crate::core::area::Area;
use std::fmt;

/// Code trait
///
/// It defines methods that code structure needs.
/// [UnOptCode](struct.UnOptCode.html) and [OptCode](struct.OptCode.html) use this trait.
pub trait Code {
    fn get_type(&self) -> u8;

    fn get_hangul_count(&self) -> usize;

    fn get_dot_count(&self) -> usize;

    fn get_area(&self) -> &Area;

    fn get_area_count(&self) -> usize;
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
#[derive(Clone)]
pub struct OptCode {
    type_: u8,
    hangul_count: usize,
    dot_count: usize,
    area_count: usize,
    area: Area,
}

impl OptCode {
    /// Makes new `OptCode`
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
    pub fn new(
        type_: u8,
        hangul_count: usize,
        dot_count: usize,
        area_count: usize,
        area: Area,
    ) -> OptCode {
        OptCode {
            type_,
            hangul_count,
            dot_count,
            area_count,
            area,
        }
    }
}

impl Code for OptCode {
    /// Return type of code
    fn get_type(&self) -> u8 {
        self.type_
    }

    /// Return hangul count of code
    fn get_hangul_count(&self) -> usize {
        self.hangul_count
    }

    /// Return dot count of code
    fn get_dot_count(&self) -> usize {
        self.dot_count
    }

    /// Return Area of code
    fn get_area(&self) -> &Area {
        &self.area
    }

    /// Return area count of code
    fn get_area_count(&self) -> usize {
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
/// let a = UnOptCode::new(0, 1, 2, (1, 2), Area::Nil, "형..".to_string());
/// assert_eq!("type: 0, cnt1: 1, cnt2: 2, area: \"_\"", format!("{:?}", a));
/// ```
#[derive(Clone)]
pub struct UnOptCode {
    // 0: 형, 혀엉, 혀어엉, 혀어어엉 ...
    // 1: 항, 하앙, 하아앙, 하아아앙 ...
    // 2: 핫, 하앗, 하아앗, 하아아앗 ...
    // 3: 흣, 흐읏, 흐으읏, 흐으으읏 ...
    // 4: 흡, 흐읍, 흐으읍, 흐으으읍 ...
    // 5: 흑, 흐윽, 흐으윽, 흐으으윽 ...
    type_: u8,
    hangul_count: usize,
    dot_count: usize,
    loc: (usize, usize),
    area: Area,
    code: String,
}

impl UnOptCode {
    /// Make new `UnOptCode`
    pub fn new(
        type_: u8,
        hangul_count: usize,
        dot_count: usize,
        loc: (usize, usize),
        area: Area,
        code: String,
    ) -> UnOptCode {
        UnOptCode {
            type_,
            hangul_count,
            dot_count,
            loc,
            area,
            code,
        }
    }

    /// Return location
    pub fn get_location(&self) -> (usize, usize) {
        self.loc
    }

    /// Return raw code
    pub fn get_raw(&self) -> String {
        self.code.clone()
    }
}

impl fmt::Debug for UnOptCode {
    /// Debug format function
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::core::code::UnOptCode;
    /// use hyeong::core::area::Area;
    ///
    /// let a = UnOptCode::new(0, 1, 2, (1, 2), Area::Nil, "형..".to_string());
    /// assert_eq!("type: 0, cnt1: 1, cnt2: 2, area: \"_\"", format!("{:?}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut area = String::new();
        area::area_to_string_debug(&mut area, &self.area);
        write!(
            f,
            "type: {}, cnt1: {}, cnt2: {}, area: {:?}",
            self.type_, self.hangul_count, self.dot_count, area
        )
    }
}

impl Code for UnOptCode {
    /// Return type of code
    fn get_type(&self) -> u8 {
        self.type_
    }

    /// Return hangul count of code
    fn get_hangul_count(&self) -> usize {
        self.hangul_count
    }

    /// Return dot count of code
    fn get_dot_count(&self) -> usize {
        self.dot_count
    }

    /// Return Area of code
    fn get_area(&self) -> &Area {
        &self.area
    }

    /// Return area count of code
    fn get_area_count(&self) -> usize {
        self.hangul_count * self.dot_count
    }
}
