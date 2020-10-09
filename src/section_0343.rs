// @ @<Input from external file, |goto restart| if no input found@>=
// @^inner loop@>
macro_rules! Input_from_external_file__goto_restart_if_no_input_found {
    ($globals:expr) => {
        // begin switch: if loc<=limit then {current line not yet finished}
        //   begin cur_chr:=buffer[loc]; incr(loc);
        //   reswitch: cur_cmd:=cat_code(cur_chr);
        //   @<Change state if necessary, and |goto switch| if the
        //     current character should be ignored,
        //     or |goto reswitch| if the current character
        //     changes to another@>;
        //   end
        // else  begin state:=new_line;@/
        //   @<Move to next line of file,
        //     or |goto restart| if there is no next line,
        //     or |return| if a \.{\\read} line has finished@>;
        //   check_interrupt;
        //   goto switch;
        //   end;
        // end
        //
    }
}
