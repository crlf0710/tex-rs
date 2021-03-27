//! @ @<Check that the necessary fonts...@>=
//! if (font_params[fam_fnt(2+text_size)]<total_mathsy_params)or@|
//!    (font_params[fam_fnt(2+script_size)]<total_mathsy_params)or@|
//!    (font_params[fam_fnt(2+script_script_size)]<total_mathsy_params) then
//!   begin print_err("Math formula deleted: Insufficient symbol fonts");@/
//! @.Math formula deleted...@>
//!   help3("Sorry, but I can't typeset math unless \textfont 2")@/
//!     ("and \scriptfont 2 and \scriptscriptfont 2 have all")@/
//!     ("the \fontdimen values needed in math symbol fonts.");
//!   error; flush_math; danger:=true;
//!   end
//! else if (font_params[fam_fnt(3+text_size)]<total_mathex_params)or@|
//!    (font_params[fam_fnt(3+script_size)]<total_mathex_params)or@|
//!    (font_params[fam_fnt(3+script_script_size)]<total_mathex_params) then
//!   begin print_err("Math formula deleted: Insufficient extension fonts");@/
//!   help3("Sorry, but I can't typeset math unless \textfont 3")@/
//!     ("and \scriptfont 3 and \scriptscriptfont 3 have all")@/
//!     ("the \fontdimen values needed in math extension fonts.");
//!   error; flush_math; danger:=true;
//!   end
//!
//! @ The |unsave| is done after everything else here; hence an appearance of
//! `\.{\\mathsurround}' inside of `\.{\$...\$}' affects the spacing at these
//! particular \.\$'s. This is consistent with the conventions of
//! `\.{\$\$...\$\$}', since `\.{\\abovedisplayskip}' inside a display affects the
//! space above that display.
//!
//! @<Finish math in text@>=
//! begin tail_append(new_math(math_surround,before));
//! cur_mlist:=p; cur_style:=text_style; mlist_penalties:=(mode>0); mlist_to_hlist;
//! link(tail):=link(temp_head);
//! while link(tail)<>null do tail:=link(tail);
//! tail_append(new_math(math_surround,after));
//! space_factor:=1000; unsave;
//! end
//!
//! @ \TeX\ gets to the following part of the program when the first `\.\$' ending
//! a display has been scanned.
//!
//! @<Check that another \.\$ follows@>=
//! begin get_x_token;
//! if cur_cmd<>math_shift then
//!   begin print_err("Display math should end with $$");
//! @.Display math...with \$\$@>
//!   help2("The `$' that I just saw supposedly matches a previous `$$'.")@/
//!     ("So I shall assume that you typed `$$' both times.");
//!   back_error;
//!   end;
//! end
//!
//! @ We have saved the worst for last: The fussiest part of math mode processing
//! occurs when a displayed formula is being centered and placed with an optional
//! equation number.
//!
//! @<Local variables for finishing...@>=
//! @!b:pointer; {box containing the equation}
//! @!w:scaled; {width of the equation}
//! @!z:scaled; {width of the line}
//! @!e:scaled; {width of equation number}
//! @!q:scaled; {width of equation number plus space to separate from equation}
//! @!d:scaled; {displacement of equation in the line}
//! @!s:scaled; {move the line right this much}
//! @!g1,@!g2:small_number; {glue parameter codes for before and after}
//! @!r:pointer; {kern node used to position the display}
//! @!t:pointer; {tail of adjustment list}
