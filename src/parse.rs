use crate::code;

pub(crate) const COMMANDS: &'static [char] = &['형', '항', '핫', '흣', '흡', '흑'];
const HEARTS: &'static [char] = &['♥', '❤', '💕', '💖', '💗', '💘', '💙', '💚', '💛', '💜', '💝', '♡'];

/// Check if the character is hangul
///
/// # Example
///
/// ```
/// use hyeong::parse;
///
/// assert_eq!(true, parse::is_hangul_syllable('가'));
/// assert_eq!(true, parse::is_hangul_syllable('힣'));
/// assert_eq!(false, parse::is_hangul_syllable('a'));
/// assert_eq!(false, parse::is_hangul_syllable('م'));
/// assert_eq!(false, parse::is_hangul_syllable('ý'));
/// assert_eq!(false, parse::is_hangul_syllable('ם'));
/// assert_eq!(false, parse::is_hangul_syllable('न'));
/// assert_eq!(false, parse::is_hangul_syllable('こ'));
/// assert_eq!(false, parse::is_hangul_syllable('你'));
/// assert_eq!(false, parse::is_hangul_syllable('д'));
/// ```
pub fn is_hangul_syllable(c: char) -> bool {
    '\u{AC00}' <= c && c <= '\u{D7A3}'
}

/// Parse the code to unoptimized code
/// Since the language itself has no compile error, it never returns error.
///
/// # State
///
/// This parsing algorithm is made with state.
/// - `0`: 
/// - `1`:
/// - `2`:
///
/// # Time Complexity
///
/// - `O(n)` where `n := code.len()`
/// - Iterates only twice: once for main loop, once for checking if the character is valid.
///
/// # Example
///
/// ```
/// use hyeong::parse;
///
/// let parsed = parse::parse("형...?💖?".to_string());
///
/// assert_eq!("type: 0, cnt1: 1, cnt2: 3, area: \"?_?💖_\"", format!("{:?}", parsed[0]));
/// ```
pub fn parse(code: String) -> Vec<code::UnOptCode> {
    let mut res: Vec<code::UnOptCode> = Vec::new();

    let mut hangul_count = 0usize;
    let mut dot_count = 0usize;
    let mut type_ = 10u8;
    let mut loc = (0usize, 0usize);

    // 0: can come: hangul, dot, area
    // 1: can come: hangul (after hangul starts)
    // 2: can come: hangul, area (after area starts)
    let mut state = 0u8;
    let mut area = code::Area::Nil;
    let mut leaf = &mut area;
    let mut qu_area = code::Area::Nil;
    let mut qu_leaf = &mut qu_area;

    let mut line_count = 0;
    let mut last_line_started = 0;

    let mut max_pos = [0usize, 0usize, 0usize];
    for (i, c) in code.chars().enumerate() {
        if let Some(t) = "엉앙앗읏읍윽".find(c) {
            max_pos[if t == 0 {
                0
            } else if t <= 6 {
                1
            } else {
                2
            }] = i;
        }
    }

    for (i, c) in code.chars().enumerate() {
        if c.is_whitespace() {
            if c == '\n' {
                line_count += 1;
                last_line_started = i + 1;
            }
            continue;
        }

        state = match state {
            0 | 2 => if let Some(mut t) = "형항핫흣흡흑혀하흐".find(c) {
                t /= 3;

                if t >= 6 && max_pos[t - 6] <= i {
                    continue;
                }

                if type_ != 10 {
                    res.push(code::UnOptCode::new(
                        type_,
                        hangul_count,
                        dot_count,
                        loc,
                        match qu_leaf {
                            code::Area::Val {
                                type_: _,
                                left: _,
                                ref mut right,
                            } => {
                                *right = Box::new(area);
                                qu_area
                            }
                            code::Area::Nil => {
                                area
                            }
                        },
                    ));

                    area = code::Area::Nil;
                    leaf = &mut area;
                    qu_area = code::Area::Nil;
                    qu_leaf = &mut qu_area;
                }

                type_ = t as u8;
                hangul_count = 1;
                dot_count = 0;
                loc = (line_count, i - last_line_started);

                if t < 6 {
                    0
                } else {
                    1
                }
            } else if ".…⋯⋮".contains(c) {
                if state == 0 {
                    dot_count += if c == '.' { 1 } else { 3 };
                }
                state
            } else if c == '?' {
                match qu_leaf {
                    code::Area::Val {
                        type_: _,
                        left: _,
                        ref mut right,
                    } => {
                        *right = Box::new(code::Area::Val {
                            type_: 0,
                            left: Box::new(area),
                            right: Box::new(code::Area::Nil),
                        });
                        qu_leaf = &mut *right;
                    }

                    code::Area::Nil => {
                        qu_area = code::Area::Val {
                            type_: 0,
                            left: Box::new(area),
                            right: Box::new(code::Area::Nil),
                        };
                        qu_leaf = &mut qu_area;
                    }
                }

                area = code::Area::Nil;
                leaf = &mut area;
                2
            } else if c == '!' {
                match leaf {
                    code::Area::Val {
                        ref type_,
                        left: _,
                        ref mut right,
                    } => if *type_ <= 1 {
                        *right = match right.as_ref() {
                            code::Area::Val {
                                type_: t,
                                left: _,
                                right: _,
                            } => Box::new(code::Area::Val {
                                type_: 1,
                                left: Box::new(code::Area::new(*t)),
                                right: Box::new(code::Area::Nil),
                            }),
                            code::Area::Nil => Box::new(code::Area::new(1)),
                        };
                        leaf = &mut *right;
                    } else {
                        area = code::Area::Val {
                            type_: 1,
                            left: Box::new(code::Area::new(*type_)),
                            right: Box::new(code::Area::Nil),
                        };
                        leaf = &mut area;
                    },
                    code::Area::Nil => {
                        area = code::Area::new(1);
                        leaf = &mut area;
                    }
                }
                2
            } else if let Some(mut t) = HEARTS.iter().position(|&x| x == c) {
                t += 2;
                match leaf {
                    code::Area::Val {
                        ref type_,
                        left: _,
                        ref mut right,
                    } => if *type_ <= 1 {
                        match right.as_ref() {
                            code::Area::Nil => {
                                *right = Box::new(code::Area::new(t as u8));
                            }
                            _ => {}
                        }
                    },
                    code::Area::Nil => {
                        area = code::Area::new(t as u8);
                        leaf = &mut area;
                    }
                }
                2
            } else {
                continue;
            }

            1 => {
                if is_hangul_syllable(c) {
                    hangul_count += 1;
                }
                match type_ {
                    6 => if "엉".contains(c) {
                        type_ = 0;
                        dot_count = 0;
                        0
                    } else {
                        1
                    }

                    7 => if let Some(t) = "앙앗".find(c) {
                        type_ = (t / 3 + 1) as u8;
                        dot_count = 0;
                        0
                    } else {
                        1
                    }

                    8 => if let Some(t) = "읏읍윽".find(c) {
                        type_ = (t / 3 + 3) as u8;
                        dot_count = 0;
                        0
                    } else {
                        1
                    }

                    _ => unreachable!()
                }
            }

            _ => unreachable!()
        };
    }

    if type_ != 10 {
        res.push(code::UnOptCode::new(
            type_,
            hangul_count,
            dot_count,
            loc,
            match qu_leaf {
                code::Area::Val {
                    type_: _,
                    left: _,
                    ref mut right,
                } => {
                    *right = Box::new(area);
                    qu_area
                }
                code::Area::Nil => {
                    area
                }
            },
        ));
    }
    res
}
