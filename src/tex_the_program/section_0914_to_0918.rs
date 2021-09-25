//! @ In this repeat loop we will insert another discretionary if |hyf[j-1]| is
//! odd, when both branches of the previous discretionary end at position |j-1|.
//! Strictly speaking, we aren't justified in doing this, because we don't know
//! that a hyphen after |j-1| is truly independent of those branches. But in almost
//! all applications we would rather not lose a potentially valuable hyphenation
//! point. (Consider the word `difficult', where the letter `c' is in position |j|.)
//!
//! @d advance_major_tail==begin major_tail:=link(major_tail); incr(r_count);
//!     end
//!
//! @<Create and append a discretionary node as an alternative...@>=
//! repeat r:=get_node(small_node_size);
//! link(r):=link(hold_head); type(r):=disc_node;
//! major_tail:=r; r_count:=0;
//! while link(major_tail)>null do advance_major_tail;
//! i:=hyphen_passed; hyf[i]:=0;
//! @<Put the \(c)characters |hu[l..i]| and a hyphen into |pre_break(r)|@>;
//! @<Put the \(c)characters |hu[i+1..@,]| into |post_break(r)|, appending to this
//!   list and to |major_tail| until synchronization has been achieved@>;
//! @<Move pointer |s| to the end of the current list, and set |replace_count(r)|
//!   appropriately@>;
//! hyphen_passed:=j-1; link(hold_head):=null;
//! until not odd(hyf[j-1])
//!
//! @ The new hyphen might combine with the previous character via ligature
//! or kern. At this point we have |l-1<=i<j| and |i<hn|.
//!
//! @<Put the \(c)characters |hu[l..i]| and a hyphen into |pre_break(r)|@>=
//! minor_tail:=null; pre_break(r):=null; hyf_node:=new_character(hf,hyf_char);
//! if hyf_node<>null then
//!   begin incr(i); c:=hu[i]; hu[i]:=hyf_char; free_avail(hyf_node);
//!   end;
//! while l<=i do
//!   begin l:=reconstitute(l,i,font_bchar[hf],non_char)+1;
//!   if link(hold_head)>null then
//!     begin if minor_tail=null then pre_break(r):=link(hold_head)
//!     else link(minor_tail):=link(hold_head);
//!     minor_tail:=link(hold_head);
//!     while link(minor_tail)>null do minor_tail:=link(minor_tail);
//!     end;
//!   end;
//! if hyf_node<>null then
//!   begin hu[i]:=c; {restore the character in the hyphen position}
//!   l:=i; decr(i);
//!   end
//!
//! @ The synchronization algorithm begins with |l=i+1<=j|.
//!
//! @<Put the \(c)characters |hu[i+1..@,]| into |post_break(r)|...@>=
//! minor_tail:=null; post_break(r):=null; c_loc:=0;
//! if bchar_label[hf]<>non_address then {put left boundary at beginning of new line}
//!   begin decr(l); c:=hu[l]; c_loc:=l; hu[l]:=256;
//!   end;
//! while l<j do
//!   begin repeat l:=reconstitute(l,hn,bchar,non_char)+1;
//!   if c_loc>0 then
//!     begin hu[c_loc]:=c; c_loc:=0;
//!     end;
//!   if link(hold_head)>null then
//!     begin if minor_tail=null then post_break(r):=link(hold_head)
//!     else link(minor_tail):=link(hold_head);
//!     minor_tail:=link(hold_head);
//!     while link(minor_tail)>null do minor_tail:=link(minor_tail);
//!     end;
//!   until l>=j;
//!   while l>j do
//!     @<Append characters of |hu[j..@,]| to |major_tail|, advancing~|j|@>;
//!   end
//!
//! @ @<Append characters of |hu[j..@,]|...@>=
//! begin j:=reconstitute(j,hn,bchar,non_char)+1;
//! link(major_tail):=link(hold_head);
//! while link(major_tail)>null do advance_major_tail;
//! end
//!
//! @ Ligature insertion can cause a word to grow exponentially in size. Therefore
//! we must test the size of |r_count| here, even though the hyphenated text
//! was at most 63 characters long.
//!
//! @<Move pointer |s| to the end of the current list...@>=
//! if r_count>127 then {we have to forget the discretionary hyphen}
//!   begin link(s):=link(r); link(r):=null; flush_node_list(r);
//!   end
//! else begin link(s):=r; replace_count(r):=r_count;
//!   end;
//! s:=major_tail
//!
