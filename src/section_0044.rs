//! @ To destroy the most recently made string, we say |flush_string|.
//!
//! @d flush_string==begin decr(str_ptr); pool_ptr:=str_start[str_ptr];
//!   end
//!
