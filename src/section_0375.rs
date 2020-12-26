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
