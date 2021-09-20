//! @ The word to be hyphenated is now moved to the |hu| and |hc| arrays.
//
// @<Skip to node |hb|, putting letters...@>=
pub(crate) macro Skip_to_node_hb__putting_letters_into_hu_and_hc {
    ($globals:expr, $s:expr) => {{
        crate::trace_span!("Skip to node |hb|, putting letters...");
        // hn:=0;
        $globals.hn = 0.into();
        crate::region_forward_label!(
        |'done3|
        {
        // loop@+  begin if is_char_node(s) then
        loop {
            if is_char_node!($globals, $s) {
                crate::trace_debug_expr_verbose!(
                    "s = {}, s.lh = {}",
                    $s,
                    $globals.mem[$s][crate::section_0113::MEMORY_WORD_HH_LH]
                );
                let c;
                // begin if font(s)<>hf then goto done3;
                if font!($globals, $s) != $globals.hf {
                    crate::goto_forward_label!('done3);
                }
                // hyf_bchar:=character(s); c:=qo(hyf_bchar);
                c = character!($globals, $s);
                $globals.hyf_bchar = c.numeric_value() as _;
                // if lc_code(c)=0 then goto done3;
                if lc_code!($globals, c) == 0 {
                    crate::goto_forward_label!('done3);
                }
                // if hn=63 then goto done3;
                if $globals.hn == 63 {
                    crate::goto_forward_label!('done3);
                }
                // hb:=s; incr(hn); hu[hn]:=c; hc[hn]:=lc_code(c); hyf_bchar:=non_char;
                $globals.hb = $s;
                incr!($globals.hn);
                $globals.hu[$globals.hn.get() as usize] = c.numeric_value() as _;
                $globals.hc[$globals.hn.get() as usize] = lc_code!($globals, c) as _;
                $globals.hyf_bchar = non_char;
                // end
            }
            // else if type(s)=ligature_node then
            else if r#type!($globals, $s) == ligature_node {
                // @<Move the characters of a ligature node to |hu| and |hc|;
                //   but |goto done3| if they are not all letters@>
                crate::section_0898::Move_the_characters_of_a_ligature_node_to_hu_and_hc__but_goto_done3_if_they_are_not_all_letters!($globals, $s, 'done3);
            }
            // else if (type(s)=kern_node)and(subtype(s)=normal) then
            else if r#type!($globals, $s) == kern_node &&
                subtype!($globals, $s) as integer == kern_node_subtype::normal as integer {
                // begin hb:=s;
                $globals.hb = $s;
                // hyf_bchar:=font_bchar[hf];
                $globals.hyf_bchar = $globals.font_bchar[$globals.hf];
                // end
            }
            // else goto done3;
            else {
                crate::goto_forward_label!('done3);
            }
            // s:=link(s);
            $s = link!($globals, $s);
            // end;
        }
        }
        // done3:
        'done3 <-
        );
        use crate::pascal::integer;
        use crate::section_0016::incr;
        use crate::section_0018::ASCII_code;
        use crate::section_0118::link;
        use crate::section_0133::r#type;
        use crate::section_0133::subtype;
        use crate::section_0134::is_char_node;
        use crate::section_0134::character;
        use crate::section_0134::font;
        use crate::section_0143::ligature_node;
        use crate::section_0155::kern_node;
        use crate::section_0155::kern_node_subtype;
        use crate::section_0230::lc_code;
        use crate::section_0549::non_char;
    }}
}
