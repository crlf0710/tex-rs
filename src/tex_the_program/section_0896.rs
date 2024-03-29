//! @ The first thing we need to do is find the node |ha| just before the
//! first letter.
//
// @<Skip to node |ha|, or |goto done1|...@>=
pub(crate) macro Skip_to_node_ha__or_goto_done1_if_no_hyphenation_should_be_attempted {
    ($globals:expr, $s:expr, $prev_s:expr, $lbl_done1:lifetime) => {{
        crate::region_forward_label!(
        |'done2|
        {
        // loop@+  begin if is_char_node(s) then
        loop {
            let c;
            crate::region_forward_label!(
            |'continue_|
            {
            if is_char_node!($globals, $s) {
                // begin c:=qo(character(s)); hf:=font(s);
                c = character!($globals, $s);
                $globals.hf = font!($globals, $s);
                // end
            }
            // else if type(s)=ligature_node then
            else if r#type!($globals, $s) == ligature_node {
                // if lig_ptr(s)=null then goto continue
                if lig_ptr!($globals, $s) == null {
                    crate::goto_forward_label!('continue_);
                }
                // else begin q:=lig_ptr(s); c:=qo(character(q)); hf:=font(q);
                else {
                    /// miscellaneous nodes of temporary interest
                    let q: pointer;
                    q = lig_ptr!($globals, $s);
                    c = character!($globals, q);
                    $globals.hf = font!($globals, q);
                }
                //   end
            }
            // else if (type(s)=kern_node)and(subtype(s)=normal) then goto continue
            else if r#type!($globals, $s) == kern_node && subtype!($globals, $s) as integer == kern_node_subtype::normal as integer {
                crate::goto_forward_label!('continue_);
            }
            // else if type(s)=whatsit_node then
            else if r#type!($globals, $s) == whatsit_node {
                // begin @<Advance \(p)past a whatsit node in the \(p)pre-hyphenation loop@>;
                crate::section_1363::Advance_past_a_whatsit_node_in_the_pre_hyphenation_loop!($globals, $s);
                // goto continue;
                crate::goto_forward_label!('continue_);
                // end
            }
            // else goto done1;
            else {
                crate::goto_forward_label!($lbl_done1);
            }
            // if lc_code(c)<>0 then
            if lc_code!($globals, c) != 0 {
                // if (lc_code(c)=c)or(uc_hyph>0) then goto done2
                if lc_code!($globals, c) as integer == c.numeric_value() as integer ||
                    uc_hyph!($globals) > 0 {
                    crate::goto_forward_label!('done2);
                }
                // else goto done1;
                else {
                    crate::goto_forward_label!($lbl_done1);
                }
            }
            }
            // continue: prev_s:=s; s:=link(prev_s);
            'continue_ <-
            );
            $prev_s = $s;
            $s = link!($globals, $prev_s);
            // end;
        }
        }
        // done2: hyf_char:=hyphen_char[hf];
        'done2 <-
        );
        $globals.hyf_char = $globals.hyphen_char[$globals.hf];
        // if hyf_char<0 then goto done1;
        if $globals.hyf_char < 0 {
            crate::goto_forward_label!($lbl_done1);
        }
        // if hyf_char>255 then goto done1;
        if $globals.hyf_char > 255 {
            crate::goto_forward_label!($lbl_done1);
        }
        // ha:=prev_s
        $globals.ha = $prev_s;
        use crate::pascal::integer;
        use crate::section_0115::pointer;
        use crate::section_0115::null;
        use crate::section_0118::link;
        use crate::section_0133::r#type;
        use crate::section_0133::subtype;
        use crate::section_0134::font;
        use crate::section_0134::character;
        use crate::section_0134::is_char_node;
        use crate::section_0143::ligature_node;
        use crate::section_0143::lig_ptr;
        use crate::section_0146::whatsit_node;
        use crate::section_0155::kern_node;
        use crate::section_0155::kern_node_subtype;
        use crate::section_0230::lc_code;
        use crate::section_0236::uc_hyph;
    }}
}
