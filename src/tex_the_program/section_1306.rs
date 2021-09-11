//! @ The inverse macros are slightly more complicated, since we need to check
//! the range of the values we are reading in. We say `|undump(a)(b)(x)|' to
//! read an integer value |x| that is supposed to be in the range |a<=x<=b|.
//! System error messages should be suppressed when undumping.
//! @^system dependencies@>
//
// @d undump_wd(#)==begin get(fmt_file); #:=fmt_file^;@+end
pub(crate) macro undump_wd($globals:expr, $v:expr) {{
    get(&mut $globals.fmt_file);
    $v = buffer_variable(&mut $globals.fmt_file);
    use crate::pascal::buffer_variable;
    use crate::pascal::get;
}}
// @d undump_int(#)==begin get(fmt_file); #:=fmt_file^.int;@+end
pub(crate) macro undump_int {
    ($globals:expr, $v:expr) => {{
        get(&mut $globals.fmt_file);
        $v = buffer_variable(&mut $globals.fmt_file)[MEMORY_WORD_INT];
        use crate::pascal::buffer_variable;
        use crate::pascal::get;
        use crate::section_0113::MEMORY_WORD_INT;
    }},
    ($globals:expr, $v:expr, $value_ctor:expr) => {{
        get(&mut $globals.fmt_file);
        $v = ($value_ctor)(buffer_variable(&mut $globals.fmt_file)[MEMORY_WORD_INT] as _);
        use crate::pascal::buffer_variable;
        use crate::pascal::get;
        use crate::section_0113::MEMORY_WORD_INT;
    }}
}
// @d undump_hh(#)==begin get(fmt_file); #:=fmt_file^.hh;@+end
pub(crate) macro undump_hh($globals:expr, $v:expr) {{
    get(&mut $globals.fmt_file);
    $v = buffer_variable(&mut $globals.fmt_file)[MEMORY_WORD_HH];
    use crate::pascal::buffer_variable;
    use crate::pascal::get;
    use crate::section_0113::MEMORY_WORD_HH;
}}
// @d undump_qqqq(#)==begin get(fmt_file); #:=fmt_file^.qqqq;@+end
pub(crate) macro undump_qqqq($globals:expr, $v:expr) {{
    get(&mut $globals.fmt_file);
    $v = buffer_variable(&mut $globals.fmt_file)[MEMORY_WORD_QQQQ];
    use crate::pascal::buffer_variable;
    use crate::pascal::get;
    use crate::section_0113::MEMORY_WORD_QQQQ;
}}
// @d undump_end_end(#)==#:=x;@+end
// @d undump_end(#)==(x>#) then goto bad_fmt@+else undump_end_end
// @d undump(#)==begin undump_int(x); if (x<#) or undump_end
pub(crate) macro undump($globals:expr, $min:expr, $max:expr, $val:expr, $val_ctor:expr, $lbl_bad_fmt:lifetime) {{
    let x;
    undump_int!($globals, x);
    if x < $min as integer || x > $max as integer {
        crate::trace_error_expr!("{} is not in {}..={}", x, $min, $max);
        crate::goto_forward_label!($lbl_bad_fmt);
    } else {
        $val = ($val_ctor)(x as _);
    }
    use crate::pascal::integer;
}}
// @d undump_size_end_end(#)==too_small(#)@+else undump_end_end
// @d undump_size_end(#)==if x># then undump_size_end_end
// @d undump_size(#)==begin undump_int(x);
//   if x<# then goto bad_fmt; undump_size_end
pub(crate) macro undump_size($globals:expr, $min:expr, $max:expr, $prompt:expr, $val:expr, $val_ctor:expr, $lbl_bad_fmt:lifetime) {{
    let x;
    undump_int!($globals, x);
    if x < $min as integer {
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    if x > $max as integer {
        too_small!($globals, $prompt, $lbl_bad_fmt);
    } else {
        $val = ($val_ctor)(x as _);
    }
    use crate::pascal::integer;
    use crate::section_1303::too_small;
}}
