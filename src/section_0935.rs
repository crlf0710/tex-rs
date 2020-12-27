//! @ @<Enter as many...@>=
//! n:=0; p:=null;
//! loop@+  begin get_x_token;
//!   reswitch: case cur_cmd of
//!   letter,other_char,char_given:@<Append a new letter or hyphen@>;
//!   char_num: begin scan_char_num; cur_chr:=cur_val; cur_cmd:=char_given;
//!     goto reswitch;
//!     end;
//!   spacer,right_brace: begin if n>1 then @<Enter a hyphenation exception@>;
//!     if cur_cmd=right_brace then return;
//!     n:=0; p:=null;
//!     end;
//!   othercases @<Give improper \.{\\hyphenation} error@>
//!   endcases;
//!   end
//!
