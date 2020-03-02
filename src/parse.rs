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

pub fn parse(code: String) -> Vec<code::UnOptCode> {
    let mut res: Vec<code::UnOptCode> = Vec::new();

    // command.type_:
    // 0: 형
    // 1: 항
    // 2: 핫
    // 3: 흣
    // 4: 흡
    // 5: 흑
    // 6: 혀
    // 7: 하
    // 8: 흐
    let mut command = code::UnOptCode::new(0);

    // 0: can come: hangul, dot, area
    // 1: can come: hangul (after hangul starts)
    // 2: can come: hangul, area (after area starts)
    let mut state = 0u8;
    let mut area = code::Area::Nil;
    let mut leaf = &mut area;
    let mut cmd_leaf = &mut command.area;

    let mut line_count = 0;
    let mut last_line_started = 0;

    let mut sum = [0usize, 0usize, 0usize, 0usize, 0usize, 0usize];
    let mut partial_sum = code.chars().map(|x| {
        if let Some(t) = "엉앙앗읏읍윽".find(x) {
            sum[t / 3] = 1;
        }
        sum
    }).rev().collect::<Vec<[usize; 6]>>();
    partial_sum.reverse();

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

                if t == 6 {
                    if partial_sum[i][0] == 0 {
                        continue;
                    }
                } else if t == 7 {
                    if partial_sum[i][1..3].iter().sum::<usize>() == 0 {
                        continue;
                    }
                } else if t == 8 {
                    if partial_sum[i][3..6].iter().sum::<usize>() == 0 {
                        continue;
                    }
                }

                if command.cnt1 != 0 {
                    match cmd_leaf {
                        code::Area::Val {
                            type_: _,
                            left: _,
                            ref mut right,
                        } => {
                            *right = Box::new(area);
                        }
                        code::Area::Nil => {
                            command.area = area;
                        }
                    }
                    area = code::Area::Nil;
                    leaf = &mut area;
                    res.push(command);
                    command = code::UnOptCode::new(t as u8);
                } else {
                    command.type_ = t as u8;
                }

                command.cnt1 = 1;
                command.cnt2 = 0;
                command.area = code::Area::Nil;
                command.line = line_count + 1;
                command.loc = i - last_line_started;
                cmd_leaf = &mut command.area;

                if t < 6 {
                    0
                } else {
                    1
                }
            } else if ".…⋯⋮".contains(c) {
                if state == 0 {
                    command.cnt2 += if c == '.' { 1 } else { 3 }
                }
                state
            } else if c == '?' {
                match cmd_leaf {
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
                        cmd_leaf = &mut *right;
                    }
                    code::Area::Nil => {
                        command.area = code::Area::Val {
                            type_: 0,
                            left: Box::new(area),
                            right: Box::new(code::Area::Nil),
                        };
                        cmd_leaf = &mut command.area;
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
                    command.cnt1 += 1;
                }
                match command.type_ {
                    6 => if "엉".contains(c) {
                        command.type_ = 0;
                        command.cnt2 = 0;
                        0
                    } else {
                        1
                    }

                    7 => if let Some(t) = "앙앗".find(c) {
                        command.type_ = (t / 3 + 1) as u8;
                        command.cnt2 = 0;
                        0
                    } else {
                        1
                    }

                    8 => if let Some(t) = "읏읍윽".find(c) {
                        command.type_ = (t / 3 + 3) as u8;
                        command.cnt2 = 0;
                        0
                    } else {
                        1
                    }

                    _ => 1
                }
            }

            _ => unreachable!()
        };
    }

    match cmd_leaf {
        code::Area::Val {
            type_: _,
            left: _,
            ref mut right,
        } => {
            *right = Box::new(area);
        }
        code::Area::Nil => {
            command.area = area;
        }
    }
    res.push(command);
    res
}
