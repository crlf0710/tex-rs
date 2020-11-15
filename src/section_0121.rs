//! @ Conversely, a one-word node is recycled by calling |free_avail|.
//! This routine is part of \TeX's ``inner loop,'' so we want it to be fast.
//! @^inner loop@>
//!
//! @d free_avail(#)== {single-word node liberation}
//!   begin link(#):=avail; avail:=#;
//!   @!stat decr(dyn_used);@+tats@/
//!   end
//!
