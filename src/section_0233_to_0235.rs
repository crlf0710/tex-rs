//! @ @<Show equivalent |n|, in region 4@>=
//! if n=par_shape_loc then
//!   begin print_esc("parshape"); print_char("=");
//!   if par_shape_ptr=null then print_char("0")
//!   else print_int(info(par_shape_ptr));
//!   end
//! else if n<toks_base then
//!   begin print_cmd_chr(assign_toks,n); print_char("=");
//!   if equiv(n)<>null then show_token_list(link(equiv(n)),null,32);
//!   end
//! else if n<box_base then
//!   begin print_esc("toks"); print_int(n-toks_base); print_char("=");
//!   if equiv(n)<>null then show_token_list(link(equiv(n)),null,32);
//!   end
//! else if n<cur_font_loc then
//!   begin print_esc("box"); print_int(n-box_base); print_char("=");
//!   if equiv(n)=null then print("void")
//!   else  begin depth_threshold:=0; breadth_max:=1; show_node_list(equiv(n));
//!     end;
//!   end
//! else if n<cat_code_base then @<Show the font identifier in |eqtb[n]|@>
//! else @<Show the halfword code in |eqtb[n]|@>
//!
//! @ @<Show the font identifier in |eqtb[n]|@>=
//! begin if n=cur_font_loc then print("current font")
//! else if n<math_font_base+16 then
//!   begin print_esc("textfont"); print_int(n-math_font_base);
//!   end
//! else if n<math_font_base+32 then
//!   begin print_esc("scriptfont"); print_int(n-math_font_base-16);
//!   end
//! else  begin print_esc("scriptscriptfont"); print_int(n-math_font_base-32);
//!   end;
//! print_char("=");@/
//! print_esc(hash[font_id_base+equiv(n)].rh);
//!   {that's |font_id_text(equiv(n))|}
//! end
//!
//! @ @<Show the halfword code in |eqtb[n]|@>=
//! if n<math_code_base then
//!   begin if n<lc_code_base then
//!     begin print_esc("catcode"); print_int(n-cat_code_base);
//!     end
//!   else if n<uc_code_base then
//!     begin print_esc("lccode"); print_int(n-lc_code_base);
//!     end
//!   else if n<sf_code_base then
//!     begin print_esc("uccode"); print_int(n-uc_code_base);
//!     end
//!   else  begin print_esc("sfcode"); print_int(n-sf_code_base);
//!     end;
//!   print_char("="); print_int(equiv(n));
//!   end
//! else  begin print_esc("mathcode"); print_int(n-math_code_base);
//!   print_char("="); print_int(ho(equiv(n)));
//!   end
//!
