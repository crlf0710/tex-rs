//!
//! @ At this time |p| points to the mlist for the formula; |a| is either
//! |null| or it points to a box containing the equation number; and we are in
//! vertical mode (or internal vertical mode).
//!
//! @<Finish displayed math@>=
//! cur_mlist:=p; cur_style:=display_style; mlist_penalties:=false;
//! mlist_to_hlist; p:=link(temp_head);@/
//! adjust_tail:=adjust_head; b:=hpack(p,natural); p:=list_ptr(b);
//! t:=adjust_tail; adjust_tail:=null;@/
//! w:=width(b); z:=display_width; s:=display_indent;
//! if (a=null)or danger then
//!   begin e:=0; q:=0;
//!   end
//! else  begin e:=width(a); q:=e+math_quad(text_size);
//!   end;
//! if w+q>z then
//!   @<Squeeze the equation as much as possible; if there is an equation
//!     number that should go on a separate line by itself,
//!     set~|e:=0|@>;
//! @<Determine the displacement, |d|, of the left edge of the equation, with
//!   respect to the line size |z|, assuming that |l=false|@>;
//! @<Append the glue or equation number preceding the display@>;
//! @<Append the display and perhaps also the equation number@>;
//! @<Append the glue or equation number following the display@>;
//! resume_after_display
//!
//! @ @<Declare act...@>=
//! procedure resume_after_display;
//! begin if cur_group<>math_shift_group then confusion("display");
//! @:this can't happen display}{\quad display@>
//! unsave; prev_graf:=prev_graf+3;
//! push_nest; mode:=hmode; space_factor:=1000; set_cur_lang; clang:=cur_lang;
//! prev_graf:=(norm_min(left_hyphen_min)*@'100+norm_min(right_hyphen_min))
//!              *@'200000+cur_lang;
//! @<Scan an optional space@>;
//! if nest_ptr=1 then build_page;
//! end;
//!
//! @ The user can force the equation number to go on a separate line
//! by causing its width to be zero.
//!
//! @<Squeeze the equation as much as possible...@>=
//! begin if (e<>0)and((w-total_shrink[normal]+q<=z)or@|
//!    (total_shrink[fil]<>0)or(total_shrink[fill]<>0)or
//!    (total_shrink[filll]<>0)) then
//!   begin free_node(b,box_node_size);
//!   b:=hpack(p,z-q,exactly);
//!   end
//! else  begin e:=0;
//!   if w>z then
//!     begin free_node(b,box_node_size);
//!     b:=hpack(p,z,exactly);
//!     end;
//!   end;
//! w:=width(b);
//! end
//!
//! @ We try first to center the display without regard to the existence of
//! the equation number. If that would make it too close (where ``too close''
//! means that the space between display and equation number is less than the
//! width of the equation number), we either center it in the remaining space
//! or move it as far from the equation number as possible. The latter alternative
//! is taken only if the display begins with glue, since we assume that the
//! user put glue there to control the spacing precisely.
//!
//! @<Determine the displacement, |d|, of the left edge of the equation...@>=
//! d:=half(z-w);
//! if (e>0)and(d<2*e) then {too close}
//!   begin d:=half(z-w-e);
//!   if p<>null then if not is_char_node(p) then if type(p)=glue_node then d:=0;
//!   end
//!
//! @ If the equation number is set on a line by itself, either before or
//! after the formula, we append an infinite penalty so that no page break will
//! separate the display from its number; and we use the same size and
//! displacement for all three potential lines of the display, even though
//! `\.{\\parshape}' may specify them differently.
//!
//! @<Append the glue or equation number preceding the display@>=
//! tail_append(new_penalty(pre_display_penalty));@/
//! if (d+s<=pre_display_size)or l then {not enough clearance}
//!   begin g1:=above_display_skip_code; g2:=below_display_skip_code;
//!   end
//! else  begin g1:=above_display_short_skip_code;
//!   g2:=below_display_short_skip_code;
//!   end;
//! if l and(e=0) then {it follows that |type(a)=hlist_node|}
//!   begin shift_amount(a):=s; append_to_vlist(a);
//!   tail_append(new_penalty(inf_penalty));
//!   end
//! else tail_append(new_param_glue(g1))
//!
//! @ @<Append the display and perhaps also the equation number@>=
//! if e<>0 then
//!   begin r:=new_kern(z-w-e-d);
//!   if l then
//!     begin link(a):=r; link(r):=b; b:=a; d:=0;
//!     end
//!   else  begin link(b):=r; link(r):=a;
//!     end;
//!   b:=hpack(b,natural);
//!   end;
//! shift_amount(b):=s+d; append_to_vlist(b)
//!
//! @ @<Append the glue or equation number following the display@>=
//! if (a<>null)and(e=0)and not l then
//!   begin tail_append(new_penalty(inf_penalty));
//!   shift_amount(a):=s+z-width(a);
//!   append_to_vlist(a);
//!   g2:=0;
//!   end;
//! if t<>adjust_head then {migrating material comes after equation number}
//!   begin link(tail):=link(adjust_head); tail:=t;
//!   end;
//! tail_append(new_penalty(post_display_penalty));
//! if g2>0 then tail_append(new_param_glue(g2))
//!
//! @ When \.{\\halign} appears in a display, the alignment routines operate
//! essentially as they do in vertical mode. Then the following program is
//! activated, with |p| and |q| pointing to the beginning and end of the
//! resulting list, and with |aux_save| holding the |prev_depth| value.
//!
//! @<Finish an alignment in a display@>=
//! begin do_assignments;
//! if cur_cmd<>math_shift then @<Pontificate about improper alignment in display@>
//! else @<Check that another \.\$ follows@>;
//! pop_nest;
//! tail_append(new_penalty(pre_display_penalty));
//! tail_append(new_param_glue(above_display_skip_code));
//! link(tail):=p;
//! if p<>null then tail:=q;
//! tail_append(new_penalty(post_display_penalty));
//! tail_append(new_param_glue(below_display_skip_code));
//! prev_depth:=aux_save.sc; resume_after_display;
//! end
//!
//! @ @<Pontificate...@>=
//! begin print_err("Missing $$ inserted");
//! @.Missing {\$\$} inserted@>
//! help2("Displays can use special alignments (like \eqalignno)")@/
//!   ("only if nothing but the alignment itself is between $$'s.");
//! back_error;
//! end
//!
