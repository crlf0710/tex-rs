//! @ @<Put each...@>=
//! primitive("parindent",assign_dimen,dimen_base+par_indent_code);@/
//! @!@:par_indent_}{\.{\\parindent} primitive@>
//! primitive("mathsurround",assign_dimen,dimen_base+math_surround_code);@/
//! @!@:math_surround_}{\.{\\mathsurround} primitive@>
//! primitive("lineskiplimit",assign_dimen,dimen_base+line_skip_limit_code);@/
//! @!@:line_skip_limit_}{\.{\\lineskiplimit} primitive@>
//! primitive("hsize",assign_dimen,dimen_base+hsize_code);@/
//! @!@:hsize_}{\.{\\hsize} primitive@>
//! primitive("vsize",assign_dimen,dimen_base+vsize_code);@/
//! @!@:vsize_}{\.{\\vsize} primitive@>
//! primitive("maxdepth",assign_dimen,dimen_base+max_depth_code);@/
//! @!@:max_depth_}{\.{\\maxdepth} primitive@>
//! primitive("splitmaxdepth",assign_dimen,dimen_base+split_max_depth_code);@/
//! @!@:split_max_depth_}{\.{\\splitmaxdepth} primitive@>
//! primitive("boxmaxdepth",assign_dimen,dimen_base+box_max_depth_code);@/
//! @!@:box_max_depth_}{\.{\\boxmaxdepth} primitive@>
//! primitive("hfuzz",assign_dimen,dimen_base+hfuzz_code);@/
//! @!@:hfuzz_}{\.{\\hfuzz} primitive@>
//! primitive("vfuzz",assign_dimen,dimen_base+vfuzz_code);@/
//! @!@:vfuzz_}{\.{\\vfuzz} primitive@>
//! primitive("delimitershortfall",
//!   assign_dimen,dimen_base+delimiter_shortfall_code);@/
//! @!@:delimiter_shortfall_}{\.{\\delimitershortfall} primitive@>
//! primitive("nulldelimiterspace",
//!   assign_dimen,dimen_base+null_delimiter_space_code);@/
//! @!@:null_delimiter_space_}{\.{\\nulldelimiterspace} primitive@>
//! primitive("scriptspace",assign_dimen,dimen_base+script_space_code);@/
//! @!@:script_space_}{\.{\\scriptspace} primitive@>
//! primitive("predisplaysize",assign_dimen,dimen_base+pre_display_size_code);@/
//! @!@:pre_display_size_}{\.{\\predisplaysize} primitive@>
//! primitive("displaywidth",assign_dimen,dimen_base+display_width_code);@/
//! @!@:display_width_}{\.{\\displaywidth} primitive@>
//! primitive("displayindent",assign_dimen,dimen_base+display_indent_code);@/
//! @!@:display_indent_}{\.{\\displayindent} primitive@>
//! primitive("overfullrule",assign_dimen,dimen_base+overfull_rule_code);@/
//! @!@:overfull_rule_}{\.{\\overfullrule} primitive@>
//! primitive("hangindent",assign_dimen,dimen_base+hang_indent_code);@/
//! @!@:hang_indent_}{\.{\\hangindent} primitive@>
//! primitive("hoffset",assign_dimen,dimen_base+h_offset_code);@/
//! @!@:h_offset_}{\.{\\hoffset} primitive@>
//! primitive("voffset",assign_dimen,dimen_base+v_offset_code);@/
//! @!@:v_offset_}{\.{\\voffset} primitive@>
//! primitive("emergencystretch",assign_dimen,dimen_base+emergency_stretch_code);@/
//! @!@:emergency_stretch_}{\.{\\emergencystretch} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_dimen: if chr_code<scaled_base then
//!     print_length_param(chr_code-dimen_base)
//!   else  begin print_esc("dimen"); print_int(chr_code-scaled_base);
//!     end;
//!
//! @ @<Initialize table entries...@>=
//! for k:=dimen_base to eqtb_size do eqtb[k].sc:=0;
//!
//! @ @<Show equivalent |n|, in region 6@>=
//! begin if n<scaled_base then print_length_param(n-dimen_base)
//! else  begin print_esc("dimen"); print_int(n-scaled_base);
//!   end;
//! print_char("="); print_scaled(eqtb[n].sc); print("pt");
//! end
//!
//! @ Here is a procedure that displays the contents of |eqtb[n]|
//! symbolically.
//!
//! @p@t\4@>@<Declare the procedure called |print_cmd_chr|@>@;@/
//! @!stat procedure show_eqtb(@!n:pointer);
//! begin if n<active_base then print_char("?") {this can't happen}
//! else if n<glue_base then @<Show equivalent |n|, in region 1 or 2@>
//! else if n<local_base then @<Show equivalent |n|, in region 3@>
//! else if n<int_base then @<Show equivalent |n|, in region 4@>
//! else if n<dimen_base then @<Show equivalent |n|, in region 5@>
//! else if n<=eqtb_size then @<Show equivalent |n|, in region 6@>
//! else print_char("?"); {this can't happen either}
//! end;
//! tats
//!
