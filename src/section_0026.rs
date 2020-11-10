//! @ Most of what we need to do with respect to input and output can be handled
//! by the I/O facilities that are standard in \PASCAL, i.e., the routines
//! called |get|, |put|, |eof|, and so on. But
//! standard \PASCAL\ does not allow file variables to be associated with file
//! names that are determined at run time, so it cannot be used to implement
//! \TeX; some sort of extension to \PASCAL's ordinary |reset| and |rewrite|
//! is crucial for our purposes. We shall assume that |name_of_file| is a variable
//! of an appropriate type such that the \PASCAL\ run-time system being used to
//! implement \TeX\ can open a file whose external name is specified by
//! |name_of_file|.
//! @^system dependencies@>

// @<Glob...@>=
// @!name_of_file:packed array[1..file_name_size] of char;@;@/
//   {on some systems this may be a \&{record} variable}
/// on some systems this may be a **record** variable
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringView)]
pub(crate) static name_of_file: name_of_file_array<crate::pascal::char> =
    name_of_file_array::from_copied(crate::pascal::char::new(b' ' as _));

type name_of_file_array_LENGTH_TYPENUM = typenum::op!(file_name_size_TYPENUM - U1 + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) name_of_file_array[u16_from_m_to_n<U1, file_name_size_TYPENUM>] =>
    u16; U16; U1; name_of_file_array_LENGTH_TYPENUM
);

#[globals_struct_use(TeXGlobals)]
use crate::section_0026::name_of_file_array;

// @!name_length:0..file_name_size;@/{this many characters are actually
//   relevant in |name_of_file| (the rest are blank)}
/// this many characters are actually relevant in `name_of_file` (the rest are blank)
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringView)]
pub(crate) static name_length: u16_from_0_to_n<file_name_size_TYPENUM> = u16_from_0_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u16_from_0_to_n;
#[globals_struct_use(TeXGlobals)]
use crate::section_0011::file_name_size_TYPENUM;

use crate::pascal::u16_from_m_to_n;
use crate::section_0011::file_name_size_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;

impl name_of_file_array<crate::pascal::char> {
    pub(crate) fn assign_str(&mut self, val: &str) {
        #[cfg(not(feature = "unicode_support"))]
        {
            debug_assert_eq!(
                name_of_file_array_LENGTH_TYPENUM::U16 as usize,
                val.bytes().count()
            );
            for (dest, src) in self.0.iter_mut().zip(val.bytes()) {
                *dest = crate::pascal::char::new(src);
            }
        }
        #[cfg(feature = "unicode_support")]
        {
            debug_assert_eq!(
                name_of_file_array_LENGTH_TYPENUM::U16 as usize,
                val.chars().count()
            );
            for (dest, src) in self.0.iter_mut().zip(val.chars()) {
                *dest = crate::pascal::char::new(src as _);
            }
        }
        use typenum::Unsigned;
    }
}

impl Into<String> for &'_ name_of_file_array<crate::pascal::char> {
    fn into(self) -> String {
        let mut result = String::new();
        for ch in self.iter() {
            #[cfg(not(feature = "unicode_support"))]
            {
                result.push(ch.0 as char);
            }
            #[cfg(feature = "unicode_support")]
            {
                use crate::unicode_support::chars_from_generalized_char;
                result.extend(chars_from_generalized_char(*ch));
            }
        }
        result
    }
}
