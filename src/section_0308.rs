//! @ The |param_stack| is an auxiliary array used to hold pointers to the token
//! lists for parameters at the current level and subsidiary levels of input.
//! This stack is maintained with convention (2), and it grows at a different
//! rate from the others.
//!
//! @<Glob...@>=
//! @!param_stack:array [0..param_size] of pointer;
//!   {token list pointers for parameters}
//! @!param_ptr:0..param_size; {first unused entry in |param_stack|}
//! @!max_param_stack:integer;
//!   {largest value of |param_ptr|, will be |<=param_size+9|}
//!