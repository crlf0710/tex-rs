//! @ Note that `\.{\\ifx}' will declare two macros different if one is \\{long}
//! or \\{outer} and the other isn't, even though the texts of the macros are
//! the same.
//!
//! We need to reset |scanner_status|, since \.{\\outer} control sequences
//! are allowed, but we might be scanning a macro definition or preamble.
//
// @<Test if two tokens match@>=
pub(crate) macro Test_if_two_tokens_match($globals:expr, $b:expr) {{
    /// to be tested against the second operand
    let (n, p, q);

    /// `scanner_status` upon entry
    let save_scanner_status;
    // begin save_scanner_status:=scanner_status; scanner_status:=normal;
    save_scanner_status = $globals.scanner_status;
    $globals.scanner_status = scanner_status_kind::normal;
    // get_next; n:=cur_cs; p:=cur_cmd; q:=cur_chr;
    get_next($globals)?;
    n = $globals.cur_cs;
    p = $globals.cur_cmd;
    q = $globals.cur_chr;
    // get_next; if cur_cmd<>p then b:=false
    get_next($globals)?;
    if $globals.cur_cmd != p {
        $b = false;
    }
    // else if cur_cmd<call then b:=(cur_chr=q)
    else if $globals.cur_cmd < call {
        $b = $globals.cur_chr == q;
    }
    // else @<Test if two macro texts match@>;
    else {
        crate::section_0508::Test_if_two_macro_texts_match!($globals, $b, n);
    }
    // scanner_status:=save_scanner_status;
    $globals.scanner_status = save_scanner_status;
    // end
    use crate::section_0210::call;
    use crate::section_0305::scanner_status_kind;
    use crate::section_0341::get_next;
}}
