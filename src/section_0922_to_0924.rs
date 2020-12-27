//! @ @<Local variables for hyph...@>=
//! @!z:trie_pointer; {an index into |trie|}
//! @!v:integer; {an index into |hyf_distance|, etc.}
//!
//! @ Assuming that these auxiliary tables have been set up properly, the
//! hyphenation algorithm is quite short. In the following code we set |hc[hn+2]|
//! to the impossible value 256, in order to guarantee that |hc[hn+3]| will
//! never be fetched.
//!
//! @<Find hyphen locations for the word in |hc|...@>=
//! for j:=0 to hn do hyf[j]:=0;
//! @<Look for the word |hc[1..hn]| in the exception table, and |goto found| (with
//!   |hyf| containing the hyphens) if an entry is found@>;
//! if trie_char(cur_lang+1)<>qi(cur_lang) then return; {no patterns for |cur_lang|}
//! hc[0]:=0; hc[hn+1]:=0; hc[hn+2]:=256; {insert delimiters}
//! for j:=0 to hn-r_hyf+1 do
//!   begin z:=trie_link(cur_lang+1)+hc[j]; l:=j;
//!   while hc[l]=qo(trie_char(z)) do
//!     begin if trie_op(z)<>min_quarterword then
//!       @<Store \(m)maximum values in the |hyf| table@>;
//!     incr(l); z:=trie_link(z)+hc[l];
//!     end;
//!   end;
//! found: for j:=0 to l_hyf-1 do hyf[j]:=0;
//! for j:=0 to r_hyf-1 do hyf[hn-j]:=0
//!
//! @ @<Store \(m)maximum values in the |hyf| table@>=
//! begin v:=trie_op(z);
//! repeat v:=v+op_start[cur_lang]; i:=l-hyf_distance[v];
//! if hyf_num[v]>hyf[i] then hyf[i]:=hyf_num[v];
//! v:=hyf_next[v];
//! until v=min_quarterword;
//! end
//!
