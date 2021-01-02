//! @ Inside an \.{\\output} routine, a user may wish to look at the page totals
//! that were present at the moment when output was triggered.
//!
//! @d max_dimen==@'7777777777 {$2^{30}-1$}
//!
//! @<Fetch something on the |page_so_far|@>=
//! begin if (page_contents=empty) and (not output_active) then
//!   if m=0 then cur_val:=max_dimen@+else cur_val:=0
//! else cur_val:=page_so_far[m];
//! cur_val_level:=dimen_val;
//! end
//!
//! @ @<Fetch the |prev_graf|@>=
//! if mode=0 then scanned_result(0)(int_val) {|prev_graf=0| within \.{\\write}}
//! else begin nest[nest_ptr]:=cur_list; p:=nest_ptr;
//!   while abs(nest[p].mode_field)<>vmode do decr(p);
//!   scanned_result(nest[p].pg_field)(int_val);
//!   end
//!
//! @ @<Fetch the |par_shape| size@>=
//! begin if par_shape_ptr=null then cur_val:=0
//! else cur_val:=info(par_shape_ptr);
//! cur_val_level:=int_val;
//! end
//!
//! @ Here is where \.{\\lastpenalty}, \.{\\lastkern}, and \.{\\lastskip} are
//! implemented. The reference count for \.{\\lastskip} will be updated later.
//!
//! We also handle \.{\\inputlineno} and \.{\\badness} here, because they are
//! legal in similar contexts.
//!
//! @<Fetch an item in the current node...@>=
//! if cur_chr>glue_val then
//!   begin if cur_chr=input_line_no_code then cur_val:=line
//!   else cur_val:=last_badness; {|cur_chr=badness_code|}
//!   cur_val_level:=int_val;
//!   end
//! else begin if cur_chr=glue_val then cur_val:=zero_glue@+else cur_val:=0;
//!   cur_val_level:=cur_chr;
//!   if not is_char_node(tail)and(mode<>0) then
//!     case cur_chr of
//!     int_val: if type(tail)=penalty_node then cur_val:=penalty(tail);
//!     dimen_val: if type(tail)=kern_node then cur_val:=width(tail);
//!     glue_val: if type(tail)=glue_node then
//!       begin cur_val:=glue_ptr(tail);
//!       if subtype(tail)=mu_glue then cur_val_level:=mu_val;
//!       end;
//!     end {there are no other cases}
//!   else if (mode=vmode)and(tail=head) then
//!     case cur_chr of
//!     int_val: cur_val:=last_penalty;
//!     dimen_val: cur_val:=last_kern;
//!     glue_val: if last_glue<>max_halfword then cur_val:=last_glue;
//!     end; {there are no other cases}
//!   end
//!
//! @ @<Fetch a font dimension@>=
//! begin find_font_dimen(false); font_info[fmem_ptr].sc:=0;
//! scanned_result(font_info[cur_val].sc)(dimen_val);
//! end
//!

