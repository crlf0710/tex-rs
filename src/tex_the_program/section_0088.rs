//! @ We allow deletion of up to 99 tokens at a time.
//!
//! @<Delete \(c)|c-"0"| tokens...@>=
//! begin s1:=cur_tok; s2:=cur_cmd; s3:=cur_chr; s4:=align_state;
//! align_state:=1000000; OK_to_interrupt:=false;
//! if (last>first+1) and (buffer[first+1]>="0")and(buffer[first+1]<="9") then
//!   c:=c*10+buffer[first+1]-"0"*11
//! else c:=c-"0";
//! while c>0 do
//!   begin get_token; {one-level recursive call of |error| is possible}
//!   decr(c);
//!   end;
//! cur_tok:=s1; cur_cmd:=s2; cur_chr:=s3; align_state:=s4; OK_to_interrupt:=true;
//! help2("I have just deleted some text, as you asked.")@/
//! ("You can now delete more, or insert, or whatever.");
//! show_context; goto continue;
//! end
//!
