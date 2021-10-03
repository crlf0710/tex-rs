//! ` `
// @<Show equivalent |n|, in region 4@>=
#[cfg(feature = "statistics")]
pub(crate) macro Show_equivalent_n__in_region_4($globals:expr, $n:expr) {{
    // if n=par_shape_loc then
    if ($n as integer) == par_shape_loc as integer {
        // begin print_esc("parshape"); print_char("=");
        // if par_shape_ptr=null then print_char("0")
        // else print_int(info(par_shape_ptr));
        // end
        todo!("par_shape_loc");
    }
    // else if n<toks_base then
    else if ($n as integer) < toks_base as integer {
        // begin print_cmd_chr(assign_toks,n); print_char("=");
        // if equiv(n)<>null then show_token_list(link(equiv(n)),null,32);
        // end
        todo!("assign_toks");
    }
    // else if n<box_base then
    else if ($n as integer) < box_base as integer {
        // begin print_esc("toks"); print_int(n-toks_base); print_char("=");
        // if equiv(n)<>null then show_token_list(link(equiv(n)),null,32);
        // end
        todo!("toks");
    }
    // else if n<cur_font_loc then
    else if ($n as integer) < cur_font_loc as integer {
        // begin print_esc("box"); print_int(n-box_base); print_char("=");
        // if equiv(n)=null then print("void")
        // else  begin depth_threshold:=0; breadth_max:=1; show_node_list(equiv(n));
        //   end;
        // end
        todo!("box");
    }
    // else if n<cat_code_base then @<Show the font identifier in |eqtb[n]|@>
    else if ($n as integer) < cat_code_base as integer {
        crate::section_0234::Show_the_font_identifier_in_eqtb_n!($globals, $n);
    }
    // else @<Show the halfword code in |eqtb[n]|@>
    else {
        todo!("show halfword code");
    }
    use crate::pascal::integer;
    use crate::section_0230::box_base;
    use crate::section_0230::cat_code_base;
    use crate::section_0230::cur_font_loc;
    use crate::section_0230::par_shape_loc;
    use crate::section_0230::toks_base;
}}
