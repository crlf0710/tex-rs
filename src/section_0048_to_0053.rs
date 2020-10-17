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
