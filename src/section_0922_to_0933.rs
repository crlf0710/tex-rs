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
//! @ The exception table that is built by \TeX's \.{\\hyphenation} primitive is
//! organized as an ordered hash table [cf.\ Amble and Knuth, {\sl The Computer
//! @^Amble, Ole@> @^Knuth, Donald Ervin@>
//! Journal\/ \bf17} (1974), 135--142] using linear probing. If $\alpha$ and
//! $\beta$ are words, we will say that $\alpha<\beta$ if $\vert\alpha\vert<
//! \vert\beta\vert$ or if $\vert\alpha\vert=\vert\beta\vert$ and
//! $\alpha$ is lexicographically smaller than $\beta$. (The notation $\vert
//! \alpha\vert$ stands for the length of $\alpha$.) The idea of ordered hashing
//! is to arrange the table so that a given word $\alpha$ can be sought by computing
//! a hash address $h=h(\alpha)$ and then looking in table positions |h|, |h-1|,
//! \dots, until encountering the first word $\L\alpha$. If this word is
//! different from $\alpha$, we can conclude that $\alpha$ is not in the table.
//!
//! The words in the table point to lists in |mem| that specify hyphen positions
//! in their |info| fields. The list for $c_1\ldots c_n$ contains the number |k| if
//! the word $c_1\ldots c_n$ has a discretionary hyphen between $c_k$ and
//! $c_{k+1}$.
//!
//! @<Types...@>=
//! @!hyph_pointer=0..hyph_size; {an index into the ordered hash table}
//!
//! @ @<Glob...@>=
//! @!hyph_word:array[hyph_pointer] of str_number; {exception words}
//! @!hyph_list:array[hyph_pointer] of pointer; {lists of hyphen positions}
//! @!hyph_count:hyph_pointer; {the number of words in the exception dictionary}
//!
//! @ @<Local variables for init...@>=
//! @!z:hyph_pointer; {runs through the exception dictionary}
//!
//! @ @<Set init...@>=
//! for z:=0 to hyph_size do
//!   begin hyph_word[z]:=0; hyph_list[z]:=null;
//!   end;
//! hyph_count:=0;
//!
//! @ The algorithm for exception lookup is quite simple, as soon as we have
//! a few more local variables to work with.
//!
//! @<Local variables for hyph...@>=
//! @!h:hyph_pointer; {an index into |hyph_word| and |hyph_list|}
//! @!k:str_number; {an index into |str_start|}
//! @!u:pool_pointer; {an index into |str_pool|}
//!
//! @ First we compute the hash code |h|, then we search until we either
//! find the word or we don't. Words from different languages are kept
//! separate by appending the language code to the string.
//!
//! @<Look for the word |hc[1...@>=
//! h:=hc[1]; incr(hn); hc[hn]:=cur_lang;
//! for j:=2 to hn do h:=(h+h+hc[j]) mod hyph_size;
//! loop@+  begin @<If the string |hyph_word[h]| is less than \(hc)|hc[1..hn]|,
//!     |goto not_found|; but if the two strings are equal,
//!     set |hyf| to the hyphen positions and |goto found|@>;
//!   if h>0 then decr(h)@+else h:=hyph_size;
//!   end;
//! not_found: decr(hn)
//!
//! @ @<If the string |hyph_word[h]| is less than \(hc)...@>=
//! k:=hyph_word[h]; if k=0 then goto not_found;
//! if length(k)<hn then goto not_found;
//! if length(k)=hn then
//!   begin j:=1; u:=str_start[k];
//!   repeat if so(str_pool[u])<hc[j] then goto not_found;
//!   if so(str_pool[u])>hc[j] then goto done;
//!   incr(j); incr(u);
//!   until j>hn;
//!   @<Insert hyphens as specified in |hyph_list[h]|@>;
//!   decr(hn); goto found;
//!   end;
//! done:
//!
//! @ @<Insert hyphens as specified...@>=
//! s:=hyph_list[h];
//! while s<>null do
//!   begin hyf[info(s)]:=1; s:=link(s);
//!   end
//!
//! @ @<Search |hyph_list| for pointers to |p|@>=
//! for q:=0 to hyph_size do
//!   begin if hyph_list[q]=p then
//!     begin print_nl("HYPH("); print_int(q); print_char(")");
//!     end;
//!   end
//!
