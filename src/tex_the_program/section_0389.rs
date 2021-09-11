//! @ After parameter scanning is complete, the parameters are moved to the
//! |param_stack|. Then the macro body is fed to the scanner; in other words,
//! |macro_call| places the defined text of the control sequence at the
//! top of\/ \TeX's input stack, so that |get_next| will proceed to read it
//! next.
//!
//! The global variable |cur_cs| contains the |eqtb| address of the control sequence
//! being expanded, when |macro_call| begins. If this control sequence has not been
//! declared \.{\\long}, i.e., if its command code in the |eq_type| field is
//! not |long_call| or |long_outer_call|, its parameters are not allowed to contain
//! the control sequence \.{\\par}. If an illegal \.{\\par} appears, the macro
//! call is aborted, and the \.{\\par} will be rescanned.

//
// @<Declare the procedure called |macro_call|@>=
// procedure macro_call; {invokes a user-defined control sequence}
/// invokes a user-defined control sequence
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn macro_call(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit, continue, done, done1, found;
    // var r:pointer; {current node in the macro's token list}
    /// current node in the macro's token list
    let mut r: pointer;
    // @!p:pointer; {current node in parameter token list being built}
    /// current node in parameter token list being built
    let mut p: pointer;
    // @!q:pointer; {new node being put into the token list}
    /// new node being put into the token list
    let mut q: pointer;
    // @!s:pointer; {backup pointer for parameter matching}
    const _: () = ();
    // @!t:pointer; {cycle pointer for backup recovery}
    // @!u,@!v:pointer; {auxiliary pointers for backup recovery}
    // @!rbrace_ptr:pointer; {one step before the last |right_brace| token}
    const _: () = ();
    // @!n:small_number; {the number of parameters scanned}
    /// the number of parameters scanned
    let mut n: small_number;
    // @!unbalance:halfword; {unmatched left braces in current parameter}
    const _: () = ();
    // @!m:halfword; {the number of tokens or groups (usually)}
    /// the number of tokens or groups (usually)
    let mut m: halfword;
    // @!ref_count:pointer; {start of the token list}
    /// start of the token list
    let ref_count: pointer;
    // @!save_scanner_status:small_number; {|scanner_status| upon entry}
    /// `scanner_status` upon entry
    let save_scanner_status: scanner_status_kind;
    // @!save_warning_index:pointer; {|warning_index| upon entry}
    /// `warning_index` upon entry
    let save_warning_index: pointer;
    // @!match_chr:ASCII_code; {character used in parameter}
    /// character used in parameter
    let mut match_chr: ASCII_code;

    /// Actually useless: These values will be overwriten before they participate.
    {
        m = 0;
        p = 0;
        match_chr = ASCII_code::default();
    }
    // begin save_scanner_status:=scanner_status; save_warning_index:=warning_index;
    save_scanner_status = globals.scanner_status;
    save_warning_index = globals.warning_index;
    // warning_index:=cur_cs; ref_count:=cur_chr; r:=link(ref_count); n:=0;
    globals.warning_index = globals.cur_cs;
    ref_count = globals.cur_chr.get() as _;
    r = link!(globals, ref_count);
    n = small_number::new(0);
    // if tracing_macros>0 then @<Show the text of the macro being expanded@>;
    if tracing_macros!(globals) > 0 {
        crate::section_0401::Show_the_text_of_the_macro_being_expanded!(globals, ref_count);
    }
    let mut info_r = info_tok!(globals, r);
    // if info(r)<>end_match_token then
    if info_r != end_match_token {
        // @<Scan the parameters and make |link(r)| point to the macro body; but
        //   |return| if an illegal \.{\\par} is detected@>;
        crate::section_0391::Scan_the_parameters_and_make_link_r_point_to_the_macro_body__but_return_if_an_illegal_par_is_detected!(
            globals, match_chr, r, info_r, m, n, p, q
        );
    }
    // @<Feed the macro body and its parameters to the scanner@>;
    crate::section_0390::Feed_the_macro_body_and_its_parameters_to_the_scanner!(
        globals, ref_count, r, n
    );
    // exit:scanner_status:=save_scanner_status; warning_index:=save_warning_index;
    globals.scanner_status = save_scanner_status;
    globals.warning_index = save_warning_index;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0113::halfword;
use crate::section_0115::pointer;
use crate::section_0118::info_tok;
use crate::section_0118::link;
use crate::section_0236::tracing_macros;
use crate::section_0289::end_match_token;
use crate::section_0305::scanner_status_kind;
