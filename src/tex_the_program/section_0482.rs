//! @ The |read_toks| procedure constructs a token list like that for any
//! macro definition, and makes |cur_val| point to it. Parameter |r| points
//! to the control sequence that will receive this token list.
//
// @p procedure read_toks(@!n:integer;@!r:pointer);
pub(crate) fn read_toks(globals: &mut TeXGlobals, mut n: integer, r: pointer) -> TeXResult<()> {
    // label done;
    // var p:pointer; {tail of the token list}
    /// tail of the token list
    let mut p: pointer;
    // @!q:pointer; {new node being added to the token list via |store_new_token|}
    /// new node being added to the token list via `store_new_token`
    let mut q: pointer;
    // @!s:integer; {saved value of |align_state|}
    /// saved value of `align_state`
    let s: integer;
    // @!m:small_number; {stream number}
    /// stream number
    let m: small_number;
    // begin scanner_status:=defining; warning_index:=r;
    globals.scanner_status = scanner_status_kind::defining;
    globals.warning_index = r;
    // def_ref:=get_avail; token_ref_count(def_ref):=null;
    globals.def_ref = get_avail(globals);
    token_ref_count!(globals, globals.def_ref) = null;
    // p:=def_ref; {the reference count}
    /// the reference count
    const _: () = ();
    p = globals.def_ref;
    // store_new_token(end_match_token);
    store_new_token!(globals, end_match_token, p, q);
    // if (n<0)or(n>15) then m:=16@+else m:=n;
    if n < 0 || n > 15 {
        m = 16.into();
    } else {
        m = small_number::new(n as _);
    }
    // s:=align_state; align_state:=1000000; {disable tab marks, etc.}
    s = globals.align_state;
    globals.align_state = 1000000;
    /// disable tab marks, etc.
    const _: () = ();
    // repeat @<Input and store tokens from the next line of the file@>;
    loop {
        crate::section_0483::Input_and_store_tokens_from_the_next_line_of_the_file!(
            globals, m, p, q, n, r
        );
        // until align_state=1000000;
        if globals.align_state == 1000000 {
            break;
        }
    }
    // cur_val:=def_ref; scanner_status:=normal; align_state:=s;
    globals.cur_val = globals.def_ref as _;
    globals.scanner_status = scanner_status_kind::normal;
    globals.align_state = s;
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0120::get_avail;
use crate::section_0200::token_ref_count;
use crate::section_0289::end_match_token;
use crate::section_0305::scanner_status_kind;
use crate::section_0371::store_new_token;
