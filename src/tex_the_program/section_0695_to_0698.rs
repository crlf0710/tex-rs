//! @ @<Display choice node |p|@>=
//! begin print_esc("mathchoice");
//! append_char("D"); show_node_list(display_mlist(p)); flush_char;
//! append_char("T"); show_node_list(text_mlist(p)); flush_char;
//! append_char("S"); show_node_list(script_mlist(p)); flush_char;
//! append_char("s"); show_node_list(script_script_mlist(p)); flush_char;
//! end
//!
//! @ @<Display normal noad |p|@>=
//! begin case type(p) of
//! ord_noad: print_esc("mathord");
//! op_noad: print_esc("mathop");
//! bin_noad: print_esc("mathbin");
//! rel_noad: print_esc("mathrel");
//! open_noad: print_esc("mathopen");
//! close_noad: print_esc("mathclose");
//! punct_noad: print_esc("mathpunct");
//! inner_noad: print_esc("mathinner");
//! over_noad: print_esc("overline");
//! under_noad: print_esc("underline");
//! vcenter_noad: print_esc("vcenter");
//! radical_noad: begin print_esc("radical"); print_delimiter(left_delimiter(p));
//!   end;
//! accent_noad: begin print_esc("accent"); print_fam_and_char(accent_chr(p));
//!   end;
//! left_noad: begin print_esc("left"); print_delimiter(delimiter(p));
//!   end;
//! right_noad: begin print_esc("right"); print_delimiter(delimiter(p));
//!   end;
//! end;
//! if subtype(p)<>normal then
//!   if subtype(p)=limits then print_esc("limits")
//!   else print_esc("nolimits");
//! if type(p)<left_noad then print_subsidiary_data(nucleus(p),".");
//! print_subsidiary_data(supscr(p),"^");
//! print_subsidiary_data(subscr(p),"_");
//! end
//!
//! @ @<Display fraction noad |p|@>=
//! begin print_esc("fraction, thickness ");
//! if thickness(p)=default_code then print("= default")
//! else print_scaled(thickness(p));
//! if (small_fam(left_delimiter(p))<>0)or@+
//!   (small_char(left_delimiter(p))<>min_quarterword)or@|
//!   (large_fam(left_delimiter(p))<>0)or@|
//!   (large_char(left_delimiter(p))<>min_quarterword) then
//!   begin print(", left-delimiter "); print_delimiter(left_delimiter(p));
//!   end;
//! if (small_fam(right_delimiter(p))<>0)or@|
//!   (small_char(right_delimiter(p))<>min_quarterword)or@|
//!   (large_fam(right_delimiter(p))<>0)or@|
//!   (large_char(right_delimiter(p))<>min_quarterword) then
//!   begin print(", right-delimiter "); print_delimiter(right_delimiter(p));
//!   end;
//! print_subsidiary_data(numerator(p),"\");
//! print_subsidiary_data(denominator(p),"/");
//! end
//!
//! @ That which can be displayed can also be destroyed.
//!
//! @<Cases of |flush_node_list| that arise...@>=
//! style_node: begin free_node(p,style_node_size); goto done;
//!   end;
//! choice_node:begin flush_node_list(display_mlist(p));
//!   flush_node_list(text_mlist(p));
//!   flush_node_list(script_mlist(p));
//!   flush_node_list(script_script_mlist(p));
//!   free_node(p,style_node_size); goto done;
//!   end;
//! ord_noad,op_noad,bin_noad,rel_noad,open_noad,close_noad,punct_noad,inner_noad,
//!   radical_noad,over_noad,under_noad,vcenter_noad,accent_noad:@t@>@;@/
//!   begin if math_type(nucleus(p))>=sub_box then
//!     flush_node_list(info(nucleus(p)));
//!   if math_type(supscr(p))>=sub_box then
//!     flush_node_list(info(supscr(p)));
//!   if math_type(subscr(p))>=sub_box then
//!     flush_node_list(info(subscr(p)));
//!   if type(p)=radical_noad then free_node(p,radical_noad_size)
//!   else if type(p)=accent_noad then free_node(p,accent_noad_size)
//!   else free_node(p,noad_size);
//!   goto done;
//!   end;
//! left_noad,right_noad: begin free_node(p,noad_size); goto done;
//!   end;
//! fraction_noad: begin flush_node_list(info(numerator(p)));
//!   flush_node_list(info(denominator(p)));
//!   free_node(p,fraction_noad_size); goto done;
//!   end;
//!
