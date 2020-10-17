//! @* \[4] String handling.
//! Control sequence names and diagnostic messages are variable-length strings
//! of eight-bit characters. Since \PASCAL\ does not have a well-developed string
//! mechanism, \TeX\ does all of its string processing by homegrown methods.
//!
//! Elaborate facilities for dynamic strings are not needed, so all of the
//! necessary operations can be handled with a simple data structure.
//! The array |str_pool| contains all of the (eight-bit) ASCII codes in all
//! of the strings, and the array |str_start| contains indices of the starting
//! points of each string. Strings are referred to by integer numbers, so that
//! string number |s| comprises the characters |str_pool[j]| for
//! |str_start[s]<=j<str_start[s+1]|. Additional integer variables
//! |pool_ptr| and |str_ptr| indicate the number of entries used so far
//! in |str_pool| and |str_start|, respectively; locations
//! |str_pool[pool_ptr]| and |str_start[str_ptr]| are
//! ready for the next string to be allocated.
//!
//! String numbers 0 to 255 are reserved for strings that correspond to single
//! ASCII characters. This is in accordance with the conventions of \.{WEB},
//! @.WEB@>
//! which converts single-character strings into the ASCII code number of the
//! single character involved, while it converts other strings into integers
//! and builds a string pool file. Thus, when the string constant \.{"."} appears
//! in the program below, \.{WEB} converts it into the integer 46, which is the
//! ASCII code for a period, while \.{WEB} will convert a string like \.{"hello"}
//! into some integer greater than~255. String number 46 will presumably be the
//! single character `\..'; but some ASCII codes have no standard visible
//! representation, and \TeX\ sometimes needs to be able to print an arbitrary
//! ASCII character, so the first 256 strings are used to specify exactly what
//! should be printed for each of the 256 possibilities.
//!
//! Elements of the |str_pool| array must be ASCII codes that can actually
//! be printed; i.e., they must have an |xchr| equivalent in the local
//! character set. (This restriction applies only to preloaded strings,
//! not to those generated dynamically by the user.)
//!
//! Some \PASCAL\ compilers won't pack integers into a single byte unless the
//! integers lie in the range |-128..127|. To accommodate such systems
//! we access the string pool only via macros that can easily be redefined.
//! @^system dependencies@>
//
// @d si(#) == # {convert from |ASCII_code| to |packed_ASCII_code|}
// @d so(#) == # {convert from |packed_ASCII_code| to |ASCII_code|}
//
// @<Types...@>=
// @!pool_pointer = 0..pool_size; {for variables that point into |str_pool|}
// @!str_number = 0..max_strings; {for variables that point into |str_start|}
#[derive(Copy, Clone)]
pub(crate) struct str_number(pub(crate) u32_from_m_to_n<U0, max_strings_TYPENUM>);

// @!packed_ASCII_code = 0..255; {elements of |str_pool| array}
//

use crate::pascal::{integer, u32_from_m_to_n};
use crate::section_0011::max_strings_TYPENUM;
use typenum::{U0, U1, U2147483648, U63};

impl str_number {
    pub(crate) fn new_zero() -> Self {
        str_number(u32_from_m_to_n::new(0))
    }
    pub(crate) fn is_zero(&self) -> bool {
        (self.0).get() == 0
    }
}
