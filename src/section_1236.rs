//! @ We use the fact that |register<advance<multiply<divide|.
//
// @<Declare subprocedures for |prefixed_command|@>=
// procedure do_register_command(@!a:small_number);
#[allow(unused_variables)]
pub(crate) fn do_register_command(globals: &mut TeXGlobals, a: small_number) -> TeXResult<()> {
    // label found,exit;
    // var l,@!q,@!r,@!s:pointer; {for list manipulation}
    /// for list manipulation
    let l: pointer;
    let q: pointer;
    // @!p:int_val..mu_val; {type of register involved}
    /// type of register involved
    let p: cur_val_level_kind;
    // begin q:=cur_cmd;
    q = globals.cur_cmd as _;
    // @<Compute the register location |l| and its type |p|; but |return| if invalid@>;
    Compute_the_register_location_l_and_its_type_p_but_return_if_invalid!(globals, p, q, l);
    // if q=register then scan_optional_equals
    if q == register as pointer {
        scan_optional_equals(globals)?;
    }
    // else if scan_keyword("by") then do_nothing; {optional `\.{by}'}
    else if scan_keyword(globals, strpool_str!("by"))? {
        /// optional `by`
        do_nothing!();
    }
    // @.by@>
    // arith_error:=false;
    globals.arith_error = false;
    // if q<multiply then @<Compute result of |register| or
    //     |advance|, put it in |cur_val|@>
    if q < multiply as _ {
        Compute_result_of_register_or_advance_put_it_in_cur_val!(globals, l, p, q);
    }
    // else @<Compute result of |multiply| or |divide|, put it in |cur_val|@>;
    else {
        todo!("multiply or more");
    }
    // if arith_error then
    //   begin print_err("Arithmetic overflow");
    // @.Arithmetic overflow@>
    //   help2("I can't carry out that multiplication or division,")@/
    //     ("since the result is out of range.");
    //   if p>=glue_val then delete_glue_ref(cur_val);
    //   error; return;
    //   end;
    // if p<glue_val then word_define(l,cur_val)
    if p < glue_val {
        word_define!(globals, a, l, globals.cur_val);
    }
    // else  begin trap_zero_glue; define(l,glue_ref,cur_val);
    else {
        trap_zero_glue(globals);
        define!(globals, a, l, glue_ref, globals.cur_val as _);
        // end;
    }
    // exit: end;

    use cur_val_level_kind::int_val;
    use cur_val_level_kind::glue_val;
    use crate::section_0209::advance;
    use crate::section_0440::scan_int;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0209::register;
use crate::section_0209::multiply;
use crate::section_0210::glue_ref;
use crate::section_0405::scan_optional_equals;
use crate::section_0407::scan_keyword;
use crate::section_0410::cur_val_level_kind;
use crate::section_1229::trap_zero_glue;
