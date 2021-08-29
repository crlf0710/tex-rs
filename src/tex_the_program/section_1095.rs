//! ` `
// @<Declare act...@>=
// procedure head_for_vmode;
pub(crate) fn head_for_vmode(globals: &mut TeXGlobals) {
    // begin if mode<0 then
    if mode!(globals) < 0 {
        todo!("head_for 1");
        //   if cur_cmd<>hrule then off_save
        //   else  begin print_err("You can't use `");
        //     print_esc("hrule"); print("' here except with leaders");
        // @.You can't use \\hrule...@>
        //     help2("To put a horizontal rule in an hbox or an alignment,")@/
        //       ("you should use \leaders or \hrulefill (see The TeXbook).");
        //     error;
        //     end
    }
    // else  begin back_input; cur_tok:=par_token; back_input; token_type:=inserted;
    else {
        back_input(globals);
        globals.cur_tok = globals.par_token;
        back_input(globals);
        token_type!(globals) = inserted;
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0307::inserted;
use crate::section_0325::back_input;