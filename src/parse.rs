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
    no: u8,
    location: usize,
    content: String,
}

impl Error {
    pub fn get_error(&self) -> String {
        format!("Error no {} Line {}: {}", self.no, self.location, self.content)
    }

    pub fn new(type_: u8, location: usize) -> Error {
        match type_ {
            1 => Error {
                no: type_,
                location,
                content: "Not right character".to_string(),
            },
            2 => Error {
                no: type_,
                location,
                content: "Last command didn't finish".to_string(),
            },
            _ => Error {
                no: type_,
                location,
                content: "Error occurred in compiler: make an issue".to_string(),
            }
        }
    }
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

    fn is_hangul_syllable(c: char) -> bool {
        '\u{AC00}' <= c && c <= '\u{D7A3}'
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

        // 0: can come: hangul, dot, area
        // 1: can come: hangul (after hangul starts)
        // 2: can come: hangul, area (after area starts)
        let mut state = 0u8;
        let mut area = Area::Nil;
        let mut temp_area = Area::Nil;
        let mut leaf = &mut area;
        let mut cmd_leaf = &mut command.area;

        for (i, c) in code.chars().enumerate() {
            if c.is_whitespace() { continue; }

            state = match state {
                0 | 2 => if let Some(mut t) = "형항핫흣흡흑혀하흐".find(c) {
                    t /= 3;

                    if command.cnt1 != 0 {
                        res.push(command);
                        command = Command::new(t as u8);
                    } else {
                        command.type_ = t as u8;
                    }

                    command.cnt1 = 1;
                    command.cnt2 = 0;
                    command.area = Area::Nil;

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
                    2
                } else if c == '!' {
                    2
                } else if let Some(t) = "♥❤💕💖💗💘💙💚💛💜💝♡".find(c) {
                    2
                } else {
                    continue;
                }

                1 => {
                    if Command::is_hangul_syllable(c) {
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

                _ => return Result::Err(Error::new(100, i))
            };
        }

        if state == 1 {
            Result::Err(Error::new(2, code.len()))
        } else {
            Result::Ok(res)
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

pub struct State {
    length: usize,
    type_: u8,
}
