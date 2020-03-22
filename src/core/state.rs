use crate::core::code::{Code, OptCode, UnOptCode};
use crate::number::number::Num;
use std::collections::HashMap;
use std::fmt;

/// State trait
///
/// It defines methods that state structure needs.
/// [UnOptState](struct.UnOptState.html) and [OptState](struct.OptState.html) use this trait.
pub trait State {
    type CodeType: Code + Clone;

    fn get_all_stack_index(&self) -> Vec<usize>;

    fn stack_size(&self) -> usize;

    fn current_stack(&self) -> usize;

    fn set_current_stack(&mut self, cur: usize);

    fn get_stack(&mut self, idx: usize) -> &mut Vec<Num>;

    fn push_stack(&mut self, idx: usize, num: Num) {
        let st = self.get_stack(idx);
        if !st.is_empty() || !num.is_nan() {
            st.push(num);
        }
    }

    fn pop_stack(&mut self, idx: usize) -> Num {
        match self.get_stack(idx).pop() {
            Some(t) => t,
            None => Num::nan(),
        }
    }

    fn get_code(&self, loc: usize) -> &Self::CodeType;

    fn push_code(&mut self, code: Self::CodeType) -> usize;

    fn get_all_code(&self) -> Vec<Self::CodeType>;

    fn set_point(&mut self, id: u128, loc: usize);

    fn get_point(&self, id: u128) -> Option<usize>;

    fn get_all_point(&self) -> Vec<(u128, usize)>;

    fn set_latest_loc(&mut self, loc: usize);

    fn get_latest_loc(&self) -> Option<usize>;
}

/// State structure for optimized code
///
/// It can be used for level 1, 2 optimization
///
/// # Examples
///
/// ```
/// use hyeong::state::{OptState, State};
///
/// let a = OptState::new(10);
/// assert_eq!(10, a.stack_size());
/// ```
#[derive(Clone)]
pub struct OptState {
    stack: Vec<Vec<Num>>,
    code: Vec<OptCode>,
    point: HashMap<u128, usize>,
    cur: usize,
    latest: Option<usize>,
}

impl OptState {
    /// Make new `OptState`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::state::{OptState, State};
    ///
    /// let a = OptState::new(10);
    /// assert_eq!(10, a.stack_size());
    /// ```
    pub fn new(size: usize) -> OptState {
        OptState {
            stack: vec![Vec::new(); size],
            code: vec![],
            point: HashMap::new(),
            cur: 3,
            latest: None,
        }
    }
}

impl State for OptState {
    type CodeType = OptCode;

    /// Return stack indices
    fn get_all_stack_index(&self) -> Vec<usize> {
        (0..self.stack.len()).collect()
    }

    /// Return stack count
    fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Return current stack index
    fn current_stack(&self) -> usize {
        self.cur
    }

    /// Set current stack to `cur`
    fn set_current_stack(&mut self, cur: usize) {
        self.cur = cur;
    }

    /// Return stack of `idx`
    fn get_stack(&mut self, idx: usize) -> &mut Vec<Num> {
        self.stack[idx].as_mut()
    }

    /// Push value to stack
    fn push_stack(&mut self, idx: usize, num: Num) {
        if idx < self.stack.len() {
            if !self.stack[idx].is_empty() || !num.is_nan() {
                self.get_stack(idx).push(num);
            }
        }
    }

    /// Pop value of stack and return popped value
    fn pop_stack(&mut self, idx: usize) -> Num {
        if idx < self.stack.len() {
            match self.get_stack(idx).pop() {
                Some(t) => t,
                None => Num::nan(),
            }
        } else {
            Num::nan()
        }
    }

    /// Return code
    fn get_code(&self, loc: usize) -> &Self::CodeType {
        &self.code[loc]
    }

    /// Push code to log
    fn push_code(&mut self, code: Self::CodeType) -> usize {
        self.code.push(code);
        self.code.len() - 1
    }

    /// Return all code
    fn get_all_code(&self) -> Vec<Self::CodeType> {
        self.code.clone()
    }

