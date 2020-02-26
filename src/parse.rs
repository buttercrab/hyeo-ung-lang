pub struct Command {
    // 0: 형, 혀엉, 혀어엉, 혀어어엉
    // 1: 항, 하앙, 하아앙, 하아아앙
    // 2: 핫, 하앗, 하아앗, 하아아앗
    // 3: 흣, 흐읏 , 흐으읏, 흐으으읏흣
    // 4: 흡, 흐읍, 흐으읍, 흐으으읍
    // 5: 흑, 흐윽, 흐으윽, 흐으으윽
    type_: u8,
    length: u128,
    area: Area,
}

pub struct Area {
    // 1 : ?
    // 2 : !
    // 3 : ♥
    // 4 : ❤
    // 5 : 💕
    // 6 : 💖
    // 7 : 💗
    // 8 : 💘
    // 9 : 💙
    // 10: 💚
    // 11: 💛
    // 12: 💜
    // 13: 💝
    type_: u8,
    left: Box<Area>,
    right: Box<Area>,
}

pub struct Register {
    length: u128,
    type_: u8,
}

pub struct Code {
    code: Vec<Command>,
    register: Vec<Register>,
}

impl Code {
    // pub fn new(code: String) -> Code {}
}
