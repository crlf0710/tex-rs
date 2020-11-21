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
