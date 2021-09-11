//! ` `

// @<Read character data@>=
pub(crate) macro Read_character_data($globals:expr, $f:expr, $bc:expr, $ec:expr, $nw:expr, $nh:expr, $nd:expr, $ni:expr, $nl:expr, $ne:expr, $lbl_bad_tfm:lifetime) {{
    // for k:=fmem_ptr to width_base[f]-1 do
    for k in $globals.fmem_ptr.get() as integer..=$globals.width_base[$f] - 1 {
        let k: pointer = k as pointer;

        // begin store_four_quarters(font_info[k].qqqq);
        let qqqq: four_quarters;
        store_four_quarters!($globals, qqqq);
        $globals.font_info[k][MEMORY_WORD_QQQQ] = qqqq;
        // if (a>=nw)or(b div @'20>=nh)or(b mod @'20>=nd)or
        //   (c div 4>=ni) then abort;
        if qqqq[FOUR_QUARTERS_B0] as u16 >= $nw
            || qqqq[FOUR_QUARTERS_B1] as u16 / 0o20 >= $nh
            || qqqq[FOUR_QUARTERS_B1] as u16 % 0o20 >= $nd
            || qqqq[FOUR_QUARTERS_B2] as u16 / 4 >= $ni
        {
            crate::goto_forward_label!($lbl_bad_tfm);
        }
        // case c mod 4 of
        let c_mod_4: char_tag = char_tag::from(qqqq[FOUR_QUARTERS_B2] % 4);
        // lig_tag: if d>=nl then abort;
        if c_mod_4 == char_tag::lig_tag {
            if qqqq[FOUR_QUARTERS_B3] as u16 >= $nl {
                crate::goto_forward_label!($lbl_bad_tfm);
            }
        }
        // ext_tag: if d>=ne then abort;
        else if c_mod_4 == char_tag::ext_tag {
            if qqqq[FOUR_QUARTERS_B3] as u16 >= $ne {
                crate::goto_forward_label!($lbl_bad_tfm);
            }
        }
        // list_tag: @<Check for charlist cycle@>;
        else if c_mod_4 == char_tag::list_tag {
            crate::section_0570::Check_for_charlist_cycle!(
                $globals,
                $f,
                k,
                qqqq[FOUR_QUARTERS_B3],
                $bc,
                $ec,
                $lbl_bad_tfm
            );
        }
        // othercases do_nothing {|no_tag|}
        else {
            /// `no_tag`
            const _: () = ();
            do_nothing!();
        }
        // endcases;
        // end
    }
    use crate::pascal::integer;
    use crate::section_0016::do_nothing;
    use crate::section_0113::four_quarters;
    use crate::section_0113::FOUR_QUARTERS_B0;
    use crate::section_0113::FOUR_QUARTERS_B1;
    use crate::section_0113::FOUR_QUARTERS_B2;
    use crate::section_0113::FOUR_QUARTERS_B3;
    use crate::section_0113::MEMORY_WORD_QQQQ;
    use crate::section_0115::pointer;
    use crate::section_0544::char_tag;
    use crate::section_0564::store_four_quarters;
}}
