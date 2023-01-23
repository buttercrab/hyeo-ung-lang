use crate::hyeong::code::{Code, CodeType};
use crate::hyeong::num_to_unicode;
use crate::hyeong::state::{OptState, State, UnOptState};
use crate::io::ReadLine;
use anyhow::{bail, Result};
use number::num::Num;
use std::io::Write;

use super::area::HeartType;

pub trait ExecutableState: State {
    fn push_stack(&mut self, out: &mut impl Write, err: &mut impl Write, idx: usize, num: Num) -> Result<()> {
        if idx == 1 {
            if num.is_pos() {
                write!(out, "{}", num_to_unicode(&num)?)?;
            } else {
                write!(out, "{}", -&num)?;
            }
        } else if idx == 2 {
            if num.is_pos() {
                write!(err, "{}", num_to_unicode(&num)?)?;
            } else {
                write!(err, "{}", -&num)?;
            }
        } else {
            self.push_stack_impl(idx, num);
        }
        Ok(())
    }

    fn push_stack_impl(&mut self, idx: usize, num: Num) {
        let st = self.get_stack(idx);
        if !st.is_empty() || !num.is_nan() {
            st.push(num);
        }
    }

    fn pop_stack(
        &mut self,
        in_: &mut impl ReadLine,
        out: &mut impl Write,
        err: &mut impl Write,
        idx: usize,
    ) -> Result<Num> {
        if idx == 0 {
            if self.get_stack(0).is_empty() {
                let s = in_.read_line_()?;
                for c in s.chars() {
                    self.push_stack_impl(0, Num::from_num(c as isize));
                }
            }
            Ok(self.pop_stack_impl(0))
        } else if idx <= 2 {
            out.flush()?;
            err.flush()?;
            bail!(idx - 1);
        } else {
            Ok(self.pop_stack_impl(idx))
        }
    }

    fn pop_stack_impl(&mut self, idx: usize) -> Num {
        self.get_stack(idx).pop().unwrap_or_else(Num::nan)
    }

    /// Executes only one line of code and return next position of code
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::io::{CustomReader, CustomWriter};
    /// use hyeong::core::state::{UnOptState, State};
    /// use hyeong::core::{parse, execute};
    ///
    /// let mut a = CustomReader::new(String::from("0"));
    /// let mut b = CustomWriter::new(|_| Result::Ok(()));
    /// let mut c = CustomWriter::new(|_| Result::Ok(()));
    /// let mut s = UnOptState::new();
    /// let t = parse::parse(String::from("형..."));
    /// s.push_code(t[0].clone());
    ///
    /// let (mut s, _) = execute::execute_one(&mut a, &mut b, &mut c, s, 0).unwrap();
    /// assert_eq!("3", s.get_stack(3)[0].to_string());
    /// ```
    fn execute_one(
        &mut self,
        in_: &mut impl ReadLine,
        out: &mut impl Write,
        err: &mut impl Write,
    ) -> Result<()> {
        let loc = self.get_loc();
        let code = (*self.get_code(loc)).clone();
        let stack_idx = self.current_stack();

        match code.type_() {
            CodeType::Hyeong => {
                let n = Num::from_num((code.hangul_count() + code.dot_count()) as isize);
                self.push_stack(out, err, stack_idx, n)?;
            }
            CodeType::Hang => {
                let n = (0..code.hangul_count())
                    .map(|_| self.pop_stack(in_, out, err, stack_idx))
                    .try_fold(Num::zero(), |acc, x| Ok::<_, anyhow::Error>(&acc + &x?))?;

                self.push_stack(out, err, code.dot_count(), n)?;
            }
            CodeType::Hat => {
                let n = (0..code.hangul_count())
                    .map(|_| self.pop_stack(in_, out, err, stack_idx))
                    .try_fold(Num::zero(), |acc, x| Ok::<_, anyhow::Error>(&acc * &x?))?;

                self.push_stack(out, err, code.dot_count(), n)?;
            }
            ty @ CodeType::Heut | ty @ CodeType::Heup => {
                let n = (0..code.hangul_count())
                    .map(|_| self.pop_stack(in_, out, err, stack_idx))
                    .rev()
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .try_fold(Num::zero(), |acc, mut x| {
                        let r = if matches!(ty, CodeType::Heut) {
                            x.minus();
                            &acc + &x
                        } else {
                            x.flip();
                            &acc * &x
                        };
                        self.push_stack(out, err, stack_idx, x)?;
                        Ok::<_, anyhow::Error>(r)
                    })?;

                self.push_stack(out, err, code.dot_count(), n)?;
            }
            CodeType::Heuk => {
                let n = self.pop_stack(in_, out, err, stack_idx)?;
                for _ in 0..code.hangul_count() {
                    self.push_stack(out, err, code.dot_count(), n.clone())?;
                }
                self.push_stack(out, err, stack_idx, n)?;
                self.set_current_stack(code.dot_count());
            }
        }

        let stack_idx = self.current_stack();

        if let Some(area_type) = code
            .area()
            .calc(code.area_count(), || self.pop_stack(in_, out, err, stack_idx))?
        {
            if matches!(area_type, HeartType::WhiteHeartSuit) {
                let id = ((code.area_count() as u128) << 4) + area_type as u128;
                match self.get_point(id) {
                    Some(value) => {
                        if loc != value {
                            self.set_latest_loc(loc);
                            self.set_loc(value);
                            return Ok(());
                        }
                    }
                    None => self.set_point(id, loc),
                }
            } else if let Some(loc) = self.get_latest_loc() {
                self.set_loc(loc);
                return Ok(());
            }
        }

        self.set_loc(loc + 1);
        Ok(())
    }

    /// Execute from new code until needs new code or finish
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::core::{execute, parse};
    /// use hyeong::util::io::{CustomReader, CustomWriter};
    /// use hyeong::core::state::{UnOptState, State};
    ///
    /// let mut a = CustomReader::new(String::from("0"));
    /// let mut b = CustomWriter::new(|_| Result::Ok(()));
    /// let mut c = CustomWriter::new(|_| Result::Ok(()));
    /// let mut s = UnOptState::new();
    /// let t = parse::parse(String::from("형..."));
    ///
    /// let mut s = execute::execute(&mut a, &mut b, &mut c, s, &t[0]).unwrap();
    /// assert_eq!("3", s.get_stack(3)[0].to_string());
    /// ```
    fn execute(
        &mut self,
        in_: &mut impl ReadLine,
        out: &mut impl Write,
        err: &mut impl Write,
        code: &Self::CodeType,
    ) -> Result<()> {
        let length = self.push_code((*code).clone()) + 1;

        while self.get_loc() < length {
            self.execute_one(in_, out, err)?;
        }

        Ok(())
    }
}

impl ExecutableState for UnOptState<'_> {}

impl ExecutableState for OptState {}
