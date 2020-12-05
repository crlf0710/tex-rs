//! ` `

// @<Declare act...@>=
// procedure issue_message;
#[allow(unused_variables)]
pub(crate) fn issue_message(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var old_setting:0..max_selector; {holds |selector| setting}
    /// holds `selector` setting
    let old_setting;
    // @!c:0..1; {identifies \.{\\message} and \.{\\errmessage}}
    /// identifies `\message` and `\errmessage`
    let c;
    // @!s:str_number; {the message}
    /// the message
    let s: str_number;
    // begin c:=cur_chr; link(garbage):=scan_toks(false,true);
    c = globals.cur_chr.get();
    link!(globals, garbage) = scan_toks(globals, false, true)?;
    // old_setting:=selector; selector:=new_string;
    old_setting = globals.selector;
    globals.selector = new_string.into();
    // token_show(def_ref); selector:=old_setting;
    token_show(globals, globals.def_ref);
    globals.selector = old_setting;
    // flush_list(def_ref);
    // str_room(1); s:=make_string;
    str_room(globals, 1);
    s = make_string(make_globals_string_view!(globals));
    // if c=0 then @<Print string |s| on the terminal@>
    if c == 0 {
        Print_string_s_on_the_terminal!(globals, s);
    }
    // else @<Print string |s| as an error message@>;
    else {
        Print_string_s_as_an_error_message!(globals, s.get() as _);
    }
    // flush_string;
    flush_string(globals);
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsStringView;
use crate::section_0054::new_string;
use crate::section_0038::str_number;
use crate::section_0042::str_room;
use crate::section_0043::make_string;
use crate::section_0044::flush_string;
use crate::section_0081::TeXResult;
use crate::section_0162::garbage;
use crate::section_0295::token_show;
use crate::section_0473::scan_toks;