//! @ Physical sizes that a \TeX\ user specifies for portions of documents are
//! represented internally as scaled points. Thus, if we define an `sp' (scaled
//! @^sp@>
//! point) as a unit equal to $2^{-16}$ printer's points, every dimension
//! inside of \TeX\ is an integer number of sp. There are exactly
//! 4,736,286.72 sp per inch.  Users are not allowed to specify dimensions
//! larger than $2^{30}-1$ sp, which is a distance of about 18.892 feet (5.7583
//! meters); two such quantities can be added without overflow on a 32-bit
//! computer.
//!
//! The present implementation of \TeX\ does not check for overflow when
//! @^overflow in arithmetic@>
//! dimensions are added or subtracted. This could be done by inserting a
//! few dozen tests of the form `\ignorespaces|if x>=@'10000000000 then
//! @t\\{report\_overflow}@>|', but the chance of overflow is so remote that
//! such tests do not seem worthwhile.
//!
//! \TeX\ needs to do only a few arithmetic operations on scaled quantities,
//! other than addition and subtraction, and the following subroutines do most of
//! the work. A single computation might use several subroutine calls, and it is
//! desirable to avoid producing multiple error messages in case of arithmetic
//! overflow; so the routines set the global variable |arith_error| to |true|
//! instead of reporting errors directly to the user. Another global variable,
//! |remainder|, holds the remainder after a division.
//
// @<Glob...@>=
// @!arith_error:boolean; {has arithmetic overflow occurred recently?}
/// has arithmetic overflow occurred recently?
#[globals_struct_field(TeXGlobals)]
pub(crate) static arith_error: boolean = false;
// @!remainder:scaled; {amount subtracted to get an exact division}
/// amount subtracted to get an exact division
#[globals_struct_field(TeXGlobals)]
pub(crate) static remainder: scaled = scaled::zero();

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::scaled;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};

