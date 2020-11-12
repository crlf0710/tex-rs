//! @ We use the fact that |register<advance<multiply<divide|.
//
// @<Declare subprocedures for |prefixed_command|@>=
// procedure do_register_command(@!a:small_number);
#[allow(unused_variables)]
pub(crate) fn do_register_command(globals: &mut TeXGlobals, a: small_number) {
    // label found,exit;
    // var l,@!q,@!r,@!s:pointer; {for list manipulation}
    // @!p:int_val..mu_val; {type of register involved}
    // begin q:=cur_cmd;
    // @<Compute the register location |l| and its type |p|; but |return| if invalid@>;
    // if q=register then scan_optional_equals
    // else if scan_keyword("by") then do_nothing; {optional `\.{by}'}
    // @.by@>
    // arith_error:=false;
    // if q<multiply then @<Compute result of |register| or
    //     |advance|, put it in |cur_val|@>
    // else @<Compute result of |multiply| or |divide|, put it in |cur_val|@>;
    // if arith_error then
    //   begin print_err("Arithmetic overflow");
    // @.Arithmetic overflow@>
    //   help2("I can't carry out that multiplication or division,")@/
    //     ("since the result is out of range.");
    //   if p>=glue_val then delete_glue_ref(cur_val);
    //   error; return;
    //   end;
    // if p<glue_val then word_define(l,cur_val)
    // else  begin trap_zero_glue; define(l,glue_ref,cur_val);
    //   end;
    // exit: end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::small_number;