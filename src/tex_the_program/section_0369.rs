//! @ The implementation of \.{\\noexpand} is a bit trickier, because it is
//! necessary to insert a special `|dont_expand|' marker into \TeX's reading
//! mechanism.  This special marker is processed by |get_next|, but it does
//! not slow down the inner loop.
//!
//! Since \.{\\outer} macros might arise here, we must also
//! clear the |scanner_status| temporarily.
//
// @<Suppress expansion...@>=
pub(crate) macro Suppress_expansion_of_the_next_token($globals:expr) {{
    /// temporary storage of `scanner_status`
    let save_scanner_status;
    /// for list manipulation
    let p: pointer;
    /// token that is being "expanded after"
    let t;

    // begin save_scanner_status:=scanner_status; scanner_status:=normal;
    save_scanner_status = $globals.scanner_status;
    $globals.scanner_status = scanner_status_kind::normal;
    // get_token; scanner_status:=save_scanner_status; t:=cur_tok;
    get_token($globals)?;
    $globals.scanner_status = save_scanner_status;
    t = $globals.cur_tok;
    // back_input; {now |start| and |loc| point to the backed-up token |t|}
    back_input($globals);
    /// now `start` and `loc` point to the backed-up token `t`
    const _: () = ();
    // if t>=cs_token_flag then
    if t.get_cs().is_some() {
        // begin p:=get_avail; info(p):=cs_token_flag+frozen_dont_expand;
        p = get_avail($globals);
        info_tok_assign!($globals, p, cur_tok_type::from_cs(frozen_dont_expand as _));
        // link(p):=loc; start:=p; loc:=p;
        link!($globals, p) = loc!($globals);
        start!($globals) = p;
        loc!($globals) = p;
        // end;
    }
    // end
    use crate::section_0036::loc;
    use crate::section_0115::pointer;
    use crate::section_0118::info_tok_assign;
    use crate::section_0118::link;
    use crate::section_0120::get_avail;
    use crate::section_0222::frozen_dont_expand;
    use crate::section_0297::cur_tok_type;
    use crate::section_0302::start;
    use crate::section_0305::scanner_status_kind;
    use crate::section_0325::back_input;
    use crate::section_0365::get_token;
}}
