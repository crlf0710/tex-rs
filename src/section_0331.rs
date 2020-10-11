//! @ To get \TeX's whole input mechanism going, we perform the following
//! actions.
//
// @<Initialize the input routines@>=
macro_rules! Initialize_the_input_routines {
    ($globals:expr, $lbl_final_end:lifetime) => {
        // begin input_ptr:=0; max_in_stack:=0;
        // in_open:=0; open_parens:=0; max_buf_stack:=0;
        // param_ptr:=0; max_param_stack:=0;
        // first:=buf_size; repeat buffer[first]:=0; decr(first); until first=0;
        // scanner_status:=normal; warning_index:=null; first:=1;
        // state:=new_line; start:=1; index:=0; line:=0; name:=0;
        // force_eof:=false;
        // align_state:=1000000;@/
        // if not init_terminal then goto final_end;
        if !init_terminal($globals) {
            goto_forward_label!($lbl_final_end);
        }
        // limit:=last; first:=last+1; {|init_terminal| has set |loc| and |last|}
        // end
    };
}
