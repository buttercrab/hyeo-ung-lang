use crate::hyeong::area::{Area, ExclamationArea, HeartType};
use crate::hyeong::code::{CodeType, UnOptCode};
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::map;
use nom::error::{context, ContextError, ParseError};
use nom::multi::{fold_many0, many0, many_till, separated_list0};
use nom::sequence::{pair, preceded};
use nom::{IResult, Parser};
use nom_locate::LocatedSpan;

const HEARTS: &str = "♥❤💕💖💗💘💙💚💛💜💝💞♡";
const ETC: &str = "?!.…⋯⋮";
const HANGUL: &str = "형항핫흣흡흑혀하흐어아으엉앙앗읏읍윽";

pub type Span<'a> = LocatedSpan<&'a str>;

fn sp<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    take_while(move |c| !HEARTS.contains(c) && !ETC.contains(c) && !HANGUL.contains(c))(i)
}

fn hangul<'a, E: ParseError<Span<'a>> + ContextError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, (CodeType, usize), E> {
    macro_rules! build_hangul {
        ($variant:ident, $one:literal, $long_begin:literal, $long_middle:literal, $long_end:literal) => {{
            alt((
                map(preceded(sp, tag($one)), |_| (CodeType::$variant, 1)),
                map(
                    pair(
                        preceded(sp, tag($long_begin)),
                        many_till(preceded(sp, tag($long_middle)), preceded(sp, tag($long_end))),
                    ),
                    |(_, (b, _))| (CodeType::$variant, b.len() + 2),
                ),
            ))
        }};
    }

    let hyeong = build_hangul!(Hyeong, "형", "혀", "어", "엉");
    let hang = build_hangul!(Hang, "항", "하", "아", "앙");
    let hat = build_hangul!(Hat, "핫", "하", "아", "앗");
    let heut = build_hangul!(Heut, "흣", "흐", "으", "읏");
    let heup = build_hangul!(Heup, "흡", "흐", "으", "읍");
    let heuk = build_hangul!(Heuk, "흑", "흐", "으", "윽");

    context("hangul", alt((hyeong, hang, hat, heut, heup, heuk)))(i)
}

fn dot<'a, E: ParseError<Span<'a>> + ContextError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, usize, E> {
    context(
        "dot",
        fold_many0(
            preceded(
                sp,
                alt((
                    map(tag("."), |_| 1),
                    map(tag("…"), |_| 3),
                    map(tag("⋯"), |_| 3),
                    map(tag("⋮"), |_| 3),
                )),
            ),
            || 0,
            |acc, item| acc + item,
        ),
    )(i)
}

fn area_one<'a, E: ParseError<Span<'a>> + ContextError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Option<HeartType>, E> {
    context(
        "heart",
        alt((
            map(preceded(sp, tag("♥")), |_| Some(HeartType::BlackHeartSuit)),
            map(preceded(sp, tag("❤")), |_| Some(HeartType::BlackHeartSymbol)),
            map(preceded(sp, tag("💕")), |_| Some(HeartType::TwoHearts)),
            map(preceded(sp, tag("💖")), |_| Some(HeartType::SparklingHeart)),
            map(preceded(sp, tag("💗")), |_| Some(HeartType::GrowingHeart)),
            map(preceded(sp, tag("💘")), |_| Some(HeartType::HeartWithArrow)),
            map(preceded(sp, tag("💙")), |_| Some(HeartType::BlueHeart)),
            map(preceded(sp, tag("💚")), |_| Some(HeartType::GreenHeart)),
            map(preceded(sp, tag("💛")), |_| Some(HeartType::YellowHeart)),
            map(preceded(sp, tag("💜")), |_| Some(HeartType::PurpleHeart)),
            map(preceded(sp, tag("💝")), |_| Some(HeartType::HeartWithRibbon)),
            map(preceded(sp, tag("♡")), |_| Some(HeartType::WhiteHeartSuit)),
            map(sp, |_| None),
        )),
    )(i)
}

fn area_exclamation<'a, E: ParseError<Span<'a>> + ContextError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, ExclamationArea, E> {
    context(
        "exclamation area",
        map(
            separated_list0(preceded(sp, tag("!")), preceded(sp, area_one)),
            ExclamationArea::from,
        ),
    )(i)
}

fn area<'a, E: ParseError<Span<'a>> + ContextError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Area, E> {
    context(
        "area",
        map(
            separated_list0(preceded(sp, tag("?")), preceded(sp, area_exclamation)),
            Area::from,
        ),
    )(i)
}

fn code<'a, E: ParseError<Span<'a>> + ContextError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, UnOptCode, E> {
    let (span, (((code_type, code_count), dot_count), area)) = context("code", hangul.and(dot).and(area))(i)?;
    Ok((
        span,
        UnOptCode::new(
            code_type,
            code_count,
            dot_count,
            (span.location_line() as usize, span.naive_get_utf8_column()),
            area,
            span.fragment(),
        ),
    ))
}

