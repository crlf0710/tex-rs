//! @ When |hlist_out| is called, its duty is to output the box represented
//! by the |hlist_node| pointed to by |temp_ptr|. The reference point of that
//! box has coordinates |(cur_h,cur_v)|.
//!
//! Similarly, when |vlist_out| is called, its duty is to output the box represented
//! by the |vlist_node| pointed to by |temp_ptr|. The reference point of that
//! box has coordinates |(cur_h,cur_v)|.
//! @^recursion@>
//!
//! @p procedure@?vlist_out; forward; {|hlist_out| and |vlist_out| are mutually
//!   recursive}
//!
