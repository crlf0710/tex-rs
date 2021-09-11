//! @* \[27] Building token lists.
//! The token lists for macros and for other things like \.{\\mark} and \.{\\output}
//! and \.{\\write} are produced by a procedure called |scan_toks|.
//!
//! Before we get into the details of |scan_toks|, let's consider a much
//! simpler task, that of converting the current string into a token list.
//! The |str_toks| function does this; it classifies spaces as type |spacer|
//! and everything else as type |other_char|.
//!
//! The token list created by |str_toks| begins at |link(temp_head)| and ends
//! at the value |p| that is returned. (If |p=temp_head|, the list is empty.)
//
// @p function str_toks(@!b:pool_pointer):pointer;
//   {converts |str_pool[b..pool_ptr-1]| to a token list}
/// converts `str_pool[b..pool_ptr-1]` to a token list
#[allow(unused_variables)]
pub(crate) fn str_toks(globals: &mut TeXGlobals, b: pool_pointer) -> pointer {
    // var p:pointer; {tail of the token list}
    /// tail of the token list
    let mut p: pointer;
    // @!q:pointer; {new node being added to the token list via |store_new_token|}
    /// new node being added to the token list via `store_new_token`
    let mut q: pointer;
    // @!t:halfword; {token being appended}
    // @!k:pool_pointer; {index into |str_pool|}
    // begin str_room(1);
    str_room(globals, 1 * character_max_room);
    // p:=temp_head; link(p):=null; k:=b;
    p = temp_head;
    link!(globals, p) = null;
    #[cfg(not(feature = "unicode_support"))]
    {
        /// index into `str_pool`
        let k: pool_pointer;
        k = b;
        // while k<pool_ptr do
        while k < globals.pool_ptr {
            todo!("str_toks");
            // begin t:=so(str_pool[k]);
            // if t=" " then t:=space_token
            // else t:=other_token+t;
            // fast_store_new_token(t);
            // incr(k);
            // end;
        }
    }
    #[cfg(feature = "unicode_support")]
    {
        let range = b..globals.pool_ptr;
        let ascii_codes = globals
            .str_pool
            .slice_ascii_codes(range)
            .collect::<Vec<_>>();
        for t in ascii_codes {
            let t: ASCII_code = xord(t);
            let tok;
            if t == ASCII_code_literal!(b' ') {
                tok = space_token;
            } else {
                tok = other_token + t.numeric_value();
            }
            fast_store_new_token!(globals, cur_tok_type::new(tok), p, q);
        }
    }
    let str_toks;
    // pool_ptr:=b; str_toks:=p;
    globals.pool_ptr = b;
    str_toks = p;
    // end;
    str_toks
}

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0018::ASCII_code_literal;
use crate::section_0020::xord;
use crate::section_0038::pool_pointer;
use crate::section_0042::character_max_room;
use crate::section_0042::str_room;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0162::temp_head;
use crate::section_0289::other_token;
use crate::section_0289::space_token;
use crate::section_0297::cur_tok_type;
use crate::section_0371::fast_store_new_token;