pub fn parse<'a, E: ParseError<Span<'a>> + ContextError<Span<'a>>>(
    input: &'a str,
) -> Result<Vec<UnOptCode>, nom::Err<E>> {
    Ok(context("code", many0(code))(Span::new(input))?.1)
}

#[cfg(test)]
mod test {
    use crate::hyeong::area::HeartType;
    use crate::hyeong::code::CodeType;
    use crate::hyeong::parse::{area, area_exclamation, area_one, dot, hangul, Span};
    use nom::error::ErrorKind;

    #[test]
    fn hangul_test() {
        assert_eq!(
            hangul::<(Span, ErrorKind)>(Span::new("형")).unwrap().1,
            (CodeType::Hyeong, 1)
        );
        assert_eq!(
            hangul::<(Span, ErrorKind)>(Span::new("혀엉")).unwrap().1,
            (CodeType::Hyeong, 2)
        );
        assert_eq!(
            hangul::<(Span, ErrorKind)>(Span::new("혀어엉")).unwrap().1,
            (CodeType::Hyeong, 3)
        );
        assert_eq!(
            hangul::<(Span, ErrorKind)>(Span::new("혀 어 어   엉")).unwrap().1,
            (CodeType::Hyeong, 4)
        );
    }

    #[test]
    fn dot_test() {
        assert_eq!(dot::<(Span, ErrorKind)>(Span::new("")).unwrap().1, 0);
        assert_eq!(dot::<(Span, ErrorKind)>(Span::new(".. . .")).unwrap().1, 4);
        assert_eq!(dot::<(Span, ErrorKind)>(Span::new(" . .....")).unwrap().1, 6);
        assert_eq!(dot::<(Span, ErrorKind)>(Span::new("asdf....")).unwrap().1, 4);
    }

    #[test]
    fn area_one_test() {
        assert_eq!(area_one::<(Span, ErrorKind)>(Span::new("")).unwrap().1, None);
        assert_eq!(
            area_one::<(Span, ErrorKind)>(Span::new("   💕")).unwrap().1,
            Some(HeartType::TwoHearts)
        );
        assert_eq!(
            area_one::<(Span, ErrorKind)>(Span::new(" 💝 ")).unwrap().1,
            Some(HeartType::HeartWithRibbon)
        );
        assert_eq!(
            area_one::<(Span, ErrorKind)>(Span::new(" 형 ...")).unwrap().1,
            None
        );
    }

    #[test]
    fn area_exclamation_test() {
        assert_eq!(
            area_exclamation::<(Span, ErrorKind)>(Span::new("")).unwrap().1,
            vec![None].into()
        );
        assert_eq!(
            area_exclamation::<(Span, ErrorKind)>(Span::new("!")).unwrap().1,
            vec![None, None].into()
        );
        assert_eq!(
            area_exclamation::<(Span, ErrorKind)>(Span::new("  ! !  "))
                .unwrap()
                .1,
            vec![None, None, None].into()
        );
        assert_eq!(
            area_exclamation::<(Span, ErrorKind)>(Span::new("💕!💕"))
                .unwrap()
                .1,
            vec![Some(HeartType::TwoHearts), Some(HeartType::TwoHearts)].into()
        );
        assert_eq!(
            area_exclamation::<(Span, ErrorKind)>(Span::new("💕!  💕 !"))
                .unwrap()
                .1,
            vec![Some(HeartType::TwoHearts), Some(HeartType::TwoHearts), None].into()
        );
    }

    #[test]
    fn area_test() {
        assert_eq!(
            area::<(Span, ErrorKind)>(Span::new("")).unwrap().1,
            vec![vec![None].into()].into()
        );
        assert_eq!(
            area::<(Span, ErrorKind)>(Span::new("?")).unwrap().1,
            vec![vec![None].into(), vec![None].into()].into()
        );
        assert_eq!(
            area::<(Span, ErrorKind)>(Span::new("  ! ?  ")).unwrap().1,
            vec![vec![None, None].into(), vec![None].into()].into()
        );
        assert_eq!(
            area::<(Span, ErrorKind)>(Span::new("💕!💕")).unwrap().1,
            vec![vec![Some(HeartType::TwoHearts), Some(HeartType::TwoHearts)].into()].into()
        );
        assert_eq!(
            area::<(Span, ErrorKind)>(Span::new("💕? ?  💕 ")).unwrap().1,
            vec![
                vec![Some(HeartType::TwoHearts)].into(),
                vec![None].into(),
                vec![Some(HeartType::TwoHearts)].into()
            ]
            .into()
        );
    }
}
