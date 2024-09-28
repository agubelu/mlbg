use core::ops::{Index, IndexMut};

/// A representation of a 10-trit ternary value.
/// Using its public methods guarantees that the value contained within
/// always remains a valid Malbolge ternary value.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Value {
    val: u16
}

impl Value {
    pub const MAX: u16 = 59048;  // 3^10 - 1

    pub const fn zero() -> Self {
        Self { val: 0 }
    }

    pub const fn new(val: u16) -> Self {
        assert!(val <= Self::MAX);
        Self { val }
    }

    pub const fn val(&self) -> u16 {
        self.val
    }

    pub fn incr(&mut self) {
        if self.val == Self::MAX {
            self.val = 0;
        } else {
            self.val += 1;
        }
    }

    /// Rotates the ternary value one trit to the right
    pub const fn shr(&self) -> Self {
        let val = 19683 * (self.val % 3) + self.val / 3;
        Self { val }
    }

    /// Performs the crazy operation with `self` and another value.
    /// Note that the crazy operation isn't commutative, so a.crz(b) != b.crz(a)
    pub fn crz(&self, other: Self) -> Self {
        // For efficiency, we compute the crazy operation 2 trits at a time.
        let val = [1, 9, 81, 729, 6561].into_iter()
            .map(|pow| pow * CRZ_TABLE[(other.val / pow % 9) as usize][(self.val / pow % 9) as usize])
            .sum();
        Self { val }
    }
}

// Aux trait implementations to help index memory
impl<T> Index<Value> for Vec<T> {
    type Output = T;

    fn index(&self, index: Value) -> &Self::Output {
        &self[index.val() as usize]
    }
}

impl<T> IndexMut<Value> for Vec<T> {
    fn index_mut(&mut self, index: Value) -> &mut Self::Output {
        &mut self[index.val() as usize]
    }
}

// Trinary results of the crazy operation depending
// on the last 2 trits of both operands
static CRZ_TABLE: [[u16; 9]; 9] = [
    [4, 3, 3, 1, 0, 0, 1, 0, 0],
    [4, 3, 5, 1, 0, 2, 1, 0, 2],
    [5, 5, 4, 2, 2, 1, 2, 2, 1],
    [4, 3, 3, 1, 0, 0, 7, 6, 6],
    [4, 3, 5, 1, 0, 2, 7, 6, 8],
    [5, 5, 4, 2, 2, 1, 8, 8, 7],
    [7, 6, 6, 7, 6, 6, 4, 3, 3],
    [7, 6, 8, 7, 6, 8, 4, 3, 5],
    [8, 8, 7, 8, 8, 7, 5, 5, 4],
];
