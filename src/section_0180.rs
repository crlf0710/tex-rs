//! @ Since boxes can be inside of boxes, |show_node_list| is inherently recursive,
//! @^recursion@>
//! up to a given maximum number of levels.  The history of nesting is indicated
//! by the current string, which will be printed at the beginning of each line;
//! the length of this string, namely |cur_length|, is the depth of nesting.
//!
//! Recursive calls on |show_node_list| therefore use the following pattern:
//
// @d node_list_display(#)==
//   begin append_char("."); show_node_list(#); flush_char;
//   end {|str_room| need not be checked; see |show_box| below}
//
