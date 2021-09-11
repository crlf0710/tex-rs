//! @ Now we can't postpone the difficulties any longer; we must bravely tackle
//! |scan_toks|. This function returns a pointer to the tail of a new token
//! list, and it also makes |def_ref| point to the reference count at the
//! head of that list.
//!
//! There are two boolean parameters, |macro_def| and |xpand|. If |macro_def|
//! is true, the goal is to create the token list for a macro definition;
//! otherwise the goal is to create the token list for some other \TeX\
//! primitive: \.{\\mark}, \.{\\output}, \.{\\everypar}, \.{\\lowercase},
//! \.{\\uppercase}, \.{\\message}, \.{\\errmessage}, \.{\\write}, or
//! \.{\\special}. In the latter cases a left brace must be scanned next; this
//! left brace will not be part of the token list, nor will the matching right
//! brace that comes at the end. If |xpand| is false, the token list will
//! simply be copied from the input using |get_token|. Otherwise all expandable
//! tokens will be expanded until unexpandable tokens are left, except that
//! the results of expanding `\.{\\the}' are not expanded further.
//! If both |macro_def| and |xpand| are true, the expansion applies
//! only to the macro body (i.e., to the material following the first
//! |left_brace| character).
//!
//! The value of |cur_cs| when |scan_toks| begins should be the |eqtb|
//! address of the control sequence to display in ``runaway'' error
//! messages.
//
// @p function scan_toks(@!macro_def,@!xpand:boolean):pointer;
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn scan_toks(
    globals: &mut TeXGlobals,
    macro_def: boolean,
    xpand: boolean,
) -> TeXResult<pointer> {
    // label found,continue,done,done1,done2;
    // var t:halfword; {token representing the highest parameter number}
    /// token representing the highest parameter number
    let mut t: cur_tok_repr;
    // @!s:halfword; {saved token}
    // @!p:pointer; {tail of the token list being built}
    /// tail of the token list being built
    let mut p: pointer;
    // @!q:pointer; {new node being added to the token list via |store_new_token|}
    /// new node being added to the token list via `store_new_token`
    let mut q: pointer;
    // @!unbalance:halfword; {number of unmatched left braces}
    /// number of unmatched left braces
    let mut unbalance: halfword;
    // @!hash_brace:halfword; {possible `\.{\#\{}' token}
    /// possible `#{` token
    let mut hash_brace: cur_tok_repr;
    // begin if macro_def then scanner_status:=defining
    // @+else scanner_status:=absorbing;
    // warning_index:=cur_cs; def_ref:=get_avail; token_ref_count(def_ref):=null;
    globals.warning_index = globals.cur_cs;
    globals.def_ref = get_avail(globals);
    token_ref_count!(globals, globals.def_ref) = null;
    // p:=def_ref; hash_brace:=0; t:=zero_token;
    p = globals.def_ref;
    hash_brace = 0;
    t = zero_token as _;
    crate::trace_expr!("t = zero_token = {}", t);
    // if macro_def then @<Scan and build the parameter part of the macro definition@>
    if macro_def {
        crate::section_0474::Scan_and_build_the_parameter_part_of_the_macro_definition!(
            globals, t, hash_brace, p, q
        );
    }
    // else scan_left_brace; {remove the compulsory left brace}
    else {
        /// remove the compulsory left brace
        scan_left_brace(globals)?;
    }
    crate::region_forward_label! {
    |'found|
    {
        // @<Scan and build the body of the token list; |goto found| when finished@>;
        crate::section_0477::Scan_and_build_the_body_of_the_token_list__goto_found_when_finished!(globals,
            macro_def, xpand, unbalance, t, p, q, 'found);
    }
    // found: scanner_status:=normal;
    'found <-
    }
    globals.scanner_status = scanner_status_kind::normal;
    // if hash_brace<>0 then store_new_token(hash_brace);
    if hash_brace != 0 {
        store_new_token!(globals, hash_brace, p, q);
    }
    // scan_toks:=p;
    crate::return_nojump!(p);
    // end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::halfword;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0120::get_avail;
use crate::section_0200::token_ref_count;
use crate::section_0297::cur_tok_repr;
use crate::section_0305::scanner_status_kind;
use crate::section_0371::store_new_token;
use crate::section_0380::get_x_token;
use crate::section_0403::scan_left_brace;
use crate::section_0445::zero_token;
