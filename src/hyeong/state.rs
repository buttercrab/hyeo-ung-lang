use std::collections::HashMap;
use std::fmt;

use number::num::Num;

use crate::hyeong::area::HeartType;
use crate::hyeong::code::{Code, OptCode, UnOptCode};

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

    fn get_code(&self, loc: usize) -> &Self::CodeType;

    fn push_code(&mut self, code: Self::CodeType) -> usize;

    fn get_all_code(&self) -> Vec<Self::CodeType>;

    fn set_point(&mut self, id: (usize, HeartType), loc: usize);

    fn get_point(&self, id: &(usize, HeartType)) -> Option<usize>;

    fn get_all_point(&self) -> Vec<((usize, HeartType), usize)>;

    fn set_latest_loc(&mut self, loc: usize);

    fn get_latest_loc(&self) -> Option<usize>;

    fn set_loc(&mut self, loc: usize);

    fn get_loc(&self) -> usize;

    fn get_exit(&self) -> Option<i32>;
}

/// State structure for optimized code
///
/// It can be used for level 1, 2 optimization
///
/// # Examples
///
/// ```
/// use hyeong::core::state::{OptState, State};
///
/// let a = OptState::new(10);
/// assert_eq!(10, a.stack_size());
/// ```
#[derive(Clone)]
pub struct OptState {
    stack: Vec<Vec<Num>>,
    code: Vec<OptCode>,
    point: HashMap<(usize, HeartType), usize>,
    stack_idx: usize,
    latest: Option<usize>,
    loc: usize,
    exit: Option<i32>,
}

impl OptState {
    /// Make new `OptState`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::core::state::{OptState, State};
    ///
    /// let a = OptState::new(10);
    /// assert_eq!(10, a.stack_size());
    /// ```
    pub fn new(size: usize) -> OptState {
        OptState {
            stack: vec![Vec::new(); size],
            code: vec![],
            point: HashMap::new(),
            stack_idx: 3,
            latest: None,
            loc: 0,
            exit: None,
        }
    }

    pub fn exit(&mut self, code: i32) {
        self.exit = Some(code);
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
        self.stack_idx
    }

    /// Set current stack to `cur`
    fn set_current_stack(&mut self, cur: usize) {
        self.stack_idx = cur;
    }

    /// Return stack of `idx`
    fn get_stack(&mut self, idx: usize) -> &mut Vec<Num> {
        self.stack[idx].as_mut()
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
    fn set_point(&mut self, id: (usize, HeartType), loc: usize) {
        self.point.insert(id, loc);
    }

    /// Return point for area
    fn get_point(&self, id: &(usize, HeartType)) -> Option<usize> {
        self.point.get(id).copied()
    }

    /// Return all points
    fn get_all_point(&self) -> Vec<((usize, HeartType), usize)> {
        self.point.iter().map(|(k, v)| (*k, *v)).collect()
    }

    /// Set latest location
    fn set_latest_loc(&mut self, loc: usize) {
        self.latest = Some(loc);
    }

    /// Return latest location
    fn get_latest_loc(&self) -> Option<usize> {
        self.latest
    }

    fn set_loc(&mut self, loc: usize) {
        self.loc = loc;
    }

    fn get_loc(&self) -> usize {
        self.loc
    }

    fn get_exit(&self) -> Option<i32> {
        self.exit
    }
}

/// State structure for unoptimized state
///
/// # Examples
///
/// ```
/// use hyeong::core::state::UnOptState;
///
/// let a = UnOptState::new();
/// ```
#[derive(Clone)]
pub struct UnOptState<'a> {
    stack: HashMap<usize, Vec<Num>>,
    code: Vec<UnOptCode<'a>>,
    point: HashMap<(usize, HeartType), usize>,
    stack_idx: usize,
    latest: Option<usize>,
    loc: usize,
}

impl<'a> UnOptState<'a> {
    /// Make new `UnOptCode`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::core::state::UnOptState;
    ///
    /// let a = UnOptState::new();
    /// ```
    pub fn new() -> UnOptState<'a> {
        UnOptState {
            stack: HashMap::new(),
            code: vec![],
            point: HashMap::new(),
            stack_idx: 3,
            latest: None,
            loc: 0,
        }
    }
}

impl Default for UnOptState<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> State for UnOptState<'a> {
    type CodeType = UnOptCode<'a>;

    /// Return stack indices
    fn get_all_stack_index(&self) -> Vec<usize> {
        self.stack.keys().copied().collect()
    }

    /// Return stack count
    fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Return current stack
    fn current_stack(&self) -> usize {
        self.stack_idx
    }

    /// Set current
    fn set_current_stack(&mut self, cur: usize) {
        self.stack_idx = cur;
    }

    /// Return stack
    fn get_stack(&mut self, idx: usize) -> &mut Vec<Num> {
        self.stack.entry(idx).or_insert_with(Vec::new)
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
    fn set_point(&mut self, id: (usize, HeartType), loc: usize) {
        self.point.insert(id, loc);
    }

    /// Return point for area
    fn get_point(&self, id: &(usize, HeartType)) -> Option<usize> {
        self.point.get(id).copied()
    }

    /// Return all points
    fn get_all_point(&self) -> Vec<((usize, HeartType), usize)> {
        self.point.iter().map(|(k, v)| (*k, *v)).collect()
    }

    /// Set latest location
    fn set_latest_loc(&mut self, loc: usize) {
        self.latest = Some(loc);
    }

    /// Return latest location
    fn get_latest_loc(&self) -> Option<usize> {
        self.latest
    }

    fn set_loc(&mut self, loc: usize) {
        self.loc = loc;
    }

    fn get_loc(&self) -> usize {
        self.loc
    }

    fn get_exit(&self) -> Option<i32> {
        None
    }
}

impl fmt::Debug for UnOptState<'_> {
    /// Debug format function
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::core::state::{UnOptState, State};
    /// use hyeong::number::num::Num;
    ///
    /// let mut  a = UnOptState::new();
    /// a.push_stack(3, Num::one());
    /// assert_eq!("current stack: 3\nstack 3: [1]\n", format!("{:?}", a));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("current stack: {}\n", self.stack_idx);
        let mut v = self.stack.iter().collect::<Vec<_>>();
        v.sort_by(|x, y| x.0.cmp(y.0));
        for (a, b) in v {
            s.push_str(&format!("stack {a}: {b:?}\n"));
        }
        write!(f, "{s}")
    }
}
