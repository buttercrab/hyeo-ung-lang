use std::io::{BufRead, Write};

use anyhow::{bail, Result};

use number::num::Num;

use crate::hyeong::area::HeartType;
use crate::hyeong::code::{Code, HangulType};
use crate::hyeong::num_to_unicode;
use crate::hyeong::state::{OptState, State, UnOptState};

pub trait ExecutableState: State {
    fn push_stack(&mut self, out: &mut impl Write, err: &mut impl Write, idx: usize, num: Num) -> Result<()> {
        macro_rules! write_num {
            ($to:expr, $num:expr) => {
                if $num.is_pos() {
                    write!($to, "{}", num_to_unicode(&$num)?)?;
                } else {
                    write!($to, "{}", (-&$num).floor())?;
                }
            };
        }

        if idx == 1 {
            write_num!(out, num);
        } else if idx == 2 {
            write_num!(err, num);
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
        in_: &mut impl BufRead,
        out: &mut impl Write,
        err: &mut impl Write,
        idx: usize,
    ) -> Result<Num> {
        if idx == 0 {
            if self.get_stack(0).is_empty() {
                let mut s = String::new();
                in_.read_line(&mut s)?;
                for c in s.chars() {
                    self.push_stack_impl(0, Num::from_num(c as isize));
                }
            }
            Ok(self.pop_stack_impl(0))
        } else if idx <= 2 {
            out.flush()?;
            err.flush()?;
            bail!(idx);
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
        in_: &mut impl BufRead,
        out: &mut impl Write,
        err: &mut impl Write,
    ) -> Result<()> {
        let loc = self.get_loc();
        let code = (*self.get_code(loc)).clone();
        let stack_idx = self.current_stack();

        macro_rules! impl_hang_hat {
            ($init:ident, $op:tt) => {{
                let n = (0..code.hangul_count())
                    .map(|_| self.pop_stack(in_, out, err, stack_idx))
                    .try_fold(Num::$init(), |acc, x| Ok::<_, anyhow::Error>(&acc $op &x?))?;

                self.push_stack(out, err, code.dot_count(), n)?;
            }};
        }

        macro_rules! impl_heut_heup {
            ($init:ident, $op:tt) => {{
                let n = (0..code.hangul_count())
                    .map(|_| self.pop_stack(in_, out, err, stack_idx))
                    .rev()
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .try_fold(Num::$init(), |acc, mut x| {
                        x.flip();
                        let r = &acc $op &x;
                        self.push_stack(out, err, stack_idx, x)?;
                        Ok::<_, anyhow::Error>(r)
                    })?;

                self.push_stack(out, err, code.dot_count(), n)?;
            }};
        }

        match code.hangul_type() {
            HangulType::Hyeong => {
                let n = Num::from_num((code.hangul_count() * code.dot_count()) as isize);
                self.push_stack(out, err, stack_idx, n)?;
            }
            HangulType::Hang => impl_hang_hat!(zero, +),
            HangulType::Hat => impl_hang_hat!(one, *),
            HangulType::Heut => impl_heut_heup!(zero, +),
            HangulType::Heup => impl_heut_heup!(one, *),
            HangulType::Heuk => {
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
            if !matches!(area_type, HeartType::WhiteHeartSuit) {
                let id = (code.area_count(), area_type);
                match self.get_point(&id) {
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
        in_: &mut impl BufRead,
        out: &mut impl Write,
        err: &mut impl Write,
        code: Self::CodeType,
    ) -> Result<()> {
        let length = self.push_code(code);

        while self.get_loc() <= length {
            self.execute_one(in_, out, err)?;
        }

        Ok(())
    }
}

impl ExecutableState for UnOptState<'_> {}

impl ExecutableState for OptState {}
