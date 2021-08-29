//! ` `

// @<Declare act...@>=
// procedure shift_case;
pub(crate) fn shift_case(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var b:pointer; {|lc_code_base| or |uc_code_base|}
    /// `lc_code_base` or `uc_code_base`
    let b: pointer;
    // @!p:pointer; {runs through the token list}
    /// runs through the token list
    let mut p: pointer;
    // @!t:halfword; {token}
    // @!c:eight_bits; {character code}
    // begin b:=cur_chr; p:=scan_toks(false,false); p:=link(def_ref);
    b = globals.cur_chr.get() as pointer;
    let _ = scan_toks(globals, false, false)?;
    p = link!(globals, globals.def_ref);
    // while p<>null do
    while p != null {
        // begin @<Change the case of the token in |p|, if a change is appropriate@>;
        Change_the_case_of_the_token_in_p_if_a_change_is_appropriate!(globals, p, b);
        // p:=link(p);
        p = link!(globals, p);
        // end;
    }
    // back_list(link(def_ref)); free_avail(def_ref); {omit reference count}
    /// omit reference count
    back_list!(globals, link!(globals, globals.def_ref));
    free_avail!(globals, globals.def_ref);
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0473::scan_toks;
