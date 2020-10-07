//! @ @<Glob...@>=
//! @!str_pool:packed array[pool_pointer] of packed_ASCII_code; {the characters}
//! @!str_start : array[str_number] of pool_pointer; {the starting pointers}
//! @!pool_ptr : pool_pointer; {first unused position in |str_pool|}
//! @!str_ptr : str_number; {number of the current string being created}
//! @!init_pool_ptr : pool_pointer; {the starting value of |pool_ptr|}
//! @!init_str_ptr : str_number; {the starting value of |str_ptr|}
//!
//! @ Several of the elementary string operations are performed using \.{WEB}
//! macros instead of \PASCAL\ procedures, because many of the
//! operations are done quite frequently and we want to avoid the
//! overhead of procedure calls. For example, here is
//! a simple macro that computes the length of a string.
//! @.WEB@>
//!
//! @d length(#)==(str_start[#+1]-str_start[#]) {the number of characters
//!   in string number \#}
//!
//! @ The length of the current string is called |cur_length|:
//!
//! @d cur_length == (pool_ptr - str_start[str_ptr])
//!
//! @ Strings are created by appending character codes to |str_pool|.
//! The |append_char| macro, defined here, does not check to see if the
//! value of |pool_ptr| has gotten too high; this test is supposed to be
//! made before |append_char| is used. There is also a |flush_char|
//! macro, which erases the last character appended.
//!
//! To test if there is room to append |l| more characters to |str_pool|,
//! we shall write |str_room(l)|, which aborts \TeX\ and gives an
//! apologetic error message if there isn't enough room.
//!
//! @d append_char(#) == {put |ASCII_code| \# at the end of |str_pool|}
//! begin str_pool[pool_ptr]:=si(#); incr(pool_ptr);
//! end
//! @d flush_char == decr(pool_ptr) {forget the last character in the pool}
//! @d str_room(#) == {make sure that the pool hasn't overflowed}
//!   begin if pool_ptr+# > pool_size then
//!   overflow("pool size",pool_size-init_pool_ptr);
//! @:TeX capacity exceeded pool size}{\quad pool size@>
//!   end
//!
//! @ Once a sequence of characters has been appended to |str_pool|, it
//! officially becomes a string when the function |make_string| is called.
//! This function returns the identification number of the new string as its
//! value.
//!
//! @p function make_string : str_number; {current string enters the pool}
//! begin if str_ptr=max_strings then
//!   overflow("number of strings",max_strings-init_str_ptr);
//! @:TeX capacity exceeded number of strings}{\quad number of strings@>
//! incr(str_ptr); str_start[str_ptr]:=pool_ptr;
//! make_string:=str_ptr-1;
//! end;
//!
//! @ To destroy the most recently made string, we say |flush_string|.
//!
//! @d flush_string==begin decr(str_ptr); pool_ptr:=str_start[str_ptr];
//!   end
//!
//! @ The following subroutine compares string |s| with another string of the
//! same length that appears in |buffer| starting at position |k|;
//! the result is |true| if and only if the strings are equal.
//! Empirical tests indicate that |str_eq_buf| is used in such a way that
//! it tends to return |true| about 80 percent of the time.
//!
//! @p function str_eq_buf(@!s:str_number;@!k:integer):boolean;
//!   {test equality of strings}
//! label not_found; {loop exit}
//! var j: pool_pointer; {running index}
//! @!result: boolean; {result of comparison}
//! begin j:=str_start[s];
//! while j<str_start[s+1] do
//!   begin if so(str_pool[j])<>buffer[k] then
//!     begin result:=false; goto not_found;
//!     end;
//!   incr(j); incr(k);
//!   end;
//! result:=true;
//! not_found: str_eq_buf:=result;
//! end;
//!
//! @ Here is a similar routine, but it compares two strings in the string pool,
//! and it does not assume that they have the same length.
//!
//! @p function str_eq_str(@!s,@!t:str_number):boolean;
//!   {test equality of strings}
//! label not_found; {loop exit}
//! var j,@!k: pool_pointer; {running indices}
//! @!result: boolean; {result of comparison}
//! begin result:=false;
//! if length(s)<>length(t) then goto not_found;
//! j:=str_start[s]; k:=str_start[t];
//! while j<str_start[s+1] do
//!   begin if str_pool[j]<>str_pool[k] then goto not_found;
//!   incr(j); incr(k);
//!   end;
//! result:=true;
//! not_found: str_eq_str:=result;
//! end;
//!
//! @ The initial values of |str_pool|, |str_start|, |pool_ptr|,
//! and |str_ptr| are computed by the \.{INITEX} program, based in part
//! on the information that \.{WEB} has output while processing \TeX.
//! @.INITEX@>
//! @^string pool@>
//!
//! @p @!init function get_strings_started:boolean; {initializes the string pool,
//!   but returns |false| if something goes wrong}
//! label done,exit;
//! var k,@!l:0..255; {small indices or counters}
//! @!m,@!n:text_char; {characters input from |pool_file|}
//! @!g:str_number; {garbage}
//! @!a:integer; {accumulator for check sum}
//! @!c:boolean; {check sum has been checked}
//! begin pool_ptr:=0; str_ptr:=0; str_start[0]:=0;
//! @<Make the first 256 strings@>;
//! @<Read the other strings from the \.{TEX.POOL} file and return |true|,
//!   or give an error message and return |false|@>;
//! exit:end;
//! tini
//!
//! @ @d app_lc_hex(#)==l:=#;
//!   if l<10 then append_char(l+"0")@+else append_char(l-10+"a")
//!
//! @<Make the first 256...@>=
//! for k:=0 to 255 do
//!   begin if (@<Character |k| cannot be printed@>) then
//!     begin append_char("^"); append_char("^");
//!     if k<@'100 then append_char(k+@'100)
//!     else if k<@'200 then append_char(k-@'100)
//!     else begin app_lc_hex(k div 16); app_lc_hex(k mod 16);
//!       end;
//!     end
//!   else append_char(k);
//!   g:=make_string;
//!   end
//!
//! @ The first 128 strings will contain 95 standard ASCII characters, and the
//! other 33 characters will be printed in three-symbol form like `\.{\^\^A}'
//! unless a system-dependent change is made here. Installations that have
//! an extended character set, where for example |xchr[@'32]=@t\.{\'^^Z\'}@>|,
//! would like string @'32 to be the single character @'32 instead of the
//! three characters @'136, @'136, @'132 (\.{\^\^Z}). On the other hand,
//! even people with an extended character set will want to represent string
//! @'15 by \.{\^\^M}, since @'15 is |carriage_return|; the idea is to
//! produce visible strings instead of tabs or line-feeds or carriage-returns
//! or bell-rings or characters that are treated anomalously in text files.
//!
//! Unprintable characters of codes 128--255 are, similarly, rendered
//! \.{\^\^80}--\.{\^\^ff}.
//!
//! The boolean expression defined here should be |true| unless \TeX\
//! internal code number~|k| corresponds to a non-troublesome visible
//! symbol in the local character set.  An appropriate formula for the
//! extended character set recommended in {\sl The \TeX book\/} would, for
//! example, be `|k in [0,@'10..@'12,@'14,@'15,@'33,@'177..@'377]|'.
//! If character |k| cannot be printed, and |k<@'200|, then character |k+@'100| or
//! |k-@'100| must be printable; moreover, ASCII codes |[@'41..@'46,
//! @'60..@'71, @'136, @'141..@'146, @'160..@'171]| must be printable.
//! Thus, at least 81 printable characters are needed.
//! @:TeXbook}{\sl The \TeX book@>
//! @^character set dependencies@>
//! @^system dependencies@>
//!
//! @<Character |k| cannot be printed@>=
//!   (k<" ")or(k>"~")
//!
//! @ When the \.{WEB} system program called \.{TANGLE} processes the \.{TEX.WEB}
//! description that you are now reading, it outputs the \PASCAL\ program
//! \.{TEX.PAS} and also a string pool file called \.{TEX.POOL}. The \.{INITEX}
//! @.WEB@>@.INITEX@>
//! program reads the latter file, where each string appears as a two-digit decimal
//! length followed by the string itself, and the information is recorded in
//! \TeX's string memory.
//!
//! @<Glob...@>=
//! @!init @!pool_file:alpha_file; {the string-pool file output by \.{TANGLE}}
//! tini
//!
//! @ @d bad_pool(#)==begin wake_up_terminal; write_ln(term_out,#);
//!   a_close(pool_file); get_strings_started:=false; return;
//!   end
//! @<Read the other strings...@>=
//! name_of_file:=pool_name; {we needn't set |name_length|}
//! if a_open_in(pool_file) then
//!   begin c:=false;
//!   repeat @<Read one string, but return |false| if the
//!     string memory space is getting too tight for comfort@>;
//!   until c;
//!   a_close(pool_file); get_strings_started:=true;
//!   end
//! else  bad_pool('! I can''t read TEX.POOL.')
//! @.I can't read TEX.POOL@>
//!
//! @ @<Read one string...@>=
//! begin if eof(pool_file) then bad_pool('! TEX.POOL has no check sum.');
//! @.TEX.POOL has no check sum@>
//! read(pool_file,m,n); {read two digits of string length}
//! if m='*' then @<Check the pool check sum@>
//! else  begin if (xord[m]<"0")or(xord[m]>"9")or@|
//!       (xord[n]<"0")or(xord[n]>"9") then
//!     bad_pool('! TEX.POOL line doesn''t begin with two digits.');
//! @.TEX.POOL line doesn't...@>
//!   l:=xord[m]*10+xord[n]-"0"*11; {compute the length}
//!   if pool_ptr+l+string_vacancies>pool_size then
//!     bad_pool('! You have to increase POOLSIZE.');
//! @.You have to increase POOLSIZE@>
//!   for k:=1 to l do
//!     begin if eoln(pool_file) then m:=' '@+else read(pool_file,m);
//!     append_char(xord[m]);
//!     end;
//!   read_ln(pool_file); g:=make_string;
//!   end;
//! end
//!
//! @ The \.{WEB} operation \.{@@\$} denotes the value that should be at the
//! end of this \.{TEX.POOL} file; any other value means that the wrong pool
//! file has been loaded.
//! @^check sum@>
//!
//! @<Check the pool check sum@>=
//! begin a:=0; k:=1;
//! loop@+  begin if (xord[n]<"0")or(xord[n]>"9") then
//!   bad_pool('! TEX.POOL check sum doesn''t have nine digits.');
//! @.TEX.POOL check sum...@>
//!   a:=10*a+xord[n]-"0";
//!   if k=9 then goto done;
//!   incr(k); read(pool_file,n);
//!   end;
//! done: if a<>@$ then bad_pool('! TEX.POOL doesn''t match; TANGLE me again.');
//! @.TEX.POOL doesn't match@>
//! c:=true;
//! end
//!
