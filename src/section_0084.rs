//! @ It is desirable to provide an `\.E' option here that gives the user
//! an easy way to return from \TeX\ to the system editor, with the offending
//! line ready to be edited. But such an extension requires some system
//! wizardry, so the present implementation simply types out the name of the
//! file that should be
//! edited and the relevant line number.
//! @^system dependencies@>
//!
//! There is a secret `\.D' option available when the debugging routines haven't
//! been commented~out.
//! @^debugging@>
//
// @<Interpret code |c| and |return| if done@>=
macro_rules! Interpret_code_c_and_return_if_done {
    ($globals:expr, $c:expr, $lbl_continue:lifetime) => {{
        #[cfg(feature = "debugging")]
        const DEBUGGING_ENABLED: bool = true;
        #[cfg(not(feature = "debugging"))]
        const DEBUGGING_ENABLED: bool = false;
        // case c of
        if false {
            unreachable!()
        }
        // "0","1","2","3","4","5","6","7","8","9": if deletions_allowed then
        //   @<Delete \(c)|c-"0"| tokens and |goto continue|@>;
        // @t\4\4@>@;@+@!debug "D": begin debug_help; goto continue;@+end;@+gubed@/
        else if DEBUGGING_ENABLED && $c == ASCII_code_literal!(b'D') {
            region_debug! {
                debug_help($globals);
                goto_backward_label!($lbl_continue);

                use crate::section_1338::debug_help;
            }
        }
        // "E": if base_ptr>0 then
        //   begin print_nl("You want to edit file ");
        // @.You want to edit file x@>
        //   slow_print(input_stack[base_ptr].name_field);
        //   print(" at line "); print_int(line);
        //   interaction:=scroll_mode; jump_out;
        //   end;
        // "H": @<Print the help information and |goto continue|@>;
        // "I":@<Introduce new material from the terminal and |return|@>;
        // "Q","R","S":@<Change the interaction level and |return|@>;
        // "X":begin interaction:=scroll_mode; jump_out;
        else if $c == ASCII_code_literal!(b'X') {
            $globals.interaction = scroll_mode.into();
            jump_out()?;
            // end;
        }
        // othercases do_nothing
        else {
            do_nothing!();
        }
        // endcases;@/
        // @<Print the menu of available options@>
        use crate::section_0073::scroll_mode;
    }}
}