//! @ @<Insert token |p| into \TeX's input@>=
//! begin t:=cur_tok; cur_tok:=p; back_input; cur_tok:=t;
//! end
//!
//! @ The |back_error| routine is used when we want to replace an offending token
//! just before issuing an error message. This routine, like |back_input|,
//! requires that |cur_tok| has been set. We disable interrupts during the
//! call of |back_input| so that the help message won't be lost.
//!
//! @p procedure back_error; {back up one token and call |error|}
//! begin OK_to_interrupt:=false; back_input; OK_to_interrupt:=true; error;
//! end;
//! @#
//! procedure ins_error; {back up one inserted token and call |error|}
//! begin OK_to_interrupt:=false; back_input; token_type:=inserted;
//! OK_to_interrupt:=true; error;
//! end;
//!
