//!
//! @* \[41] Post-hyphenation.
//! If a hyphen may be inserted between |hc[j]| and |hc[j+1]|, the hyphenation
//! procedure will set |hyf[j]| to some small odd number. But before we look
//! at \TeX's hyphenation procedure, which is independent of the rest of the
//! line-breaking algorithm, let us consider what we will do with the hyphens
//! it finds, since it is better to work on this part of the program before
//! forgetting what |ha| and |hb|, etc., are all about.
//!
//! @<Glob...@>=
//! @!hyf:array [0..64] of 0..9; {odd values indicate discretionary hyphens}
//! @!init_list:pointer; {list of punctuation characters preceding the word}
//! @!init_lig:boolean; {does |init_list| represent a ligature?}
//! @!init_lft:boolean; {if so, did the ligature involve a left boundary?}
//!
//! @ @<Local variables for hyphenation@>=
//! @!i,@!j,@!l:0..65; {indices into |hc| or |hu|}
//! @!q,@!r,@!s:pointer; {temporary registers for list manipulation}
//! @!bchar:halfword; {right boundary character of hyphenated word, or |non_char|}
//!
//! @ \TeX\ will never insert a hyphen that has fewer than
//! \.{\\lefthyphenmin} letters before it or fewer than
//! \.{\\righthyphenmin} after it; hence, a short word has
//! comparatively little chance of being hyphenated. If no hyphens have
//! been found, we can save time by not having to make any changes to the
//! paragraph.
//!
//! @<If no hyphens were found, |return|@>=
//! for j:=l_hyf to hn-r_hyf do if odd(hyf[j]) then goto found1;
//! return;
//! found1:
//!
//! @ If hyphens are in fact going to be inserted, \TeX\ first deletes the
//! subsequence of nodes between |ha| and~|hb|. An attempt is made to
//! preserve the effect that implicit boundary characters and punctuation marks
//! had on ligatures inside the hyphenated word, by storing a left boundary or
//! preceding character in |hu[0]| and by storing a possible right boundary
//! in |bchar|. We set |j:=0| if |hu[0]| is to be part of the reconstruction;
//! otherwise |j:=1|.
//! The variable |s| will point to the tail of the current hlist, and
//! |q| will point to the node following |hb|, so that
//! things can be hooked up after we reconstitute the hyphenated word.
//!
//! @<Replace nodes |ha..hb| by a sequence of nodes...@>=
//! q:=link(hb); link(hb):=null; r:=link(ha); link(ha):=null; bchar:=hyf_bchar;
//! if is_char_node(ha) then
//!   if font(ha)<>hf then goto found2
//!   else begin init_list:=ha; init_lig:=false; hu[0]:=qo(character(ha));
//!     end
//! else if type(ha)=ligature_node then
//!   if font(lig_char(ha))<>hf then goto found2
//!   else begin init_list:=lig_ptr(ha); init_lig:=true; init_lft:=(subtype(ha)>1);
//!     hu[0]:=qo(character(lig_char(ha)));
//!     if init_list=null then if init_lft then
//!       begin hu[0]:=256; init_lig:=false;
//!       end; {in this case a ligature will be reconstructed from scratch}
//!     free_node(ha,small_node_size);
//!     end
//! else begin {no punctuation found; look for left boundary}
//!   if not is_char_node(r) then if type(r)=ligature_node then
//!    if subtype(r)>1 then goto found2;
//!   j:=1; s:=ha; init_list:=null; goto common_ending;
//!   end;
//! s:=cur_p; {we have |cur_p<>ha| because |type(cur_p)=glue_node|}
//! while link(s)<>ha do s:=link(s);
//! j:=0; goto common_ending;
//! found2: s:=ha; j:=0; hu[0]:=256; init_lig:=false; init_list:=null;
//! common_ending: flush_node_list(r);
//! @<Reconstitute nodes for the hyphenated word, inserting discretionary hyphens@>;
//! flush_list(init_list)
//!
//! @ We must now face the fact that the battle is not over, even though the
//! {\def\!{\kern-1pt}%
//! hyphens have been found: The process of reconstituting a word can be nontrivial
//! because ligatures might change when a hyphen is present. {\sl The \TeX book\/}
//! discusses the difficulties of the word ``difficult'', and
//! the discretionary material surrounding a
//! hyphen can be considerably more complex than that. Suppose
//! \.{abcdef} is a word in a font for which the only ligatures are \.{b\!c},
//! \.{c\!d}, \.{d\!e}, and \.{e\!f}. If this word permits hyphenation
//! between \.b and \.c, the two patterns with and without hyphenation are
//! $\.a\,\.b\,\.-\,\.{c\!d}\,\.{e\!f}$ and $\.a\,\.{b\!c}\,\.{d\!e}\,\.f$.
//! Thus the insertion of a hyphen might cause effects to ripple arbitrarily
//! far into the rest of the word. A further complication arises if additional
//! hyphens appear together with such rippling, e.g., if the word in the
//! example just given could also be hyphenated between \.c and \.d; \TeX\
//! avoids this by simply ignoring the additional hyphens in such weird cases.}
//!
//! Still further complications arise in the presence of ligatures that do not
//! delete the original characters. When punctuation precedes the word being
//! hyphenated, \TeX's method is not perfect under all possible scenarios,
//! because punctuation marks and letters can propagate information back and forth.
//! For example, suppose the original pre-hyphenation pair
//! \.{*a} changes to \.{*y} via a \.{\?=:} ligature, which changes to \.{xy}
//! via a \.{=:\?} ligature; if $p_{a-1}=\.x$ and $p_a=\.y$, the reconstitution
//! procedure isn't smart enough to obtain \.{xy} again. In such cases the
//! font designer should include a ligature that goes from \.{xa} to \.{xy}.
//!
//! @ The processing is facilitated by a subroutine called |reconstitute|. Given
//! a string of characters $x_j\ldots x_n$, there is a smallest index $m\ge j$
//! such that the ``translation'' of $x_j\ldots x_n$ by ligatures and kerning
//! has the form $y_1\ldots y_t$ followed by the translation of $x_{m+1}\ldots x_n$,
//! where $y_1\ldots y_t$ is some nonempty sequence of character, ligature, and
//! kern nodes. We call $x_j\ldots x_m$ a ``cut prefix'' of $x_j\ldots x_n$.
//! For example, if $x_1x_2x_3=\.{fly}$, and if the font contains `fl' as a
//! ligature and a kern between `fl' and `y', then $m=2$, $t=2$, and $y_1$ will
//! be a ligature node for `fl' followed by an appropriate kern node~$y_2$.
//! In the most common case, $x_j$~forms no ligature with $x_{j+1}$ and we
//! simply have $m=j$, $y_1=x_j$. If $m<n$ we can repeat the procedure on
//! $x_{m+1}\ldots x_n$ until the entire translation has been found.
//!
//! The |reconstitute| function returns the integer $m$ and puts the nodes
//! $y_1\ldots y_t$ into a linked list starting at |link(hold_head)|,
//! getting the input $x_j\ldots x_n$ from the |hu| array. If $x_j=256$,
//! we consider $x_j$ to be an implicit left boundary character; in this
//! case |j| must be strictly less than~|n|. There is a
//! parameter |bchar|, which is either 256 or an implicit right boundary character
//! assumed to be present just following~$x_n$. (The value |hu[n+1]| is never
//! explicitly examined, but the algorithm imagines that |bchar| is there.)
//!
//! If there exists an index |k| in the range $j\le k\le m$ such that |hyf[k]|
//! is odd and such that the result of |reconstitute| would have been different
//! if $x_{k+1}$ had been |hchar|, then |reconstitute| sets |hyphen_passed|
//! to the smallest such~|k|. Otherwise it sets |hyphen_passed| to zero.
//!
//! A special convention is used in the case |j=0|: Then we assume that the
//! translation of |hu[0]| appears in a special list of charnodes starting at
//! |init_list|; moreover, if |init_lig| is |true|, then |hu[0]| will be
//! a ligature character, involving a left boundary if |init_lft| is |true|.
//! This facility is provided for cases when a hyphenated
//! word is preceded by punctuation (like single or double quotes) that might
//! affect the translation of the beginning of the word.
//!
//! @<Glob...@>=
//! @!hyphen_passed:small_number; {first hyphen in a ligature, if any}
//!
//! @ @<Declare the function called |reconstitute|@>=
//! function reconstitute(@!j,@!n:small_number;@!bchar,@!hchar:halfword):
//!   small_number;
//! label continue,done;
//! var @!p:pointer; {temporary register for list manipulation}
//! @!t:pointer; {a node being appended to}
//! @!q:four_quarters; {character information or a lig/kern instruction}
//! @!cur_rh:halfword; {hyphen character for ligature testing}
//! @!test_char:halfword; {hyphen or other character for ligature testing}
//! @!w:scaled; {amount of kerning}
//! @!k:font_index; {position of current lig/kern instruction}
//! begin hyphen_passed:=0; t:=hold_head; w:=0; link(hold_head):=null;
//!  {at this point |ligature_present=lft_hit=rt_hit=false|}
//! @<Set up data structures with the cursor following position |j|@>;
//! continue:@<If there's a ligature or kern at the cursor position, update the data
//!   structures, possibly advancing~|j|; continue until the cursor moves@>;
//! @<Append a ligature and/or kern to the translation;
//!   |goto continue| if the stack of inserted ligatures is nonempty@>;
//! reconstitute:=j;
//! end;
//!
//! @ The reconstitution procedure shares many of the global data structures
//! by which \TeX\ has processed the words before they were hyphenated.
//! There is an implied ``cursor'' between characters |cur_l| and |cur_r|;
//! these characters will be tested for possible ligature activity. If
//! |ligature_present| then |cur_l| is a ligature character formed from the
//! original characters following |cur_q| in the current translation list.
//! There is a ``ligature stack'' between the cursor and character |j+1|,
//! consisting of pseudo-ligature nodes linked together by their |link| fields.
//! This stack is normally empty unless a ligature command has created a new
//! character that will need to be processed later. A pseudo-ligature is
//! a special node having a |character| field that represents a potential
//! ligature and a |lig_ptr| field that points to a |char_node| or is |null|.
//! We have
//! $$|cur_r|=\cases{|character(lig_stack)|,&if |lig_stack>null|;\cr
//!   |qi(hu[j+1])|,&if |lig_stack=null| and |j<n|;\cr
//!   bchar,&if |lig_stack=null| and |j=n|.\cr}$$
//!
//! @<Glob...@>=
//! @!cur_l,@!cur_r:halfword; {characters before and after the cursor}
//! @!cur_q:pointer; {where a ligature should be detached}
//! @!lig_stack:pointer; {unfinished business to the right of the cursor}
//! @!ligature_present:boolean; {should a ligature node be made for |cur_l|?}
//! @!lft_hit,@!rt_hit:boolean; {did we hit a ligature with a boundary character?}
//!
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
