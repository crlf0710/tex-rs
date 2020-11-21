//! @ @d append_charnode_to_t(#)== begin link(t):=get_avail; t:=link(t);
//!     font(t):=hf; character(t):=#;
//!     end
//! @d set_cur_r==begin if j<n then cur_r:=qi(hu[j+1])@+else cur_r:=bchar;
//!     if odd(hyf[j]) then cur_rh:=hchar@+else cur_rh:=non_char;
//!     end
//!
//! @<Set up data structures with the cursor following position |j|@>=
//! cur_l:=qi(hu[j]); cur_q:=t;
//! if j=0 then
//!   begin ligature_present:=init_lig; p:=init_list;
//!   if ligature_present then lft_hit:=init_lft;
//!   while p>null do
//!     begin append_charnode_to_t(character(p)); p:=link(p);
//!     end;
//!   end
//! else if cur_l<non_char then append_charnode_to_t(cur_l);
//! lig_stack:=null; set_cur_r
//!
//! @ We may want to look at the lig/kern program twice, once for a hyphen
//! and once for a normal letter. (The hyphen might appear after the letter
//! in the program, so we'd better not try to look for both at once.)
//!
//! @<If there's a ligature or kern at the cursor position, update...@>=
//! if cur_l=non_char then
//!   begin k:=bchar_label[hf];
//!   if k=non_address then goto done@+else q:=font_info[k].qqqq;
//!   end
//! else begin q:=char_info(hf)(cur_l);
//!   if char_tag(q)<>lig_tag then goto done;
//!   k:=lig_kern_start(hf)(q); q:=font_info[k].qqqq;
//!   if skip_byte(q)>stop_flag then
//!     begin k:=lig_kern_restart(hf)(q); q:=font_info[k].qqqq;
//!     end;
//!   end; {now |k| is the starting address of the lig/kern program}
//! if cur_rh<non_char then test_char:=cur_rh@+else test_char:=cur_r;
//! loop@+begin if next_char(q)=test_char then if skip_byte(q)<=stop_flag then
//!     if cur_rh<non_char then
//!       begin hyphen_passed:=j; hchar:=non_char; cur_rh:=non_char;
//!       goto continue;
//!       end
//!     else begin if hchar<non_char then if odd(hyf[j]) then
//!         begin hyphen_passed:=j; hchar:=non_char;
//!         end;
//!       if op_byte(q)<kern_flag then
//!       @<Carry out a ligature replacement, updating the cursor structure
//!         and possibly advancing~|j|; |goto continue| if the cursor doesn't
//!         advance, otherwise |goto done|@>;
//!       w:=char_kern(hf)(q); goto done; {this kern will be inserted below}
//!      end;
//!   if skip_byte(q)>=stop_flag then
//!     if cur_rh=non_char then goto done
//!     else begin cur_rh:=non_char; goto continue;
//!       end;
//!   k:=k+qo(skip_byte(q))+1; q:=font_info[k].qqqq;
//!   end;
//! done:
//!
//! @ @d wrap_lig(#)==if ligature_present then
//!     begin p:=new_ligature(hf,cur_l,link(cur_q));
//!     if lft_hit then
//!       begin subtype(p):=2; lft_hit:=false;
//!       end;
//!     if # then if lig_stack=null then
//!       begin incr(subtype(p)); rt_hit:=false;
//!       end;
//!     link(cur_q):=p; t:=p; ligature_present:=false;
//!     end
//! @d pop_lig_stack==begin if lig_ptr(lig_stack)>null then
//!     begin link(t):=lig_ptr(lig_stack); {this is a charnode for |hu[j+1]|}
//!     t:=link(t); incr(j);
//!     end;
//!   p:=lig_stack; lig_stack:=link(p); free_node(p,small_node_size);
//!   if lig_stack=null then set_cur_r@+else cur_r:=character(lig_stack);
//!   end {if |lig_stack| isn't |null| we have |cur_rh=non_char|}
//!
//! @<Append a ligature and/or kern to the translation...@>=
//! wrap_lig(rt_hit);
//! if w<>0 then
//!   begin link(t):=new_kern(w); t:=link(t); w:=0;
//!   end;
//! if lig_stack>null then
//!   begin cur_q:=t; cur_l:=character(lig_stack); ligature_present:=true;
//!   pop_lig_stack; goto continue;
//!   end
//!
//! @ @<Carry out a ligature replacement, updating the cursor structure...@>=
//! begin if cur_l=non_char then lft_hit:=true;
//! if j=n then if lig_stack=null then rt_hit:=true;
//! check_interrupt; {allow a way out in case there's an infinite ligature loop}
//! case op_byte(q) of
//! qi(1),qi(5):begin cur_l:=rem_byte(q); {\.{=:\?}, \.{=:\?>}}
//!   ligature_present:=true;
//!   end;
//! qi(2),qi(6):begin cur_r:=rem_byte(q); {\.{\?=:}, \.{\?=:>}}
//!   if lig_stack>null then character(lig_stack):=cur_r
//!   else begin lig_stack:=new_lig_item(cur_r);
//!     if j=n then bchar:=non_char
//!     else begin p:=get_avail; lig_ptr(lig_stack):=p;
//!       character(p):=qi(hu[j+1]); font(p):=hf;
//!       end;
//!     end;
//!   end;
//! qi(3):begin cur_r:=rem_byte(q); {\.{\?=:\?}}
//!   p:=lig_stack; lig_stack:=new_lig_item(cur_r); link(lig_stack):=p;
//!   end;
//! qi(7),qi(11):begin wrap_lig(false); {\.{\?=:\?>}, \.{\?=:\?>>}}
//!   cur_q:=t; cur_l:=rem_byte(q); ligature_present:=true;
//!   end;
//! othercases begin cur_l:=rem_byte(q); ligature_present:=true; {\.{=:}}
//!   if lig_stack>null then pop_lig_stack
//!   else if j=n then goto done
//!   else begin append_charnode_to_t(cur_r); incr(j); set_cur_r;
//!     end;
//!   end
//! endcases;
//! if op_byte(q)>qi(4) then if op_byte(q)<>qi(7) then goto done;
//! goto continue;
//! end
//!
//! @ Okay, we're ready to insert the potential hyphenations that were found.
//! When the following program is executed, we want to append the word
//! |hu[1..hn]| after node |ha|, and node |q| should be appended to the result.
//! During this process, the variable |i| will be a temporary
//! index into |hu|; the variable |j| will be an index to our current position
//! in |hu|; the variable |l| will be the counterpart of |j|, in a discretionary
//! branch; the variable |r| will point to new nodes being created; and
//! we need a few new local variables:
//!
//! @<Local variables for hyph...@>=
//! @!major_tail,@!minor_tail:pointer; {the end of lists in the main and
//!   discretionary branches being reconstructed}
//! @!c:ASCII_code; {character temporarily replaced by a hyphen}
//! @!c_loc:0..63; {where that character came from}
//! @!r_count:integer; {replacement count for discretionary}
//! @!hyf_node:pointer; {the hyphen, if it exists}
//!
//! @ When the following code is performed, |hyf[0]| and |hyf[hn]| will be zero.
//!
//! @<Reconstitute nodes for the hyphenated word...@>=
//! repeat l:=j; j:=reconstitute(j,hn,bchar,qi(hyf_char))+1;
//! if hyphen_passed=0 then
//!   begin link(s):=link(hold_head);
//!   while link(s)>null do s:=link(s);
//!   if odd(hyf[j-1]) then
//!     begin l:=j; hyphen_passed:=j-1; link(hold_head):=null;
//!     end;
//!   end;
//! if hyphen_passed>0 then
//!   @<Create and append a discretionary node as an alternative to the
//!     unhyphenated word, and continue to develop both branches until they
//!     become equivalent@>;
//! until j>hn;
//! link(s):=q
//!
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
