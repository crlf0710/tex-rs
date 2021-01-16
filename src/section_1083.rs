//! @ Here is where we enter restricted horizontal mode or internal vertical
//! mode, in order to make a box.
//
// @<Initiate the construction of an hbox or vbox, then |return|@>=
macro_rules! Initiate_the_construction_of_an_hbox_or_vbox_then_return {
    ($globals:expr, $box_context:expr) => {{
        /// 0 or `vmode` or `hmode`
        let mut k: halfword;
        // begin k:=cur_chr-vtop_code; saved(0):=box_context;
        k = ($globals.cur_chr.get() - vtop_code) as _;
        saved!($globals, 0) = $box_context;
        // if k=hmode then
        if k == hmode as halfword {
            // if (box_context<box_flag)and(abs(mode)=vmode) then
            if $box_context < box_flag && mode!($globals).get().abs() == vmode {
                todo!("scan_spec1");
                // scan_spec(adjusted_hbox_group,true)
            }
            // else scan_spec(hbox_group,true)
            else {
                scan_spec($globals, hbox_group.into(), true)?;
            }
        }
        // else  begin if k=vmode then scan_spec(vbox_group,true)
        else {
            if k == vmode as halfword {
                scan_spec($globals, vbox_group.into(), true)?;
            }
            // else  begin scan_spec(vtop_group,true); k:=vmode;
            else {
                scan_spec($globals, vtop_group.into(), true)?;
                k = vmode as _;
                // end;
            }
            // normal_paragraph;
            normal_paragraph($globals)?;
            // end;
        }
        // push_nest; mode:=-k;
        push_nest($globals);
        mode!($globals) = (-(k as i16)).into();
        // if k=vmode then
        if k == vmode as halfword {
            // begin prev_depth:=ignore_depth;
            prev_depth!($globals) = ignore_depth;
            // if every_vbox<>null then begin_token_list(every_vbox,every_vbox_text);
            if every_vbox!($globals) != null {
                begin_token_list($globals, every_vbox!($globals), every_vbox_text);
            }
            // end
        }
        // else  begin space_factor:=1000;
        else {
            space_factor!($globals) = 1000;
            // if every_hbox<>null then begin_token_list(every_hbox,every_hbox_text);
            if every_hbox!($globals) != null {
                begin_token_list($globals, every_hbox!($globals), every_hbox_text);
            }
            // end;
        }
        // return;
        return_nojump!();
        // end
        use crate::section_0113::halfword;
        use crate::section_0115::null;
        use crate::section_0211::hmode;
        use crate::section_0211::vmode;
        use crate::section_0212::ignore_depth;
        use crate::section_0216::push_nest;
        use crate::section_0269::hbox_group;
        use crate::section_0269::vbox_group;
        use crate::section_0269::vtop_group;
        use crate::section_0307::every_hbox_text;
        use crate::section_0307::every_vbox_text;
        use crate::section_0323::begin_token_list;
        use crate::section_0645::scan_spec;
        use crate::section_1070::normal_paragraph;
    }}
}
