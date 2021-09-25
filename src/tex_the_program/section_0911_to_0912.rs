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
