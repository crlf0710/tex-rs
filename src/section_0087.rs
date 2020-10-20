//! @ When the following code is executed, |buffer[(first+1)..(last-1)]| may
//! contain the material inserted by the user; otherwise another prompt will
//! be given. In order to understand this part of the program fully, you need
//! to be familiar with \TeX's input stacks.
//!
//! @<Introduce new material...@>=
//! begin begin_file_reading; {enter a new syntactic level for terminal input}
//! {now |state=mid_line|, so an initial blank space will count as a blank}
//! if last>first+1 then
//!   begin loc:=first+1; buffer[first]:=" ";
//!   end
//! else  begin prompt_input("insert>"); loc:=first;
//! @.insert>@>
//!   end;
//! first:=last;
//! cur_input.limit_field:=last-1; {no |end_line_char| ends this line}
//! return;
//! end
//!
