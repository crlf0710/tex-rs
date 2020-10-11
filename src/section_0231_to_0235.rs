//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_toks: if chr_code>=toks_base then
//!   begin print_esc("toks"); print_int(chr_code-toks_base);
//!   end
//! else  case chr_code of
//!   output_routine_loc: print_esc("output");
//!   every_par_loc: print_esc("everypar");
//!   every_math_loc: print_esc("everymath");
//!   every_display_loc: print_esc("everydisplay");
//!   every_hbox_loc: print_esc("everyhbox");
//!   every_vbox_loc: print_esc("everyvbox");
//!   every_job_loc: print_esc("everyjob");
//!   every_cr_loc: print_esc("everycr");
//!   othercases print_esc("errhelp")
//!   endcases;
//!
//! @ We initialize most things to null or undefined values. An undefined font
//! is represented by the internal code |font_base|.
//!
//! However, the character code tables are given initial values based on the
//! conventional interpretation of ASCII code. These initial values should
//! not be changed when \TeX\ is adapted for use with non-English languages;
//! all changes to the initialization conventions should be made in format
//! packages, not in \TeX\ itself, so that global interchange of formats is
//! possible.
//!
//! @d null_font==font_base
//! @d var_code==@'70000 {math code meaning ``use the current family''}
//!
//! @<Initialize table entries...@>=
//! par_shape_ptr:=null; eq_type(par_shape_loc):=shape_ref;
//! eq_level(par_shape_loc):=level_one;@/
//! for k:=output_routine_loc to toks_base+255 do
//!   eqtb[k]:=eqtb[undefined_control_sequence];
//! box(0):=null; eq_type(box_base):=box_ref; eq_level(box_base):=level_one;
//! for k:=box_base+1 to box_base+255 do eqtb[k]:=eqtb[box_base];
//! cur_font:=null_font; eq_type(cur_font_loc):=data;
//! eq_level(cur_font_loc):=level_one;@/
//! for k:=math_font_base to math_font_base+47 do eqtb[k]:=eqtb[cur_font_loc];
//! equiv(cat_code_base):=0; eq_type(cat_code_base):=data;
//! eq_level(cat_code_base):=level_one;@/
//! for k:=cat_code_base+1 to int_base-1 do eqtb[k]:=eqtb[cat_code_base];
//! for k:=0 to 255 do
//!   begin cat_code(k):=other_char; math_code(k):=hi(k); sf_code(k):=1000;
//!   end;
//! cat_code(carriage_return):=car_ret; cat_code(" "):=spacer;
//! cat_code("\"):=escape; cat_code("%"):=comment;
//! cat_code(invalid_code):=invalid_char; cat_code(null_code):=ignore;
//! for k:="0" to "9" do math_code(k):=hi(k+var_code);
//! for k:="A" to "Z" do
//!   begin cat_code(k):=letter; cat_code(k+"a"-"A"):=letter;@/
//!   math_code(k):=hi(k+var_code+@"100);
//!   math_code(k+"a"-"A"):=hi(k+"a"-"A"+var_code+@"100);@/
//!   lc_code(k):=k+"a"-"A"; lc_code(k+"a"-"A"):=k+"a"-"A";@/
//!   uc_code(k):=k; uc_code(k+"a"-"A"):=k;@/
//!   sf_code(k):=999;
//!   end;
//!
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