    /// Set point for area
    fn set_point(&mut self, id: u128, loc: usize) {
        self.point.insert(id, loc);
    }

    /// Return point for area
    fn get_point(&self, id: u128) -> Option<usize> {
        self.point.get(&id).map(|&x| x)
    }

    /// Return all points
    fn get_all_point(&self) -> Vec<(u128, usize)> {
        let mut v = Vec::with_capacity(self.point.len());
        for (a, b) in &self.point {
            v.push((*a, *b));
        }
        v
    }

    /// Set latest location
    fn set_latest_loc(&mut self, loc: usize) {
        self.latest = Option::Some(loc);
    }

    /// Return latest location
    fn get_latest_loc(&self) -> Option<usize> {
        self.latest
    }
}

/// State structure for unoptimized state
///
/// # Examples
///
/// ```
/// use hyeong::state::UnOptState;
///
/// let a = UnOptState::new();
/// ```
#[derive(Clone)]
pub struct UnOptState {
    stack: HashMap<usize, Vec<Num>>,
    code: Vec<UnOptCode>,
    point: HashMap<u128, usize>,
    cur: usize,
    latest: Option<usize>,
}

impl UnOptState {
    /// Make new `UnOptCode`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::state::UnOptState;
    ///
    /// let a = UnOptState::new();
    /// ```
    pub fn new() -> UnOptState {
        UnOptState {
            stack: HashMap::new(),
            code: vec![],
            point: HashMap::new(),
            cur: 3,
            latest: None,
        }
    }
}

impl State for UnOptState {
    type CodeType = UnOptCode;

    /// Return stack indices
    fn get_all_stack_index(&self) -> Vec<usize> {
        let mut v = Vec::with_capacity(self.stack.len());
        for (i, _) in &self.stack {
            v.push(*i);
        }
        v
    }

    /// Return stack count
    fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Return current stack
    fn current_stack(&self) -> usize {
        self.cur
    }

    /// Set current
    fn set_current_stack(&mut self, cur: usize) {
        self.cur = cur;
    }

    /// Return stack
    fn get_stack(&mut self, idx: usize) -> &mut Vec<Num> {
        self.stack.entry(idx).or_insert(Vec::new())
    }

    /// Return code
    fn get_code(&self, loc: usize) -> &Self::CodeType {
        &self.code[loc]
    }

    /// Push code to log
    fn push_code(&mut self, code: Self::CodeType) -> usize {
        self.code.push(code);
        self.code.len() - 1
    }

    /// Return all code
    fn get_all_code(&self) -> Vec<Self::CodeType> {
        self.code.clone()
    }

    /// Set point for area
    fn set_point(&mut self, id: u128, loc: usize) {
        self.point.insert(id, loc);
    }

    /// Return point for area
    fn get_point(&self, id: u128) -> Option<usize> {
        self.point.get(&id).map(|&x| x)
    }

    /// Return all points
    fn get_all_point(&self) -> Vec<(u128, usize)> {
        let mut v = Vec::with_capacity(self.point.len());
        for (a, b) in &self.point {
            v.push((*a, *b));
        }
        v
    }

    /// Set latest location
    fn set_latest_loc(&mut self, loc: usize) {
        self.latest = Option::Some(loc);
    }

    /// Return latest location
    fn get_latest_loc(&self) -> Option<usize> {
        self.latest
    }
}

impl fmt::Debug for UnOptState {
    /// Debug format function
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::state::{UnOptState, State};
    /// use hyeong::number::Num;
    ///
    /// let mut  a = UnOptState::new();
    /// a.push_stack(3, Num::one());
    /// assert_eq!("current stack: 3\nstack 3: [1]\n", format!("{:?}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("current stack: {}\n", self.cur);
        let mut v = self.stack.iter().collect::<Vec<_>>();
        v.sort_by(|x, y| x.0.cmp(&y.0));
        for (a, b) in v {
            s.push_str(&*format!("stack {}: {:?}\n", a, b));
        }
        write!(f, "{}", s)
    }
}
