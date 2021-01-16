//! @ The parameter to |init_span| is a pointer to the alignrecord where the
//! next column or group of columns will begin. A new semantic level is
//! entered, so that the columns will generate a list for subsequent packaging.
//!
//! @<Declare the procedure called |init_span|@>=
//! procedure init_span(@!p:pointer);
//! begin push_nest;
//! if mode=-hmode then space_factor:=1000
//! else  begin prev_depth:=ignore_depth; normal_paragraph;
//!   end;
//! cur_span:=p;
//! end;
//!
