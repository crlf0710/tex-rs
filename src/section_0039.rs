//! @ @<Glob...@>=
// @!str_pool:packed array[pool_pointer] of packed_ASCII_code; {the characters}
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringView)]
#[globals_struct_field_view(TeXGlobalsStringView)]
pub(crate) static str_pool: str_pool_array<packed_ASCII_code> = str_pool_array::default();

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) str_pool_array[pool_pointer] => u32; U32; pool_size_TYPENUM
);

#[globals_struct_use(TeXGlobals)]
use crate::section_0039::str_pool_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::packed_ASCII_code;

// @!str_start : array[str_number] of pool_pointer; {the starting pointers}
/// the starting pointers
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringView)]
#[globals_struct_field_view(TeXGlobalsStringView)]
pub(crate) static str_start: str_start_array<pool_pointer> = str_start_array::default();

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) str_start_array[str_number] => u32; U32; max_strings_TYPENUM
);

#[globals_struct_use(TeXGlobals)]
use crate::section_0039::str_start_array;

// @!pool_ptr : pool_pointer; {first unused position in |str_pool|}
/// first unused position in `str_pool`
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringView)]
#[globals_struct_field_view(TeXGlobalsIoStringView)]
#[globals_struct_field_view(TeXGlobalsStringView)]
pub(crate) static pool_ptr: pool_pointer = pool_pointer::new_zero();
// @!str_ptr : str_number; {number of the current string being created}
/// number of the current string being created
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringView)]
#[globals_struct_field_view(TeXGlobalsStringView)]
pub(crate) static str_ptr: str_number = str_number::new_zero();
// @!init_pool_ptr : pool_pointer; {the starting value of |pool_ptr|}
/// the starting value of `pool_ptr`
#[globals_struct_field(TeXGlobals)]
pub(crate) static init_pool_ptr: pool_pointer = pool_pointer::new_zero();
// @!init_str_ptr : str_number; {the starting value of |str_ptr|}
/// the starting value of `str_ptr`
#[globals_struct_field(TeXGlobals)]
pub(crate) static init_str_ptr: str_number = str_number::new_zero();

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::pool_pointer;
#[globals_struct_use(TeXGlobals)]
use crate::section_0038::str_number;

use crate::section_0004::TeXGlobals;
use crate::section_0011::max_strings_TYPENUM;
use crate::section_0011::pool_size_TYPENUM;
use crate::section_0038::packed_ASCII_code;
use crate::section_0038::pool_pointer;
use crate::section_0038::str_number;
use globals_struct::{globals_struct_field, globals_struct_use};

#[cfg(feature = "unicode_support")]
impl str_pool_array<packed_ASCII_code> {
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(level = "trace", skip(self, str_start))
    )]
    pub(crate) fn str_ascii_codes(
        &self,
        str_start: &str_start_array<pool_pointer>,
        s: str_number,
    ) -> crate::unicode_support::GenericCharIter<'_> {
        let slice = &self[str_start[s]..str_start[s + 1]];
        trace_expr!("slice_len = {}", slice.len());
        crate::unicode_support::GenericCharIter::new(slice)
    }
}
