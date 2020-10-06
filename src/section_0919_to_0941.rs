//! @* \[42] Hyphenation.
//! When a word |hc[1..hn]| has been set up to contain a candidate for hyphenation,
//! \TeX\ first looks to see if it is in the user's exception dictionary. If not,
//! hyphens are inserted based on patterns that appear within the given word,
//! using an algorithm due to Frank~M. Liang.
//! @^Liang, Franklin Mark@>
//!
//! Let's consider Liang's method first, since it is much more interesting than the
//! exception-lookup routine.  The algorithm begins by setting |hyf[j]| to zero
//! for all |j|, and invalid characters are inserted into |hc[0]|
//! and |hc[hn+1]| to serve as delimiters. Then a reasonably fast method is
//! used to see which of a given set of patterns occurs in the word
//! |hc[0..(hn+1)]|. Each pattern $p_1\ldots p_k$ of length |k| has an associated
//! sequence of |k+1| numbers $n_0\ldots n_k$; and if the pattern occurs in
//! |hc[(j+1)..(j+k)]|, \TeX\ will set |hyf[j+i]:=@tmax@>(hyf[j+i],@t$n_i$@>)| for
//! |0<=i<=k|. After this has been done for each pattern that occurs, a
//! discretionary hyphen will be inserted between |hc[j]| and |hc[j+1]| when
//! |hyf[j]| is odd, as we have already seen.
//!
//! The set of patterns $p_1\ldots p_k$ and associated numbers $n_0\ldots n_k$
//! depends, of course, on the language whose words are being hyphenated, and
//! on the degree of hyphenation that is desired. A method for finding
//! appropriate |p|'s and |n|'s, from a given dictionary of words and acceptable
//! hyphenations, is discussed in Liang's Ph.D. thesis (Stanford University,
//! 1983); \TeX\ simply starts with the patterns and works from there.
//!
//! @ The patterns are stored in a compact table that is also efficient for
//! retrieval, using a variant of ``trie memory'' [cf.\ {\sl The Art of
//! Computer Programming \bf3} (1973), 481--505]. We can find each pattern
//! $p_1\ldots p_k$ by letting $z_0$ be one greater than the relevant language
//! index and then, for |1<=i<=k|,
//! setting |@t$z_i$@>:=trie_link@t$(z_{i-1})+p_i$@>|; the pattern will be
//! identified by the number $z_k$. Since all the pattern information is
//! packed together into a single |trie_link| array, it is necessary to
//! prevent confusion between the data from inequivalent patterns, so another
//! table is provided such that |trie_char@t$(z_i)=p_i$@>| for all |i|. There
//! is also a table |trie_op|$(z_k)$ to identify the numbers $n_0\ldots n_k$
//! associated with $p_1\ldots p_k$.
//!
//! Comparatively few different number sequences $n_0\ldots n_k$ actually occur,
//! since most of the |n|'s are generally zero. Therefore the number sequences
//! are encoded in such a way that |trie_op|$(z_k)$ is only one byte long.
//! If |trie_op(@t$z_k$@>)<>min_quarterword|, when $p_1\ldots p_k$ has matched
//! the letters in |hc[(l-k+1)..l@,]| of language |t|,
//! we perform all of the required operations
//! for this pattern by carrying out the following little program: Set
//! |v:=trie_op(@t$z_k$@>)|. Then set |v:=v+op_start[t]|,
//! |hyf[l-hyf_distance[v]]:=@tmax@>(hyf[l-hyf_distance[v]], hyf_num[v])|,
//! and |v:=hyf_next[v]|; repeat, if necessary, until |v=min_quarterword|.
//!
//! @<Types...@>=
//! @!trie_pointer=0..trie_size; {an index into |trie|}
//!
//! @ @d trie_link(#)==trie[#].rh {``downward'' link in a trie}
//! @d trie_char(#)==trie[#].b1 {character matched at this trie location}
//! @d trie_op(#)==trie[#].b0 {program for hyphenation at this trie location}
//!
//! @<Glob...@>=
//! @!trie:array[trie_pointer] of two_halves; {|trie_link|, |trie_char|, |trie_op|}
//! @!hyf_distance:array[1..trie_op_size] of small_number; {position |k-j| of $n_j$}
//! @!hyf_num:array[1..trie_op_size] of small_number; {value of $n_j$}
//! @!hyf_next:array[1..trie_op_size] of quarterword; {continuation code}
//! @!op_start:array[ASCII_code] of 0..trie_op_size; {offset for current language}
//!
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
//! @ We have now completed the hyphenation routine, so the |line_break| procedure
//! is finished at last. Since the hyphenation exception table is fresh in our
//! minds, it's a good time to deal with the routine that adds new entries to it.
//!
//! When \TeX\ has scanned `\.{\\hyphenation}', it calls on a procedure named
//! |new_hyph_exceptions| to do the right thing.
//!
//! @d set_cur_lang==if language<=0 then cur_lang:=0
//!   else if language>255 then cur_lang:=0
//!   else cur_lang:=language
//!
//! @p procedure new_hyph_exceptions; {enters new exceptions}
//! label reswitch, exit, found, not_found;
//! var n:0..64; {length of current word; not always a |small_number|}
//! @!j:0..64; {an index into |hc|}
//! @!h:hyph_pointer; {an index into |hyph_word| and |hyph_list|}
//! @!k:str_number; {an index into |str_start|}
//! @!p:pointer; {head of a list of hyphen positions}
//! @!q:pointer; {used when creating a new node for list |p|}
//! @!s,@!t:str_number; {strings being compared or stored}
//! @!u,@!v:pool_pointer; {indices into |str_pool|}
//! begin scan_left_brace; {a left brace must follow \.{\\hyphenation}}
//! set_cur_lang;
//! @<Enter as many hyphenation exceptions as are listed,
//! until coming to a right brace; then |return|@>;
//! exit:end;
//!
//! @ @<Enter as many...@>=
//! n:=0; p:=null;
//! loop@+  begin get_x_token;
//!   reswitch: case cur_cmd of
//!   letter,other_char,char_given:@<Append a new letter or hyphen@>;
//!   char_num: begin scan_char_num; cur_chr:=cur_val; cur_cmd:=char_given;
//!     goto reswitch;
//!     end;
//!   spacer,right_brace: begin if n>1 then @<Enter a hyphenation exception@>;
//!     if cur_cmd=right_brace then return;
//!     n:=0; p:=null;
//!     end;
//!   othercases @<Give improper \.{\\hyphenation} error@>
//!   endcases;
//!   end
//!
//! @ @<Give improper \.{\\hyph...@>=
//! begin print_err("Improper "); print_esc("hyphenation");
//! @.Improper \\hyphenation...@>
//!   print(" will be flushed");
//! help2("Hyphenation exceptions must contain only letters")@/
//!   ("and hyphens. But continue; I'll forgive and forget.");
//! error;
//! end
//!
//! @ @<Append a new letter or hyphen@>=
//! if cur_chr="-" then @<Append the value |n| to list |p|@>
//! else  begin if lc_code(cur_chr)=0 then
//!     begin print_err("Not a letter");
//! @.Not a letter@>
//!     help2("Letters in \hyphenation words must have \lccode>0.")@/
//!       ("Proceed; I'll ignore the character I just read.");
//!     error;
//!     end
//!   else if n<63 then
//!     begin incr(n); hc[n]:=lc_code(cur_chr);
//!     end;
//!   end
//!
//! @ @<Append the value |n| to list |p|@>=
//! begin if n<63 then
//!   begin q:=get_avail; link(q):=p; info(q):=n; p:=q;
//!   end;
//! end
//!
//! @ @<Enter a hyphenation exception@>=
//! begin incr(n); hc[n]:=cur_lang; str_room(n); h:=0;
//! for j:=1 to n do
//!   begin h:=(h+h+hc[j]) mod hyph_size;
//!   append_char(hc[j]);
//!   end;
//! s:=make_string;
//! @<Insert the \(p)pair |(s,p)| into the exception table@>;
//! end
//!
//! @ @<Insert the \(p)pair |(s,p)|...@>=
//! if hyph_count=hyph_size then overflow("exception dictionary",hyph_size);
//! @:TeX capacity exceeded exception dictionary}{\quad exception dictionary@>
//! incr(hyph_count);
//! while hyph_word[h]<>0 do
//!   begin @<If the string |hyph_word[h]| is less than \(or)or equal to
//!   |s|, interchange |(hyph_word[h],hyph_list[h])| with |(s,p)|@>;
//!   if h>0 then decr(h)@+else h:=hyph_size;
//!   end;
//! hyph_word[h]:=s; hyph_list[h]:=p
//!
//! @ @<If the string |hyph_word[h]| is less than \(or)...@>=
//! k:=hyph_word[h];
//! if length(k)<length(s) then goto found;
//! if length(k)>length(s) then goto not_found;
//! u:=str_start[k]; v:=str_start[s];
//! repeat if str_pool[u]<str_pool[v] then goto found;
//! if str_pool[u]>str_pool[v] then goto not_found;
//! incr(u); incr(v);
//! until u=str_start[k+1];
//! found:q:=hyph_list[h]; hyph_list[h]:=p; p:=q;@/
//! t:=hyph_word[h]; hyph_word[h]:=s; s:=t;
//! not_found:
//!
