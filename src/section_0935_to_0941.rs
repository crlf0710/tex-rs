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
