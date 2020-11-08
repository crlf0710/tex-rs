//! @ The |get_x_token| procedure is equivalent to two consecutive
//! procedure calls: |get_next; x_token|.
//
// @p procedure x_token; {|get_x_token| without the initial |get_next|}
/// `get_x_token` without the initial `get_next`
pub(crate) fn x_token(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // begin while cur_cmd>max_command do
    while globals.cur_cmd > max_command {
        // begin expand;
        expand(globals);
        // get_next;
        get_next(globals)?;
        // end;
    }
    // if cur_cs=0 then cur_tok:=(cur_cmd*@'400)+cur_chr
    if globals.cur_cs == 0 {
        globals.cur_tok = cur_tok_type::from_cmd_and_chr(globals.cur_cmd, globals.cur_chr);
    }
    // else cur_tok:=cs_token_flag+cur_cs;
    else {
        globals.cur_tok = cur_tok_type::from_cs(globals.cur_cs);
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
use crate::section_0209::max_command;
use crate::section_0297::cur_tok_type;
use crate::section_0341::get_next;
use crate::section_0366::expand;
