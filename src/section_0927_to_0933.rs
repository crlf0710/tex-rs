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
