use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Shl};

use num_bigint::BigInt;
use num_traits::{ConstOne, Num, One, Signed, Zero};

type Exp = u8;

/// (a + b sqrt(5)) / 2^n
#[derive(Debug, Clone)]
pub(crate) struct ZRoot5 {
    pub(crate) a: BigInt,
    pub(crate) b: BigInt,
    pub(crate) n: Exp,
}

impl ZRoot5 {
    pub(crate) fn new(a: impl Into<BigInt>, b: impl Into<BigInt>, n: Exp) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            n,
        }
    }

    /*
        exp_by_squaring(x, n)
      if n < 0 then
         return exp_by_squaring(1 / x, -n);
      else if n = 0 then
         return 1;
      else if n is even then
         return exp_by_squaring(x * x, n / 2);
      else if n is odd then
         return x * exp_by_squaring(x * x, (n - 1) / 2);
    end function
        */

    pub(crate) fn pow(&self, n: u32) -> Self {
        if n == 0 {
            return ZRoot5::new(1, 0, 0);
        }

        if n % 2 == 0 {
            let x = self.pow(n / 2);
            return x.clone() * &x;
        }

        let x = self * self.pow(n - 1);
        x
    }

    pub(crate) fn canonicalise(&mut self) {
        while (&self.a % 2u8).is_zero()
            && (&self.b % 2u8).is_zero()
            && !(self.a.is_zero() && self.b.is_zero())
            && self.n > 0
        {
            // println!(
            //     "({}, {}, {}) -> ({}, {}, {})",
            //     &self.a,
            //     &self.b,
            //     &self.n,
            //     &self.a / 2,
            //     &self.b / 2,
            //     &self.n - 1
            // );
            self.a /= 2;
            self.b /= 2;
            self.n -= 1;
        }
    }
}

impl Neg for ZRoot5 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ZRoot5 {
            a: -self.a,
            b: -self.b,
            n: self.n,
        }
    }
}

// o+o = o
impl Add<ZRoot5> for ZRoot5 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max = self.n.max(rhs.n);
        let min = self.n.min(rhs.n);
        let e = 1 << (max - min);
        let mut x = ZRoot5 {
            a: e * self.a + rhs.a,
            b: e * self.b + rhs.b,
            n: max,
        };
        x.canonicalise();
        x
    }
}

// // o+&o = o
// impl Add<&ZRoot5> for ZRoot5 {
//     type Output = Self;

//     fn add(self, rhs: &Self) -> Self::Output {
//         ZRoot5 {
//             a: self.a + &rhs.a,
//             b: self.b + &rhs.b,
//         }
//     }
// }

// // &o+o = o
// impl Add<ZRoot5> for &ZRoot5 {
//     type Output = ZRoot5;

//     fn add(self, rhs: ZRoot5) -> Self::Output {
//         ZRoot5 {
//             a: &self.a + rhs.a,
//             b: &self.b + rhs.b,
//         }
//     }
// }

// &o+&o = o
impl Add<&ZRoot5> for &ZRoot5 {
    type Output = ZRoot5;

    fn add(self, rhs: &ZRoot5) -> Self::Output {
        let max = self.n.max(rhs.n);
        let min = self.n.min(rhs.n);
        let e = 1 << (max - min);
        let mut x = ZRoot5 {
            a: e * &self.a + &rhs.a,
            b: e * &self.b + &rhs.b,
            n: max,
        };
        x.canonicalise();
        x
    }
}

// o*o = o
// impl Mul<ZRoot5> for ZRoot5 {
//     type Output = Self;

//     fn mul(self, rhs: ZRoot5) -> Self::Output {
//         ZRoot5 {
//             a: &self.a * &rhs.a + 5 * &self.b * &rhs.b,
//             b: self.a * rhs.b + self.b * rhs.a,
//         }
//     }
// }

// o*&o = o
impl Mul<&ZRoot5> for ZRoot5 {
    type Output = Self;

    fn mul(self, rhs: &ZRoot5) -> Self::Output {
        let mut x = ZRoot5 {
            a: &self.a * &rhs.a + 5 * &self.b * &rhs.b,
            b: self.a * &rhs.b + self.b * &rhs.a,
            n: self.n + rhs.n,
        };
        x.canonicalise();
        x
    }
}

// &o*o = o
impl Mul<ZRoot5> for &ZRoot5 {
    type Output = ZRoot5;

    fn mul(self, rhs: ZRoot5) -> Self::Output {
        let mut x = ZRoot5 {
            a: &self.a * &rhs.a + 5 * &self.b * &rhs.b,
            b: &self.a * &rhs.b + &self.b * &rhs.a,
            n: self.n + rhs.n,
        };
        x.canonicalise();
        x
    }
}
