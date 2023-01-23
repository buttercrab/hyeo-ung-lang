use anyhow::Result;
use derive_more::{Display, From};
use number::num::Num;
use std::cmp::Ordering;
use std::fmt;
use std::ops::ControlFlow;

pub type Area = QuestionArea;

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
// pub enum SplitType {
//     #[display(fmt = "?")]
//     QuestionMark,
//     #[display(fmt = "!")]
//     ExclamationMark,
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
pub enum HeartType {
    #[display(fmt = "♥")]
    BlackHeartSuit,
    #[display(fmt = "❤")]
    BlackHeartSymbol,
    #[display(fmt = "💕")]
    TwoHearts,
    #[display(fmt = "💖")]
    SparklingHeart,
    #[display(fmt = "💗")]
    GrowingHeart,
    #[display(fmt = "💘")]
    HeartWithArrow,
    #[display(fmt = "💙")]
    BlueHeart,
    #[display(fmt = "💚")]
    GreenHeart,
    #[display(fmt = "💛")]
    YellowHeart,
    #[display(fmt = "💜")]
    PurpleHeart,
    #[display(fmt = "💝")]
    HeartWithRibbon,
    #[display(fmt = "♡")]
    WhiteHeartSuit,
}

// vector size >= 1
#[derive(Debug, Clone, Eq, PartialEq, From)]
pub struct ExclamationArea(Vec<Option<HeartType>>);

impl ExclamationArea {
    pub fn calc<F>(&self, area_size: usize, pop: &mut F) -> Result<Option<HeartType>>
    where
        F: FnMut() -> Result<Num>,
    {
        let mut it = self.0.iter();
        let init = *it.next().unwrap();

        match it.copied().try_fold(init, |ret, heart| match pop() {
            Ok(p) => {
                if matches!(
                    p.partial_cmp(&Num::from_num(area_size as isize)),
                    Some(Ordering::Equal)
                ) {
                    ControlFlow::Break(Ok(ret))
                } else {
                    ControlFlow::Continue(heart)
                }
            }
            Err(e) => ControlFlow::Break(Err(e)),
        }) {
            ControlFlow::Continue(ret) => Ok(ret),
            ControlFlow::Break(ret) => ret,
        }
    }
}

impl fmt::Display for ExclamationArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(x) = self.0.first().unwrap() {
            write!(f, "[{x}]")?;
        } else {
            write!(f, "[_]")?;
        }

        self.0.iter().skip(1).try_for_each(|x| {
            if let Some(x) = x {
                write!(f, "![{x}]")
            } else {
                write!(f, "![_]")
            }
        })?;

        Ok(())
    }
}

// vector size >= 1
#[derive(Debug, Clone, Eq, PartialEq, From)]
pub struct QuestionArea(Vec<ExclamationArea>);

impl QuestionArea {
    pub fn calc<F>(&self, area_size: usize, mut pop: F) -> Result<Option<HeartType>>
    where
        F: FnMut() -> Result<Num>,
    {
        let mut it = self.0.iter();
        let init = it.next().unwrap();

        match it.try_fold(init, |ret, heart| match pop() {
            Ok(p) => {
                if matches!(
                    p.partial_cmp(&Num::from_num(area_size as isize)),
                    Some(Ordering::Equal)
                ) {
                    ControlFlow::Break(ret.calc(area_size, &mut pop))
                } else {
                    ControlFlow::Continue(heart)
                }
            }
            Err(e) => ControlFlow::Break(Err(e)),
        }) {
            ControlFlow::Continue(ret) => ret.calc(area_size, &mut pop),
            ControlFlow::Break(ret) => ret,
        }
    }
}

impl fmt::Display for QuestionArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.first().unwrap())?;
        self.0.iter().skip(1).try_for_each(|x| write!(f, "?[{x}]"))?;

        Ok(())
    }
}
