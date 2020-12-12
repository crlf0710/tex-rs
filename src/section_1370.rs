//! ` `
// @<Declare procedures needed in |hlist_out|, |vlist_out|@>=
// procedure write_out(@!p:pointer);
pub(crate) fn write_out(globals: &mut TeXGlobals, p: pointer) -> TeXResult<()> {
    // var old_setting:0..max_selector; {holds print |selector|}
    /// holds print `selector`
    let old_settings;
    // @!old_mode:integer; {saved |mode|}
    // @!j:small_number; {write stream number}
    /// write stream number
    let j: small_number;
    // @!q,@!r:pointer; {temporary variables for list manipulation}
    // begin @<Expand macros in the token list
    //   and make |link(def_ref)| point to the result@>;
    Expand_macros_in_the_token_list_and_make_link_def_ref_point_to_the_result!(globals, p);
    // old_setting:=selector; j:=write_stream(p);
    old_settings = globals.selector;
    j = small_number::new(write_stream!(globals, p) as _);
    // if write_open[j] then selector:=j
    if globals.write_open[j.get()] {
        globals.selector = j.get().into();
    }
    // else  begin {write to the terminal if file isn't open}
    else {
        /// write to the terminal if file isn't open
        const _ : () = ();
        // if (j=17)and(selector=term_and_log) then selector:=log_only;
        if j == 17 && globals.selector == term_and_log {
            globals.selector = log_only.into();
        }
        // print_nl("");
        print_nl(globals, strpool_str!(""));
        // end;
    }
    // token_show(def_ref); print_ln;
    token_show(globals, globals.def_ref);
    print_ln(make_globals_io_view!(globals));
    // flush_list(def_ref); selector:=old_setting;
    flush_list(globals, globals.def_ref);
    globals.selector = old_settings;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0054::term_and_log;
use crate::section_0054::log_only;
use crate::section_0057::print_ln;
use crate::section_0062::print_nl;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0123::flush_list;
use crate::section_0295::token_show;
