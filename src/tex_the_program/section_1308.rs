//! @ Sections of a \.{WEB} program that are ``commented out'' still contribute
//! strings to the string pool; therefore \.{INITEX} and \TeX\ will have
//! the same strings. (And it is, of course, a good thing that they do.)
//! @.WEB@>
//! @^string pool@>
//
// @<Undump constants for consistency check@>=
pub(crate) macro Undump_constants_for_consistency_check($globals:expr, $lbl_bad_fmt:lifetime) {{
    /// something undumped
    let mut x: integer;
    // x:=fmt_file^.int;
    x = buffer_variable(&mut $globals.fmt_file)[MEMORY_WORD_INT];
    // if x<>@$ then goto bad_fmt; {check that strings are the same}
    if x != string_pool_checksum() as integer {
        /// check that strings are the same
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    // undump_int(x);
    undump_int!($globals, x);
    // if x<>mem_bot then goto bad_fmt;
    if x != mem_bot as integer {
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    // undump_int(x);
    undump_int!($globals, x);
    // if x<>mem_top then goto bad_fmt;
    if x != mem_top as integer {
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    // undump_int(x);
    undump_int!($globals, x);
    // if x<>eqtb_size then goto bad_fmt;
    if x != eqtb_size as integer {
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    // undump_int(x);
    undump_int!($globals, x);
    // if x<>hash_prime then goto bad_fmt;
    if x != hash_prime {
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    // undump_int(x);
    undump_int!($globals, x);
    // if x<>hyph_size then goto bad_fmt
    if x != hyph_size {
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    use crate::pascal::buffer_variable;
    use crate::pascal::integer;
    use crate::section_0012::hash_prime;
    use crate::section_0012::hyph_size;
    use crate::section_0012::mem_bot;
    use crate::section_0012::mem_top;
    use crate::section_0113::MEMORY_WORD_INT;
    use crate::section_0247::eqtb_size;
    use crate::section_1306::undump_int;
    use crate::string_pool::string_pool_checksum;
}}
