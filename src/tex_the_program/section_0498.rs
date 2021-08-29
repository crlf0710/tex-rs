//! @ A condition is started when the |expand| procedure encounters
//! an |if_test| command; in that case |expand| reduces to |conditional|,
//! which is a recursive procedure.
//! @^recursion@>
//
// @p procedure conditional;
#[allow(unused_variables)]
pub(crate) fn conditional(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit,common_ending;
    // var b:boolean; {is the condition true?}
    /// is the condition true?
    let b: boolean;
    // @!r:"<"..">"; {relation to be evaluated}
    // @!m,@!n:integer; {to be tested against the second operand}
    // @!p,@!q:pointer; {for traversing token lists in \.{\\ifx} tests}
    // @!save_scanner_status:small_number; {|scanner_status| upon entry}
    // @!save_cond_ptr:pointer; {|cond_ptr| corresponding to this conditional}
    /// `cond_ptr` corresponding to this conditional
    let save_cond_ptr: pointer;
    // @!this_if:small_number; {type of this conditional}
    /// type of this conditional
    let this_if: small_number;
    // begin @<Push the condition stack@>;@+save_cond_ptr:=cond_ptr;this_if:=cur_chr;@/
    Push_the_condition_stack!(globals);
    save_cond_ptr = globals.cond_ptr;
    this_if = (globals.cur_chr.get() as u8).into();
    region_forward_label!(
    |'common_ending|
    {
        // @<Either process \.{\\ifcase} or set |b| to the value of a boolean condition@>;
        Either_process_ifcase_or_set_b_to_the_value_of_a_boolean_condition!(globals, this_if, b, save_cond_ptr, 'common_ending);
        // if tracing_commands>1 then @<Display the value of |b|@>;
        if tracing_commands!(globals) > 1 {
            Display_the_value_of_b!(globals, b);
        }
        // if b then
        if b {
            // begin change_if_limit(else_code,save_cond_ptr);
            change_if_limit(globals, else_code.into(), save_cond_ptr);
            // return; {wait for \.{\\else} or \.{\\fi}}
            /// wait for `\else` or `\fi`
            return_nojump!();
            // end;
        }
        // @<Skip to \.{\\else} or \.{\\fi}, then |goto common_ending|@>;
        Skip_to_else_or_fi__then_goto_common_ending!(globals, save_cond_ptr, 'common_ending);
    }
    // common_ending: if cur_chr=fi_code then @<Pop the condition stack@>
    'common_ending <-
    );
    if globals.cur_chr.get() == fi_code as chr_code_repr {
        Pop_the_condition_stack!(globals);
    }
    // else if_limit:=fi_code; {wait for \.{\\fi}}
    else {
        /// wait for `\fi`
        const _ : () = ();
        globals.if_limit = fi_code.into();
    }
    // exit:end;
    ok_nojump!()
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0297::chr_code_repr;
use crate::section_0489::else_code;
use crate::section_0489::fi_code;
use crate::section_0497::change_if_limit;
