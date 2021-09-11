//! @ The procedure |conv_toks| uses |str_toks| to insert the token list
//! for |convert| functions into the scanner; `\.{\\outer}' control sequences
//! are allowed to follow `\.{\\string}' and `\.{\\meaning}'.
//
// @p procedure conv_toks;
pub(crate) fn conv_toks(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var old_setting:0..max_selector; {holds |selector| setting}
    /// holds `selector` setting
    let old_setting;
    // @!c:number_code..job_name_code; {desired type of conversion}
    /// desired type of conversion
    let c: convert_code_kind;
    // @!save_scanner_status:small_number; {|scanner_status| upon entry}
    /// `scanner_status` upon entry
    let save_scanner_status;
    // @!b:pool_pointer; {base of temporary string}
    /// base of temporary string
    let b: pool_pointer;
    // begin c:=cur_chr; @<Scan the argument for command |c|@>;
    c = (globals.cur_chr.get() as u8).into();
    crate::section_0471::Scan_the_argument_for_command_c!(globals, c, save_scanner_status);
    // old_setting:=selector; selector:=new_string; b:=pool_ptr;
    old_setting = globals.selector;
    globals.selector = new_string.into();
    b = globals.pool_ptr;
    // @<Print the result of command |c|@>;
    crate::section_0472::Print_the_result_of_command_c!(globals, c);
    // selector:=old_setting; link(garbage):=str_toks(b); ins_list(link(temp_head));
    globals.selector = old_setting;
    link!(globals, garbage) = str_toks(globals, b);
    ins_list!(globals, link!(globals, temp_head));
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::pool_pointer;
use crate::section_0054::new_string;
use crate::section_0081::TeXResult;
use crate::section_0118::link;
use crate::section_0162::garbage;
use crate::section_0162::temp_head;
use crate::section_0323::ins_list;
use crate::section_0464::str_toks;
use crate::section_0468::convert_code_kind;
