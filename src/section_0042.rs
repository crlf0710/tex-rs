//! @ Strings are created by appending character codes to |str_pool|.
//! The |append_char| macro, defined here, does not check to see if the
//! value of |pool_ptr| has gotten too high; this test is supposed to be
//! made before |append_char| is used. There is also a |flush_char|
//! macro, which erases the last character appended.
//!
//! To test if there is room to append |l| more characters to |str_pool|,
//! we shall write |str_room(l)|, which aborts \TeX\ and gives an
//! apologetic error message if there isn't enough room.
//!
//! @d append_char(#) == {put |ASCII_code| \# at the end of |str_pool|}
//! begin str_pool[pool_ptr]:=si(#); incr(pool_ptr);
//! end
//! @d flush_char == decr(pool_ptr) {forget the last character in the pool}
//! @d str_room(#) == {make sure that the pool hasn't overflowed}
//!   begin if pool_ptr+# > pool_size then
//!   overflow("pool size",pool_size-init_pool_ptr);
//! @:TeX capacity exceeded pool size}{\quad pool size@>
//!   end
//!
