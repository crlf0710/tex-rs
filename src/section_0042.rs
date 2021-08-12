//! @ Strings are created by appending character codes to |str_pool|.
//! The |append_char| macro, defined here, does not check to see if the
//! value of |pool_ptr| has gotten too high; this test is supposed to be
//! made before |append_char| is used. There is also a |flush_char|
//! macro, which erases the last character appended.
//!
//! To test if there is room to append |l| more characters to |str_pool|,
//! we shall write |str_room(l)|, which aborts \TeX\ and gives an
//! apologetic error message if there isn't enough room.
//
// @d append_char(#) == {put |ASCII_code| \# at the end of |str_pool|}
/// put `ASCII_code` `#` at the end of `str_pool`
pub(crate) fn append_char(globals: TeXGlobalsStringView<'_>, val: ASCII_code) {
    // begin str_pool[pool_ptr]:=si(#); incr(pool_ptr);
    #[cfg(not(feature = "unicode_support"))]
    {
        globals.str_pool[globals.pool_ptr] = si(val);
        incr!(globals.pool_ptr);
    }
    #[cfg(feature = "unicode_support")]
    {
        let mut buffer = [0; 6];
        let encoded = val.0.encode_runestr(&mut buffer);
        for byte in encoded.bytes() {
            (*globals.str_pool)[*globals.pool_ptr] = packed_ASCII_code::from(byte);
            incr!(*globals.pool_ptr);
        }
        use crate::section_0038::packed_ASCII_code;
    }
    // end
}

// @d flush_char == decr(pool_ptr) {forget the last character in the pool}
/// forget the last character in the pool
pub(crate) fn flush_char(globals: &mut TeXGlobals) {
    decr!(globals.pool_ptr);
}

// @d str_room(#) == {make sure that the pool hasn't overflowed}
/// make sure that the pool hasn't overflowed
pub(crate) fn str_room(globals: &mut TeXGlobals, bytes_count: integer) {
    // begin if pool_ptr+# > pool_size then
    if globals.pool_ptr.get() + bytes_count as u32 > pool_size as u32 {
        todo!("overflow");
        // overflow("pool size",pool_size-init_pool_ptr);
        // @:TeX capacity exceeded pool size}{\quad pool size@>
        // end
    }
}

#[cfg(not(feature = "unicode_support"))]
pub(crate) const character_max_room: integer = 1;

#[cfg(feature = "unicode_support")]
pub(crate) const character_max_room: integer = 6;

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsStringView;
use crate::section_0011::pool_size;
use crate::section_0018::ASCII_code;
