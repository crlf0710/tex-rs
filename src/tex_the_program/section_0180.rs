//! @ Since boxes can be inside of boxes, |show_node_list| is inherently recursive,
//! @^recursion@>
//! up to a given maximum number of levels.  The history of nesting is indicated
//! by the current string, which will be printed at the beginning of each line;
//! the length of this string, namely |cur_length|, is the depth of nesting.
//!
//! Recursive calls on |show_node_list| therefore use the following pattern:
//
// @d node_list_display(#)==
pub(crate) macro node_list_display($globals:expr, $p:expr) {{
    // begin append_char("."); show_node_list(#); flush_char;
    append_char(
        make_globals_string_view!($globals),
        ASCII_code_literal!(b'.'),
    );
    show_node_list($globals, $p as _);
    flush_char($globals);
    // end {|str_room| need not be checked; see |show_box| below}
    /// `str_room` need not be checked; see `show_box` below
    const _: () = ();
    use crate::section_0004::make_globals_string_view;
    use crate::section_0004::TeXGlobalsStringView;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0042::append_char;
    use crate::section_0042::flush_char;
    use crate::section_0182::show_node_list;
}}
