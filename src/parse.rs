use std::error::Error;
use std::fmt;

use colored::Colorize;

pub struct ParseError {
    no: u8,
    line: usize,
    location: usize,
    content: String,
}

impl ParseError {
    pub fn new(no: u8, line: usize, location: usize) -> ParseError {
        match no {
            0x1 => ParseError {
                no,
                line,
                location,
                content: "Not right character".to_string(),
            },
            0x2 => ParseError {
                no,
                line,
                location,
                content: "Last command didn't finish".to_string(),
            },
            _ => ParseError {
                no,
                line,
                location,
                content: "Error occurred in compiler: make an issue".to_string(),
            }
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error[{:04X}] {}:{}:{}", self.no, self.line, self.location, self.content)
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error[{:04X}] {}:{}:{}", self.no, self.line, self.location, self.content)
    }
}

pub struct Command {
    // 0: 형, 혀엉, 혀어엉, 혀어어엉 ...
    // 1: 항, 하앙, 하아앙, 하아아앙 ...
    // 2: 핫, 하앗, 하아앗, 하아아앗 ...
    // 3: 흣, 흐읏, 흐으읏, 흐으으읏 ...
    // 4: 흡, 흐읍, 흐으읍, 흐으으읍 ...
    // 5: 흑, 흐윽, 흐으윽, 흐으으윽 ...
    type_: u8,
    cnt1: u128,
    cnt2: u128,
    line: usize,
    loc: usize,
    area: Area,
}


impl Command {
    pub fn new(type_: u8) -> Command {
        Command {
            type_,
            cnt1: 0,
            cnt2: 0,
            line: 0,
            loc: 0,
            area: Area::Nil,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}   {}_{}_{} : {}",
                (&*format!("{}:{}", self.line, self.loc)).yellow()
                , COMMANDS[self.type_ as usize], self.cnt1, self.cnt2, self.area)
    }
}

impl fmt::Display for Command {
    // for debug
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut area = String::new();
        area_to_string(&mut area, &self.area, true);
        write!(f, "type: {}, cnt1: {}, cnt2: {}, area: {}", self.type_, self.cnt1, self.cnt2, area)
    }
}

const COMMANDS: &'static [char] = &['형', '항', '핫', '흣', '흡', '흑'];
const HEARTS: &'static [char] = &['♥', '❤', '💕', '💖', '💗', '💘', '💙', '💚', '💛', '💜', '💝', '♡'];

fn is_hangul_syllable(c: char) -> bool {
    '\u{AC00}' <= c && c <= '\u{D7A3}'
}

pub fn parse(code: String) -> Result<Vec<Command>, ParseError> {
    let mut res: Vec<Command> = Vec::new();

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
    let mut command = Command::new(0);

    // 0: can come: hangul, dot, area
    // 1: can come: hangul (after hangul starts)
    // 2: can come: hangul, area (after area starts)
    let mut state = 0u8;
    let mut area = Area::Nil;
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
                        Area::Val {
                            type_: _,
                            left: _,
                            ref mut right,
                        } => {
                            *right = Box::new(area);
                        }
                        Area::Nil => {
                            command.area = area;
                        }
                    }
                    area = Area::Nil;
                    leaf = &mut area;
                    res.push(command);
                    command = Command::new(t as u8);
                } else {
                    command.type_ = t as u8;
                }

                command.cnt1 = 1;
                command.cnt2 = 0;
                command.area = Area::Nil;
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
                    Area::Val {
                        type_: _,
                        left: _,
                        ref mut right,
                    } => {
                        *right = Box::new(Area::Val {
                            type_: 0,
                            left: Box::new(area),
                            right: Box::new(Area::Nil),
                        });
                        cmd_leaf = &mut *right;
                    }
                    Area::Nil => {
                        command.area = Area::Val {
                            type_: 0,
                            left: Box::new(area),
                            right: Box::new(Area::Nil),
                        };
                        cmd_leaf = &mut command.area;
                    }
                }
                area = Area::Nil;
                leaf = &mut area;
                2
            } else if c == '!' {
                match leaf {
                    Area::Val {
                        ref type_,
                        left: _,
                        ref mut right,
                    } => if *type_ <= 1 {
                        *right = match right.as_ref() {
                            Area::Val {
                                type_: t,
                                left: _,
                                right: _,
                            } => Box::new(Area::Val {
                                type_: 1,
                                left: Box::new(Area::new(*t)),
                                right: Box::new(Area::Nil),
                            }),
                            Area::Nil => Box::new(Area::new(1)),
                        };
                        leaf = &mut *right;
                    } else {
                        area = Area::Val {
                            type_: 1,
                            left: Box::new(Area::new(*type_)),
                            right: Box::new(Area::Nil),
                        };
                        leaf = &mut area;
                    },
                    Area::Nil => {
                        area = Area::new(1);
                        leaf = &mut area;
                    }
                }
                2
            } else if let Some(mut t) = HEARTS.iter().position(|&x| x == c) {
                t += 2;
                match leaf {
                    Area::Val {
                        ref type_,
                        left: _,
                        ref mut right,
                    } => if *type_ <= 1 {
                        match right.as_ref() {
                            Area::Nil => {
                                *right = Box::new(Area::new(t as u8));
                            }
                            _ => {}
                        }
                    },
                    Area::Nil => {
                        area = Area::new(t as u8);
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

            _ => return Result::Err(ParseError::new(0x10, line_count, i - last_line_started))
        };
    }

    if state == 1 {
        Result::Err(ParseError::new(0x2, line_count, code.len() - last_line_started))
    } else {
        match cmd_leaf {
            Area::Val {
                type_: _,
                left: _,
                ref mut right,
            } => {
                *right = Box::new(area);
            }
            Area::Nil => {
                command.area = area;
            }
        }
        res.push(command);
        Result::Ok(res)
    }
}

fn area_to_string(s: &mut String, area: &Area, need: bool) {
    match area {
        Area::Val {
            ref type_,
            ref left,
            ref right
        } => {
            let c = "?!♥❤💕💖💗💘💙💚💛💜💝♡".chars().collect::<Vec<char>>()[*type_ as usize];
            s.push(c);
            area_to_string(s, left, c == '?' || c == '!');
            area_to_string(s, right, c == '?' || c == '!');
        }
        Area::Nil => {
            if need {
                s.push('_')
            }
        }
    }
}


pub enum Area {
    //  0: ?
    //  1: !
    //  2: ♥
    //  3: ❤
    //  4: 💕
    //  5: 💖
    //  6: 💗
    //  7: 💘
    //  8: 💙
    //  9: 💚
    // 10: 💛
    // 11: 💜
    // 12: 💝
    // 13: ♡
    Val {
        type_: u8,
        left: Box<Area>,
        right: Box<Area>,
    },
    Nil,
}

impl Area {
    pub fn new(type_: u8) -> Area {
        Area::Val {
            type_,
            left: Box::new(Area::Nil),
            right: Box::new(Area::Nil),
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        area_to_string(&mut s, self, false);
        if s.is_empty() {
            s = "(no area)".to_string();
        }
        write!(f, "{}", s)
    }
}