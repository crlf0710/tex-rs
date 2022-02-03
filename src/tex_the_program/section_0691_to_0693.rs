//! @ Here are some simple routines used in the display of noads.
//!
//! @<Declare procedures needed for displaying the elements of mlists@>=
//! procedure print_fam_and_char(@!p:pointer); {prints family and character}
//! begin print_esc("fam"); print_int(fam(p)); print_char(" ");
//! print_ASCII(qo(character(p)));
//! end;
//! @#
//! procedure print_delimiter(@!p:pointer); {prints a delimiter as 24-bit hex value}
//! var a:integer; {accumulator}
//! begin a:=small_fam(p)*256+qo(small_char(p));
//! a:=a*@"1000+large_fam(p)*256+qo(large_char(p));
//! if a<0 then print_int(a) {this should never happen}
//! else print_hex(a);
//! end;
//!
//! @ The next subroutine will descend to another level of recursion when a
//! subsidiary mlist needs to be displayed. The parameter |c| indicates what
//! character is to become part of the recursion history. An empty mlist is
//! distinguished from a field with |math_type(p)=empty|, because these are
//! not equivalent (as explained above).
//! @^recursion@>
//!
//! @<Declare procedures needed for displaying...@>=
//! procedure@?show_info; forward;@t\2@>@?{|show_node_list(info(temp_ptr))|}
//! procedure print_subsidiary_data(@!p:pointer;@!c:ASCII_code);
//!   {display a noad field}
//! begin if cur_length>=depth_threshold then
//!   begin if math_type(p)<>empty then print(" []");
//!   end
//! else  begin append_char(c); {include |c| in the recursion history}
//!   temp_ptr:=p; {prepare for |show_info| if recursion is needed}
//!   case math_type(p) of
//!   math_char: begin print_ln; print_current_string; print_fam_and_char(p);
//!     end;
//!   sub_box: show_info; {recursive call}
//!   sub_mlist: if info(p)=null then
//!       begin print_ln; print_current_string; print("{}");
//!       end
//!     else show_info; {recursive call}
//!   othercases do_nothing {|empty|}
//!   endcases;@/
//!   flush_char; {remove |c| from the recursion history}
//!   end;
//! end;
//!
//! @ The inelegant introduction of |show_info| in the code above seems better
//! than the alternative of using \PASCAL's strange |forward| declaration for a
//! procedure with parameters. The \PASCAL\ convention about dropping parameters
//! from a post-|forward| procedure is, frankly, so intolerable to the author
//! of \TeX\ that he would rather stoop to communication via a global temporary
//! variable. (A similar stoopidity occurred with respect to |hlist_out| and
//! |vlist_out| above, and it will occur with respect to |mlist_to_hlist| below.)
//! @^Knuth, Donald Ervin@>
//! @:PASCAL}{\PASCAL@>
//!
//! @p procedure show_info; {the reader will kindly forgive this}
//! begin show_node_list(info(temp_ptr));
//! end;
//!
