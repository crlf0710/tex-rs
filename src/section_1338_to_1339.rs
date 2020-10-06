//! @* \[52] Debugging.
//! Once \TeX\ is working, you should be able to diagnose most errors with
//! the \.{\\show} commands and other diagnostic features. But for the initial
//! stages of debugging, and for the revelation of really deep mysteries, you
//! can compile \TeX\ with a few more aids, including the \PASCAL\ runtime
//! checks and its debugger. An additional routine called |debug_help|
//! will also come into play when you type `\.D' after an error message;
//! |debug_help| also occurs just before a fatal error causes \TeX\ to succumb.
//! @^debugging@>
//! @^system dependencies@>
//!
//! The interface to |debug_help| is primitive, but it is good enough when used
//! with a \PASCAL\ debugger that allows you to set breakpoints and to read
//! variables and change their values. After getting the prompt `\.{debug \#}', you
//! type either a negative number (this exits |debug_help|), or zero (this
//! goes to a location where you can set a breakpoint, thereby entering into
//! dialog with the \PASCAL\ debugger), or a positive number |m| followed by
//! an argument |n|. The meaning of |m| and |n| will be clear from the
//! program below. (If |m=13|, there is an additional argument, |l|.)
//! @.debug \#@>
//!
//! @d breakpoint=888 {place where a breakpoint is desirable}
//!
//! @<Last-minute...@>=
//! @!debug procedure debug_help; {routine to display various things}
//! label breakpoint,exit;
//! var k,@!l,@!m,@!n:integer;
//! begin loop begin wake_up_terminal;
//!   print_nl("debug # (-1 to exit):"); update_terminal;
//! @.debug \#@>
//!   read(term_in,m);
//!   if m<0 then return
//!   else if m=0 then
//!     begin goto breakpoint;@\ {go to every label at least once}
//!     breakpoint: m:=0; @{'BREAKPOINT'@}@\
//!     end
//!   else  begin read(term_in,n);
//!     case m of
//!     @t\4@>@<Numbered cases for |debug_help|@>@;
//!     othercases print("?")
//!     endcases;
//!     end;
//!   end;
//! exit:end;
//! gubed
//!
//! @ @<Numbered cases...@>=
//! 1: print_word(mem[n]); {display |mem[n]| in all forms}
//! 2: print_int(info(n));
//! 3: print_int(link(n));
//! 4: print_word(eqtb[n]);
//! 5: print_word(font_info[n]);
//! 6: print_word(save_stack[n]);
//! 7: show_box(n);
//!   {show a box, abbreviated by |show_box_depth| and |show_box_breadth|}
//! 8: begin breadth_max:=10000; depth_threshold:=pool_size-pool_ptr-10;
//!   show_node_list(n); {show a box in its entirety}
//!   end;
//! 9: show_token_list(n,null,1000);
//! 10: slow_print(n);
//! 11: check_mem(n>0); {check wellformedness; print new busy locations if |n>0|}
//! 12: search_mem(n); {look for pointers to |n|}
//! 13: begin read(term_in,l); print_cmd_chr(n,l);
//!   end;
//! 14: for k:=0 to n do print(buffer[k]);
//! 15: begin font_in_short_display:=null_font; short_display(n);
//!   end;
//! 16: panicking:=not panicking;
//!
