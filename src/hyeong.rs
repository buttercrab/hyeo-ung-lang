pub mod area;
pub mod code;
pub mod execute;
pub mod optimize;
pub mod parse;
pub mod state;

use anyhow::{anyhow, Result};
use number::num::Num;

/// change `Num` to unicode char
///
/// # Examples
///
/// ```
/// use hyeong::number::num::Num;
/// use hyeong::util::ext;
///
/// let a = Num::from_num(55357);
/// let b = Num::from_num(0xAC00);
///
/// assert!(ext::num_to_unicode(&a).is_err());
/// assert!(matches!(ext::num_to_unicode(&b), Ok('가')));
/// ```
pub fn num_to_unicode(num: &Num) -> Result<char> {
    let n = num.floor().to_int();
    std::char::from_u32(n).ok_or_else(|| anyhow!("utf-8 encoding error: number {} is not valid unicode", n))
}
