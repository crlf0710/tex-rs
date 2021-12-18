//! ` `
// @<Declare action...@>=
// procedure insert_dollar_sign;
#[allow(unused_variables)]
pub(crate) fn insert_dollar_sign(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin back_input; cur_tok:=math_shift_token+"$";
    back_input(globals);
    globals.cur_tok = cur_tok_type::new(math_shift_token + b'$' as cur_tok_repr);
    // print_err("Missing $ inserted");
    print_err!(globals, crate::strpool_str!("Missing $ inserted"));
    // @.Missing \$ inserted@>
    // help2("I've inserted a begin-math/end-math symbol since I think")@/
    // ("you left one out. Proceed, with fingers crossed."); ins_error;
    help2!(
        globals,
        crate::strpool_str!("I've inserted a begin-math/end-math symbol since I think"),
        crate::strpool_str!("you left one out. Proceed, with fingers crossed.")
    );
    ins_error(globals)?;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0073::print_err;
use crate::section_0079::help2;
use crate::section_0081::TeXResult;
use crate::section_0207::math_shift;
use crate::section_0289::math_shift_token;
use crate::section_0297::cur_tok_repr;
use crate::section_0297::cur_tok_type;
use crate::section_0325::back_input;
use crate::section_0327::ins_error;
