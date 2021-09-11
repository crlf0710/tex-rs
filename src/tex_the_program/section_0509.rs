//! ` `

// @<Select the appropriate case and |return| or |goto common_ending|@>=
pub(crate) macro Select_the_appropriate_case_and_return_or_goto_common_ending($globals:expr, $save_cond_ptr:expr, $lbl_common_ending:lifetime) {{
    /// to be tested against the second operand
    let mut n: integer;

    // begin scan_int; n:=cur_val; {|n| is the number of cases to pass}
    scan_int($globals)?;
    /// `n` is the number of cases to pass
    const _: () = ();
    n = $globals.cur_val;
    // if tracing_commands>1 then
    if tracing_commands!($globals) > 1 {
        // begin begin_diagnostic; print("{case "); print_int(n); print_char("}");
        begin_diagnostic($globals);
        print($globals, crate::strpool_str!("{case ").get() as _);
        print_int($globals, n);
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'}'),
        );
        // end_diagnostic(false);
        end_diagnostic($globals, false);
        // end;
    }
    // while n<>0 do
    while n != 0 {
        // begin pass_text;
        pass_text($globals)?;
        // if cond_ptr=save_cond_ptr then
        if $globals.cond_ptr == $save_cond_ptr {
            // if cur_chr=or_code then decr(n)
            if $globals.cur_chr.get() == or_code as chr_code_repr {
                decr!(n);
            }
            // else goto common_ending
            else {
                crate::goto_forward_label!($lbl_common_ending);
            }
        }
        // else if cur_chr=fi_code then @<Pop the condition stack@>;
        else if $globals.cur_chr.get() == fi_code as chr_code_repr {
            crate::section_0496::Pop_the_condition_stack!($globals);
        }
        // end;
    }
    // change_if_limit(or_code,save_cond_ptr);
    change_if_limit($globals, or_code.into(), $save_cond_ptr);
    // return; {wait for \.{\\or}, \.{\\else}, or \.{\\fi}}
    /// wait for `\or`, `\else`, or `\fi`
    crate::return_nojump!();
    // end
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0016::decr;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0065::print_int;
    use crate::section_0236::tracing_commands;
    use crate::section_0245::begin_diagnostic;
    use crate::section_0245::end_diagnostic;
    use crate::section_0297::chr_code_repr;
    use crate::section_0440::scan_int;
    use crate::section_0489::fi_code;
    use crate::section_0489::or_code;
    use crate::section_0494::pass_text;
    use crate::section_0497::change_if_limit;
}}
