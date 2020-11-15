//! @ A condition is started when the |expand| procedure encounters
//! an |if_test| command; in that case |expand| reduces to |conditional|,
//! which is a recursive procedure.
//! @^recursion@>
//
// @p procedure conditional;
#[allow(unused_variables)]
pub(crate) fn conditional(globals: &mut TeXGlobals) {
    // label exit,common_ending;
    // var b:boolean; {is the condition true?}
    // @!r:"<"..">"; {relation to be evaluated}
    // @!m,@!n:integer; {to be tested against the second operand}
    // @!p,@!q:pointer; {for traversing token lists in \.{\\ifx} tests}
    // @!save_scanner_status:small_number; {|scanner_status| upon entry}
    // @!save_cond_ptr:pointer; {|cond_ptr| corresponding to this conditional}
    // @!this_if:small_number; {type of this conditional}
    // begin @<Push the condition stack@>;@+save_cond_ptr:=cond_ptr;this_if:=cur_chr;@/
    // @<Either process \.{\\ifcase} or set |b| to the value of a boolean condition@>;
    // if tracing_commands>1 then @<Display the value of |b|@>;
    // if b then
    //   begin change_if_limit(else_code,save_cond_ptr);
    //   return; {wait for \.{\\else} or \.{\\fi}}
    //   end;
    // @<Skip to \.{\\else} or \.{\\fi}, then |goto common_ending|@>;
    // common_ending: if cur_chr=fi_code then @<Pop the condition stack@>
    // else if_limit:=fi_code; {wait for \.{\\fi}}
    // exit:end;
}

use crate::section_0004::TeXGlobals;
