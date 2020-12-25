//! @ An |end_template| command is effectively changed to an |endv| command
//! by the following code. (The reason for this is discussed below; the
//! |frozen_end_template| at the end of the template has passed the
//! |check_outer_validity| test, so its mission of error detection has been
//! accomplished.)
//!
//! @<Insert a token containing |frozen_endv|@>=
//! begin cur_tok:=cs_token_flag+frozen_endv; back_input;
//! end
//!
//! @ The processing of \.{\\input} involves the |start_input| subroutine,
//! which will be declared later; the processing of \.{\\endinput} is trivial.
//!
//! @<Put each...@>=
//! primitive("input",input,0);@/
//! @!@:input_}{\.{\\input} primitive@>
//! primitive("endinput",input,1);@/
//! @!@:end_input_}{\.{\\endinput} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! input: if chr_code=0 then print_esc("input")@+else print_esc("endinput");
//!
//! @ @<Initiate or terminate input...@>=
//! if cur_chr>0 then force_eof:=true
//! else if name_in_progress then insert_relax
//! else start_input
//!
