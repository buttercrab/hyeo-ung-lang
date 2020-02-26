use std::collections::HashMap;

pub struct Command {
    // 0: 형, 혀엉, 혀어엉, 혀어어엉 ...
    // 1: 항, 하앙, 하아앙, 하아아앙 ...
    // 2: 핫, 하앗, 하아앗, 하아아앗 ...
    // 3: 흣, 흐읏, 흐으읏, 흐으으읏 ...
    // 4: 흡, 흐읍, 흐으읍, 흐으으읍 ...
    // 5: 흑, 흐윽, 흐으윽, 흐으으윽 ...
    type_: u8,
    length: usize,
    area: Area,
}

pub struct Error {
    type_: u8,
    location: usize,
    content: String,
}

impl Command {
    pub fn new(code: String) -> Result<Vec<Command>, Error> {
        let mut res: Vec<Command> = Vec::new();

        // 0: 형
        // 1: 항
        // 2: 핫
        // 3: 흣
        // 4: 흡
        // 5: 흑
        // 6: 혀
        // 7: 하
        // 8: 흐
        let mut type_ = 0u8;
        let mut cnt1 = 0u128;
        let mut cnt2 = 0u128;

        // 0: before hangul begin
        // 1: hangul
        // 2: before dot begin
        // 3: dot
        // 4: before area begin
        // 5: area
        let mut state = 0u8;
        let mut area = Area::Nil;
        let mut parent_area = Area::Nil;

        for (i, c) in code.chars().enumerate() {
            if c.is_whitespace() { continue; }

            state = match state {
                0 => {
                    cnt1 = 1;
                    if let Some(t) = "형항핫흣흡흑".find(c) {
                        type_ = t as u8;
                        2
                    } else if let Some(t) = "혀하흐".find(c) {
                        type_ = (t as u8) + 6;
                        1
                    } else {
                        return Result::Err(Error {
                            type_: 0,
                            location: i,
                            content: "not supporting character appeared".to_string(),
                        });
                    }
                }
                1 => {}
                2 => {}
                3 => {}
                4 => {}
                5 => {}
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
    Cons(u8, Box<Area>, Box<Area>),
    Nil,
}

pub struct State {
    length: usize,
    type_: u8,
}
