use std::fmt::{Display, Formatter, Error};
use std::ops::*;

/// Structure for bit arrays
#[derive(Clone, Debug)]
pub struct BitVector {
    len: usize,
    vec: Vec<u64>,
}

impl BitVector {
    /// Creates new bit array
    pub fn new() -> Self {
        BitVector { len: 0, vec: Vec::new() }
    }

    /// Gets length of bit array
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Appends element to the last of bit array
    pub fn push(&mut self, value: bool) {
        if self.vec.len() <= (self.len+63)/64 {
            self.vec.push(0);
        }

        self.len += 1;
        self.set(self.len-1, value);
    }

    /// Removes the last element of bit array and return it
    pub fn pop(&mut self) -> Option<bool> {
        if self.len == 0 {
            None
        }else {
            let value = self.get(self.len-1);
            self.set(self.len-1, false);

            self.len -= 1;

            Some(value)
        }
    }

    /// Gets indicaded element of bit array
    pub fn get(&self, index: usize) -> bool {
        if self.len <= index {
            panic!("out of range");
        }

        let global_index = index/64;
        let local_index = index - global_index*64;
        
        if self.vec[global_index] & (0x1 << local_index) != 0 {
            true
        }else {
            false
        }
    }

    /// Sets element
    pub fn set(&mut self, index: usize, value: bool) {
        if self.len <= index {
            panic!("out of range");
        }

        let global_index = index/64;
        let local_index = index - global_index*64;

        if value {
            self.vec[global_index] |= 0x1 << local_index;
        }else {
            self.vec[global_index] &= !(0x1 << local_index);
        }
    }
}

impl Display for BitVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[")?;

        for i in 0 .. self.len() {
            if i != 0 {
                write!(f, ", ")?;
            }

            if self.get(i) {
                write!(f, "1")?;
            }else {
                write!(f, "0")?;
            }
        }

        write!(f, "]")?;

        Ok(())
    }
}

impl BitAnd<&Self> for BitVector {
    type Output = Self;

    fn bitand(mut self, rhs: &Self) -> Self::Output {
        self &= &rhs;
        self
    }
}

impl BitAndAssign<&Self> for BitVector {
    fn bitand_assign(&mut self, rhs: &Self) {
        if self.len() != rhs.len() {
            panic!("invalid input");
        }

        for i in 0 .. self.vec.len() {
            self.vec[i] &= rhs.vec[i];
        }
    }
}

impl BitOr<&Self> for BitVector {
    type Output = Self;

    fn bitor(mut self, rhs: &Self) -> Self::Output {
        self |= &rhs;
        self
    }
}

impl BitOrAssign<&Self> for BitVector {
    fn bitor_assign(&mut self, rhs: &Self) {
        if self.len() != rhs.len() {
            panic!("invalid input");
        }

        for i in 0 .. self.vec.len() {
            self.vec[i] |= rhs.vec[i];
        }
    }
}

impl BitXor<&Self> for BitVector {
    type Output = Self;

    fn bitxor(mut self, rhs: &Self) -> Self::Output {
        self ^= &rhs;
        self
    }
}

impl BitXorAssign<&Self> for BitVector {
    fn bitxor_assign(&mut self, rhs: &Self) {
        if self.len() != rhs.len() {
            panic!("invalid input");
        }

        for i in 0 .. self.vec.len() {
            self.vec[i] ^= rhs.vec[i];
        }
    }
}

impl Not for BitVector {
    type Output = Self;

    fn not(mut self) -> Self {
        for i in 0 .. self.vec.len() {
            self.vec[i] = !self.vec[i];
        }

        self
    }
}
