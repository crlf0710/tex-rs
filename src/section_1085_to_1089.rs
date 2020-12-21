//! @ When the right brace occurs at the end of an \.{\\hbox} or \.{\\vbox} or
//! \.{\\vtop} construction, the |package| routine comes into action. We might
//! also have to finish a paragraph that hasn't ended.
//!
//! @<Cases of |handle...@>=
//! hbox_group: package(0);
//! adjusted_hbox_group: begin adjust_tail:=adjust_head; package(0);
//!   end;
//! vbox_group: begin end_graf; package(0);
//!   end;
//! vtop_group: begin end_graf; package(vtop_code);
//!   end;
//!
//! @ @<Declare action...@>=
//! procedure package(@!c:small_number);
//! var h:scaled; {height of box}
//! @!p:pointer; {first node in a box}
//! @!d:scaled; {max depth}
//! begin d:=box_max_depth; unsave; save_ptr:=save_ptr-3;
//! if mode=-hmode then cur_box:=hpack(link(head),saved(2),saved(1))
//! else  begin cur_box:=vpackage(link(head),saved(2),saved(1),d);
//!   if c=vtop_code then @<Readjust the height and depth of |cur_box|,
//!     for \.{\\vtop}@>;
//!   end;
//! pop_nest; box_end(saved(0));
//! end;
//!
//! @ The height of a `\.{\\vtop}' box is inherited from the first item on its list,
//! if that item is an |hlist_node|, |vlist_node|, or |rule_node|; otherwise
//! the \.{\\vtop} height is zero.
//!
//!
//! @<Readjust the height...@>=
//! begin h:=0; p:=list_ptr(cur_box);
//! if p<>null then if type(p)<=rule_node then h:=height(p);
//! depth(cur_box):=depth(cur_box)-h+height(cur_box); height(cur_box):=h;
//! end
//!
//! @ A paragraph begins when horizontal-mode material occurs in vertical mode,
//! or when the paragraph is explicitly started by `\.{\\indent}' or
//! `\.{\\noindent}'.
//!
//! @<Put each...@>=
//! primitive("indent",start_par,1);
//! @!@:indent_}{\.{\\indent} primitive@>
//! primitive("noindent",start_par,0);
//! @!@:no_indent_}{\.{\\noindent} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! start_par: if chr_code=0 then print_esc("noindent")@+ else print_esc("indent");
//!
