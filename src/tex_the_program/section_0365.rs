//! @ No new control sequences will be defined except during a call of
//! |get_token|, or when \.{\\csname} compresses a token list, because
//! |no_new_control_sequence| is always |true| at other times.
//
// @p procedure get_token; {sets |cur_cmd|, |cur_chr|, |cur_tok|}
/// sets `cur_cmd`, `cur_chr`, `cur_tok`
pub(crate) fn get_token(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin no_new_control_sequence:=false; get_next; no_new_control_sequence:=true;
    globals.no_new_control_sequence = false;
    get_next(globals)?;
    globals.no_new_control_sequence = true;
    // @^inner loop@>
    // if cur_cs=0 then cur_tok:=(cur_cmd*@'400)+cur_chr
    if globals.cur_cs == 0 {
        globals.cur_tok = cur_tok_type::from_cmd_and_chr(globals.cur_cmd, globals.cur_chr);
    }
    // else cur_tok:=cs_token_flag+cur_cs;
    else {
        globals.cur_tok = cur_tok_type::from_cs(globals.cur_cs);
    }
    // end;
    crate::return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0297::cur_tok_type;
use crate::section_0341::get_next;
