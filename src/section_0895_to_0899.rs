//! @ @<Declare subprocedures for |line_break|@>=
//! @t\4@>@<Declare the function called |reconstitute|@>
//! procedure hyphenate;
//! label common_ending,done,found,found1,found2,not_found,exit;
//! var @<Local variables for hyphenation@>@;
//! begin @<Find hyphen locations for the word in |hc|, or |return|@>;
//! @<If no hyphens were found, |return|@>;
//! @<Replace nodes |ha..hb| by a sequence of nodes that includes
//!   the discretionary hyphens@>;
//! exit:end;
//!
//! @ The first thing we need to do is find the node |ha| just before the
//! first letter.
//!
//! @<Skip to node |ha|, or |goto done1|...@>=
//! loop@+  begin if is_char_node(s) then
//!     begin c:=qo(character(s)); hf:=font(s);
//!     end
//!   else if type(s)=ligature_node then
//!     if lig_ptr(s)=null then goto continue
//!     else begin q:=lig_ptr(s); c:=qo(character(q)); hf:=font(q);
//!       end
//!   else if (type(s)=kern_node)and(subtype(s)=normal) then goto continue
//!   else if type(s)=whatsit_node then
//!     begin @<Advance \(p)past a whatsit node in the \(p)pre-hyphenation loop@>;
//!     goto continue;
//!     end
//!   else goto done1;
//!   if lc_code(c)<>0 then
//!     if (lc_code(c)=c)or(uc_hyph>0) then goto done2
//!     else goto done1;
//! continue: prev_s:=s; s:=link(prev_s);
//!   end;
//! done2: hyf_char:=hyphen_char[hf];
//! if hyf_char<0 then goto done1;
//! if hyf_char>255 then goto done1;
//! ha:=prev_s
//!
//! @ The word to be hyphenated is now moved to the |hu| and |hc| arrays.
//!
//! @<Skip to node |hb|, putting letters...@>=
//! hn:=0;
//! loop@+  begin if is_char_node(s) then
//!     begin if font(s)<>hf then goto done3;
//!     hyf_bchar:=character(s); c:=qo(hyf_bchar);
//!     if lc_code(c)=0 then goto done3;
//!     if hn=63 then goto done3;
//!     hb:=s; incr(hn); hu[hn]:=c; hc[hn]:=lc_code(c); hyf_bchar:=non_char;
//!     end
//!   else if type(s)=ligature_node then
//!     @<Move the characters of a ligature node to |hu| and |hc|;
//!       but |goto done3| if they are not all letters@>
//!   else if (type(s)=kern_node)and(subtype(s)=normal) then
//!     begin hb:=s;
//!     hyf_bchar:=font_bchar[hf];
//!     end
//!   else goto done3;
//!   s:=link(s);
//!   end;
//! done3:
//!
//! @ We let |j| be the index of the character being stored when a ligature node
//! is being expanded, since we do not want to advance |hn| until we are sure
//! that the entire ligature consists of letters. Note that it is possible
//! to get to |done3| with |hn=0| and |hb| not set to any value.
//!
//! @<Move the characters of a ligature node to |hu| and |hc|...@>=
//! begin if font(lig_char(s))<>hf then goto done3;
//! j:=hn; q:=lig_ptr(s);@+if q>null then hyf_bchar:=character(q);
//! while q>null do
//!   begin c:=qo(character(q));
//!   if lc_code(c)=0 then goto done3;
//!   if j=63 then goto done3;
//!   incr(j); hu[j]:=c; hc[j]:=lc_code(c);@/
//!   q:=link(q);
//!   end;
//! hb:=s; hn:=j;
//! if odd(subtype(s)) then hyf_bchar:=font_bchar[hf]@+else hyf_bchar:=non_char;
//! end
//!
//! @ @<Check that the nodes following |hb| permit hyphenation...@>=
//! if hn<l_hyf+r_hyf then goto done1; {|l_hyf| and |r_hyf| are |>=1|}
//! loop@+  begin if not(is_char_node(s)) then
//!     case type(s) of
//!     ligature_node: do_nothing;
//!     kern_node: if subtype(s)<>normal then goto done4;
//!     whatsit_node,glue_node,penalty_node,ins_node,adjust_node,mark_node:
//!       goto done4;
//!     othercases goto done1
//!     endcases;
//!   s:=link(s);
//!   end;
//! done4:
