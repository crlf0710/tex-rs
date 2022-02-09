//! @ Here is a routine that calculates half of an integer, using an
//! unambiguous convention with respect to signed odd numbers.

pub(crate) trait Half {
    fn half(self) -> Self;
}

// @p function half(@!x:integer):integer;
pub(crate) fn half(x: integer) -> integer {
    // begin if odd(x) then half:=(x+1) div 2
    if x.is_odd() {
        (x + 1) / 2
    }
    // else half:=x @!div 2;
    else {
        x / 2
    }
    // end;
}

impl Half for integer {
    fn half(self) -> Self {
        half(self)
    }
}

impl Half for scaled {
    fn half(self) -> Self {
        scaled::new_from_inner(half(self.inner()))
    }
}

use crate::pascal::integer;
use crate::pascal::IsOddOrEven;
use crate::section_0101::scaled;
