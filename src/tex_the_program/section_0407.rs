//! @ In case you are getting bored, here is a slightly less trivial routine:
//! Given a string of lowercase letters, like `\.{pt}' or `\.{plus}' or
//! `\.{width}', the |scan_keyword| routine checks to see whether the next
//! tokens of input match this string. The match must be exact, except that
//! uppercase letters will match their lowercase counterparts; uppercase
//! equivalents are determined by subtracting |"a"-"A"|, rather than using the
//! |uc_code| table, since \TeX\ uses this routine only for its own limited
//! set of keywords.
//!
//! If a match is found, the characters are effectively removed from the input
//! and |true| is returned. Otherwise |false| is returned, and the input
//! is left essentially unchanged (except for the fact that some macros
//! may have been expanded, etc.).
//! @^inner loop@>
//
// @p function scan_keyword(@!s:str_number):boolean; {look for a given string}
/// look for a given string
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn scan_keyword(globals: &mut TeXGlobals, s: str_number) -> TeXResult<boolean> {
    // label exit;
    // var p:pointer; {tail of the backup list}
    /// tail of the backup list
    let mut p: pointer;
    // @!q:pointer; {new node being added to the token list via |store_new_token|}
    /// new node being added to the token list via `store_new_token`
    let mut q: pointer;
    // @!k:pool_pointer; {index into |str_pool|}
    /// index into `str_pool`
    #[cfg(not(feature = "unicode_support"))]
    let k: pool_pointer;
    // begin p:=backup_head; link(p):=null; k:=str_start[s];
    p = backup_head;
    link!(globals, p) = null;
    #[cfg(not(feature = "unicode_support"))]
    {
        k = globals.str_start[s];
    }
    #[cfg(not(feature = "unicode_support"))]
    let u_count = (globals.str_start[s + 1] - k) as usize;
    #[cfg(feature = "unicode_support")]
    let u_count = globals
        .str_pool
        .str_ascii_codes(&globals.str_start, s)
        .count();
    let mut u = 0;
    // while k<str_start[s+1] do
    while u < u_count {
        // begin get_x_token; {recursion is possible here}
        /// recursion is possible here
        get_x_token(globals)?;
        // @^recursion@>
        #[cfg(not(feature = "unicode_support"))]
        let s_k1 = so(globals.str_pool[k]);
        #[cfg(feature = "unicode_support")]
        let s_k1 = xord(
            globals
                .str_pool
                .str_ascii_codes(&globals.str_start, s)
                .nth(u)
                .unwrap(),
        );
        let s_k2 = if s_k1 >= ASCII_code_literal!(b'a') && s_k1 <= ASCII_code_literal!(b'z') {
            ASCII_code::from(s_k1.numeric_value() as integer - b'a' as integer + b'A' as integer)
        } else {
            s_k1
        };
        // if (cur_cs=0)and@|
        //  ((cur_chr=so(str_pool[k]))or(cur_chr=so(str_pool[k])-"a"+"A")) then
        if globals.cur_cs == 0 && (globals.cur_chr == s_k1.into() || globals.cur_chr == s_k2.into())
        {
            // begin store_new_token(cur_tok); incr(k);
            store_new_token!(globals, globals.cur_tok.get(), p, q);
            #[cfg(not(feature = "unicode_support"))]
            incr!(k);
            incr!(u);
            // end
        }
        // else if (cur_cmd<>spacer)or(p<>backup_head) then
        else if globals.cur_cmd != spacer || p != backup_head {
            // begin back_input;
            back_input(globals);
            // if p<>backup_head then back_list(link(backup_head));
            if p != backup_head {
                back_list!(globals, link!(globals, backup_head));
            }
            // scan_keyword:=false; return;
            crate::return_nojump!(false);
            // end;
        }
        // end;
    }
    // flush_list(link(backup_head)); scan_keyword:=true;
    // exit:end;
    flush_list(globals, link!(globals, backup_head));
    crate::ok_nojump!(true)
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code;
use crate::section_0018::ASCII_code_literal;
use crate::section_0020::xchr;
use crate::section_0020::xord;
use crate::section_0038::pool_pointer;
use crate::section_0038::str_number;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0123::flush_list;
use crate::section_0162::backup_head;
use crate::section_0207::spacer;
use crate::section_0323::back_list;
use crate::section_0325::back_input;
use crate::section_0371::store_new_token;
use crate::section_0380::get_x_token;
