//! @ @<Set init...@>=
//! TEX_format_default:='TeXformats:plain.fmt';
//! @.TeXformats@>
//! @.plain@>
//! @^system dependencies@>
//!
//! @ @<Check the ``constant'' values for consistency@>=
//! if format_default_length>file_name_size then bad:=31;
//!
//! @ Here is the messy routine that was just mentioned. It sets |name_of_file|
//! from the first |n| characters of |TEX_format_default|, followed by
//! |buffer[a..b]|, followed by the last |format_ext_length| characters of
//! |TEX_format_default|.
//!
//! We dare not give error messages here, since \TeX\ calls this routine before
//! the |error| routine is ready to roll. Instead, we simply drop excess characters,
//! since the error will be detected in another way when a strange file name
//! isn't found.
//! @^system dependencies@>
//!
//! @p procedure pack_buffered_name(@!n:small_number;@!a,@!b:integer);
//! var k:integer; {number of positions filled in |name_of_file|}
//! @!c: ASCII_code; {character being packed}
//! @!j:integer; {index into |buffer| or |TEX_format_default|}
//! begin if n+b-a+1+format_ext_length>file_name_size then
//!   b:=a+file_name_size-n-1-format_ext_length;
//! k:=0;
//! for j:=1 to n do append_to_name(xord[TEX_format_default[j]]);
//! for j:=a to b do append_to_name(buffer[j]);
//! for j:=format_default_length-format_ext_length+1 to format_default_length do
//!   append_to_name(xord[TEX_format_default[j]]);
//! if k<=file_name_size then name_length:=k@+else name_length:=file_name_size;
//! for k:=name_length+1 to file_name_size do name_of_file[k]:=' ';
//! end;
//!
//! @ Here is the only place we use |pack_buffered_name|. This part of the program
//! becomes active when a ``virgin'' \TeX\ is trying to get going, just after
//! the preliminary initialization, or when the user is substituting another
//! format file by typing `\.\&' after the initial `\.{**}' prompt.  The buffer
//! contains the first line of input in |buffer[loc..(last-1)]|, where
//! |loc<last| and |buffer[loc]<>" "|.
//!
//! @<Declare the function called |open_fmt_file|@>=
//! function open_fmt_file:boolean;
//! label found,exit;
//! var j:0..buf_size; {the first space after the format file name}
//! begin j:=loc;
//! if buffer[loc]="&" then
//!   begin incr(loc); j:=loc; buffer[last]:=" ";
//!   while buffer[j]<>" " do incr(j);
//!   pack_buffered_name(0,loc,j-1); {try first without the system file area}
//!   if w_open_in(fmt_file) then goto found;
//!   pack_buffered_name(format_area_length,loc,j-1);
//!     {now try the system format file area}
//!   if w_open_in(fmt_file) then goto found;
//!   wake_up_terminal;
//!   wterm_ln('Sorry, I can''t find that format;',' will try PLAIN.');
//! @.Sorry, I can't find...@>
//!   update_terminal;
//!   end;
//!   {now pull out all the stops: try for the system \.{plain} file}
//! pack_buffered_name(format_default_length-format_ext_length,1,0);
//! if not w_open_in(fmt_file) then
//!   begin wake_up_terminal;
//!   wterm_ln('I can''t find the PLAIN format file!');
//! @.I can't find PLAIN...@>
//! @.plain@>
//!   open_fmt_file:=false; return;
//!   end;
//! found:loc:=j; open_fmt_file:=true;
//! exit:end;
//!
