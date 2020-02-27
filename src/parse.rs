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
    area: Area,
}

pub struct Error {
    type_: u8,
    location: usize,
    content: String,
}

impl Command {
    pub fn new(type_: u8) -> Command {
        Command {
            type_,
            cnt1: 0,
            cnt2: 0,
            area: Area::Nil,
        }
    }

    pub fn parse(code: String) -> Result<Vec<Command>, Error> {
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
        let mut command = Command {
            type_: 0,
            cnt1: 0,
            cnt2: 0,
            area: Area::Nil,
        };

        // 0: before hangul begins && '?'|'!'
        // 1: hangul
        // 2: before area begins (dot)
        // 3: area (heart only)
        let mut state = 0u8;
        let mut area = Area::Nil;
        let mut temp_area = Area::Nil;
        let mut leaf = &mut area;

        for (i, c) in code.chars().enumerate() {
            if c.is_whitespace() { continue; }

            state = match state {
                0 => if let Some(t) = "형항핫흣흡흑".find(c) {
                    if command.cnt1 != 0 {
                        leaf = &mut temp_area;
                        if let Area::Nil = area {
                            match command.area {
                                Area::Val {
                                    ref type_, ref left, ref mut right
                                } => {
                                    *right = Box::new(area);
                                }
                                Area::Nil => {
                                    command.area = area;
                                }
                            }
                            area = Area::Nil;
                            leaf = &mut area;
                        }
                        res.push(command);
                    }
                    command = Command::new(0);
                    command.cnt1 = 1;
                    command.type_ = t as u8;
                    2
                } else if let Some(t) = "혀하흐".find(c) {
                    if command.cnt1 != 0 {
                        leaf = &mut temp_area;
                        if let Area::Nil = area {
                            match command.area {
                                Area::Val {
                                    ref type_, ref left, ref mut right
                                } => {
                                    *right = Box::new(area);
                                }
                                Area::Nil => {
                                    command.area = area;
                                }
                            }
                            area = Area::Nil;
                            leaf = &mut area;
                        }
                        res.push(command);
                    }
                    command = Command::new(0);
                    command.cnt1 = 1;
                    command.type_ = (t as u8) + 6;
                    1
                } else if c == '?' {
                    leaf = &mut temp_area;
                    match command.area {
                        Area::Val {
                            ref type_, ref left, ref mut right
                        } => {
                            *right = Box::new(Area::Val {
                                type_: 0,
                                left: Box::new(area),
                                right: Box::new(Area::Nil),
                            });
                        }
                        Area::Nil => {
                            command.area = *Box::new(Area::Val {
                                type_: 0,
                                left: Box::new(area),
                                right: Box::new(Area::Nil),
                            });
                        }
                    };

                    area = Area::Nil;
                    leaf = &mut area;
                    3
                } else if c == '!' {
                    match leaf {
                        Area::Val {
                            type_: _, left: _, ref mut right
                        } => match right.as_ref() {
                            Area::Val {
                                type_: t,
                                left: _,
                                right: _,
                            } => {
                                *right = Box::new(Area::Val {
                                    type_: 1,
                                    left: Box::new(Area::new(*t)),
                                    right: Box::new(Area::Nil),
                                });
                            }
                            _ => return Result::Err(Error {
                                type_: 0,
                                location: i,
                                content: "not supporting character appeared".to_string(),
                            })
                        },
                        _ => return Result::Err(Error {
                            type_: 0,
                            location: i,
                            content: "not supporting character appeared".to_string(),
                        })
                    }
                    3
                } else {
                    return Result::Err(Error {
                        type_: 0,
                        location: i,
                        content: "not supporting character appeared".to_string(),
                    });
                },

                1 => match command.type_ {
                    6 => {
                        match c {
                            '어' => {
                                command.cnt1 += 1;
                                1
                            }

                            '엉' => {
                                command.cnt1 += 1;
                                command.type_ = 0;
                                command.cnt2 = 0;
                                2
                            }

                            _ => return Result::Err(Error {
                                type_: 0,
                                location: i,
                                content: "not supporting character appeared".to_string(),
                            })
                        }
                    }

                    7 => {
                        match c {
                            '아' => {
                                command.cnt1 += 1;
                                1
                            }

                            '앙' => {
                                command.cnt1 += 1;
                                command.type_ = 1;
                                command.cnt2 = 0;
                                2
                            }

                            '앗' => {
                                command.cnt1 += 1;
                                command.type_ = 2;
                                command.cnt2 = 0;
                                2
                            }

                            _ => return Result::Err(Error {
                                type_: 0,
                                location: i,
                                content: "not supporting character appeared".to_string(),
                            })
                        }
                    }

                    8 => {
                        match c {
                            '으' => {
                                command.cnt1 += 1;
                                1
                            }

                            '읏' => {
                                command.cnt1 += 1;
                                command.type_ = 3;
                                command.cnt2 = 0;
                                2
                            }

                            '읍' => {
                                command.cnt1 += 1;
                                command.type_ = 4;
                                command.cnt2 = 0;
                                2
                            }

                            '윽' => {
                                command.cnt1 += 1;
                                command.type_ = 5;
                                command.cnt2 = 0;
                                2
                            }

                            _ => return Result::Err(Error {
                                type_: 0,
                                location: i,
                                content: "not supporting character appeared".to_string(),
                            })
                        }
                    }

                    _ => return Result::Err(Error {
                        type_: u8::max_value(),
                        location: 0,
                        content: "error in compiler: make an issue".to_string(),
                    })
                },

                2 => if c == '.' {
                    command.cnt2 += 1;
                    2
                } else if let Some(t) = "♥❤💕💖💗💘💙💚💛💜💝".find(c) {
                    area = Area::new(t as u8);
                    leaf = &mut area;
                    0
                } else {
                    return Result::Err(Error {
                        type_: 0,
                        location: i,
                        content: "not supporting character appeared".to_string(),
                    });
                },

                3 => if let Some(t) = "♥❤💕💖💗💘💙💚💛💜💝".find(c) {
                    match leaf {
                        Area::Val {
                            ref type_, ref left, ref mut right
                        } => {
                            *right = Box::new(Area::new(t as u8));
                        }
                        _ => {
                            area = Area::new(t as u8);
                            leaf = &mut area;
                        }
                    };
                    0
                } else {
                    return Result::Err(Error {
                        type_: 0,
                        location: i,
                        content: "not supporting character appeared".to_string(),
                    });
                },

                _ => 0
            }
        }
        Result::Ok(res)
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

pub struct State {
    length: usize,
    type_: u8,
}
