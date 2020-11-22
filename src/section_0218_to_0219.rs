//! @ Here is a procedure that displays what \TeX\ is working on, at all levels.
//!
//! @p procedure@?print_totals; forward;@t\2@>
//! procedure show_activities;
//! var p:0..nest_size; {index into |nest|}
//! @!m:-mmode..mmode; {mode}
//! @!a:memory_word; {auxiliary}
//! @!q,@!r:pointer; {for showing the current page}
//! @!t:integer; {ditto}
//! begin nest[nest_ptr]:=cur_list; {put the top level into the array}
//! print_nl(""); print_ln;
//! for p:=nest_ptr downto 0 do
//!   begin m:=nest[p].mode_field; a:=nest[p].aux_field;
//!   print_nl("### "); print_mode(m);
//!   print(" entered at line "); print_int(abs(nest[p].ml_field));
//!   if m=hmode then if nest[p].pg_field <> @'40600000 then
//!     begin print(" (language"); print_int(nest[p].pg_field mod @'200000);
//!     print(":hyphenmin"); print_int(nest[p].pg_field div @'20000000);
//!     print_char(","); print_int((nest[p].pg_field div @'200000) mod @'100);
//!     print_char(")");
//!     end;
//!   if nest[p].ml_field<0 then print(" (\output routine)");
//!   if p=0 then
//!     begin @<Show the status of the current page@>;
//!     if link(contrib_head)<>null then
//!       print_nl("### recent contributions:");
//!     end;
//!   show_box(link(nest[p].head_field));
//!   @<Show the auxiliary field, |a|@>;
//!   end;
//! end;
//!
//! @ @<Show the auxiliary...@>=
//! case abs(m) div (max_command+1) of
//! 0: begin print_nl("prevdepth ");
//!   if a.sc<=ignore_depth then print("ignored")
//!   else print_scaled(a.sc);
//!   if nest[p].pg_field<>0 then
//!     begin print(", prevgraf ");
//!     print_int(nest[p].pg_field); print(" line");
//!     if nest[p].pg_field<>1 then print_char("s");
//!     end;
//!   end;
//! 1: begin print_nl("spacefactor "); print_int(a.hh.lh);
//!   if m>0 then@+ if a.hh.rh>0 then
//!     begin print(", current language "); print_int(a.hh.rh);@+
//!     end;
//!   end;
//! 2: if a.int<>null then
//!   begin print("this will be denominator of:"); show_box(a.int);@+
//!   end;
//! end {there are no other cases}
//!
