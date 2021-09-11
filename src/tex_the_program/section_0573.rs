//! ` `

// @d check_existence(#)==@t@>@;@/
pub(crate) macro check_existence($globals:expr, $f:expr, $chr:expr, $bc:expr, $ec:expr, $lbl_bad_tfm:lifetime) {{
    // begin check_byte_range(#);
    check_byte_range!($globals, $chr, $bc, $ec, $lbl_bad_tfm);
    // qw:=char_info(f)(#); {N.B.: not |qi(#)|}
    /// N.B.: not `qi(#)`
    const _: () = ();
    let i = char_info!($globals, $f, $chr);
    // if not char_exists(qw) then abort;
    if !i.char_exists() {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // end
    use crate::section_0554::char_info;
    use crate::section_0570::check_byte_range;
}}

// @<Read ligature/kern program@>=
pub(crate) macro Read_ligature_kern_program($globals:expr, $f:expr, $z:expr, $bch_label:expr, $bchar:expr,
        $alpha:expr, $beta:expr, $bc:expr, $ec:expr, $nl:expr, $nk:expr, $lbl_bad_tfm:lifetime) {{
    // bch_label:=@'77777; bchar:=256;
    $bch_label = 0o77777;
    $bchar = non_char;
    // if nl>0 then
    if $nl > 0 {
        let mut qqqq = four_quarters::default();
        // begin for k:=lig_kern_base[f] to kern_base[f]+kern_base_offset-1 do
        for k in $globals.lig_kern_base[$f]..=$globals.kern_base[$f] + kern_base_offset - 1 {
            let k = k as pointer;
            // begin store_four_quarters(font_info[k].qqqq);
            store_four_quarters!($globals, qqqq);
            $globals.font_info[k][MEMORY_WORD_QQQQ] = qqqq;
            // if a>128 then
            if qqqq[FOUR_QUARTERS_B0] > 128 {
                // begin if 256*c+d>=nl then abort;
                if 256 * qqqq[FOUR_QUARTERS_B2] as integer + qqqq[FOUR_QUARTERS_B3] as integer
                    >= $nl as integer
                {
                    crate::goto_forward_label!($lbl_bad_tfm);
                }
                // if a=255 then if k=lig_kern_base[f] then bchar:=b;
                if qqqq[FOUR_QUARTERS_B0] == 255 {
                    if k as integer == $globals.lig_kern_base[$f] {
                        $bchar = qqqq[FOUR_QUARTERS_B1] as ASCII_code_or_non_char;
                    }
                }
            // end
            }
            // else begin if b<>bchar then check_existence(b);
            else {
                if qqqq[FOUR_QUARTERS_B1] as ASCII_code_or_non_char != $bchar {
                    check_existence!($globals, $f, qqqq[FOUR_QUARTERS_B1], $bc, $ec, $lbl_bad_tfm);
                }
                // if c<128 then check_existence(d) {check ligature}
                if qqqq[FOUR_QUARTERS_B2] < 128 {
                    /// check ligature
                    const _: () = ();
                    check_existence!($globals, $f, qqqq[FOUR_QUARTERS_B3], $bc, $ec, $lbl_bad_tfm);
                }
                // else if 256*(c-128)+d>=nk then abort; {check kern}
                else if 256 * (qqqq[FOUR_QUARTERS_B2] - 128) as integer
                    + qqqq[FOUR_QUARTERS_B3] as integer
                    >= $nk as integer
                {
                    /// check kern
                    const _: () = ();
                    crate::goto_forward_label!($lbl_bad_tfm);
                }
                // if a<128 then if k-lig_kern_base[f]+a+1>=nl then abort;
                if qqqq[FOUR_QUARTERS_B0] < 128 {
                    if k as integer - $globals.lig_kern_base[$f]
                        + qqqq[FOUR_QUARTERS_B0] as integer
                        + 1
                        >= $nl as integer
                    {
                        crate::goto_forward_label!($lbl_bad_tfm);
                    }
                }
                // end;
            }
            // end;
        }
        // if a=255 then bch_label:=256*c+d;
        if qqqq[FOUR_QUARTERS_B0] == 255 {
            $bch_label =
                256 * qqqq[FOUR_QUARTERS_B2] as integer + qqqq[FOUR_QUARTERS_B3] as integer;
        }
        // end;
    }
    // for k:=kern_base[f]+kern_base_offset to exten_base[f]-1 do
    for k in $globals.kern_base[$f] + kern_base_offset..=$globals.exten_base[$f] - 1 {
        let k = k as pointer;
        // store_scaled(font_info[k].sc);
        store_scaled!(
            $globals,
            $globals.font_info[k][MEMORY_WORD_SC],
            $z.inner(),
            $alpha,
            $beta,
            $lbl_bad_tfm
        );
    }

    use crate::pascal::integer;
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0113::four_quarters;
    use crate::section_0113::FOUR_QUARTERS_B0;
    use crate::section_0113::FOUR_QUARTERS_B1;
    use crate::section_0113::FOUR_QUARTERS_B2;
    use crate::section_0113::FOUR_QUARTERS_B3;
    use crate::section_0113::MEMORY_WORD_QQQQ;
    use crate::section_0115::pointer;
    use crate::section_0549::non_char;
    use crate::section_0557::kern_base_offset;
    use crate::section_0564::store_four_quarters;
    use crate::section_0571::store_scaled;
    use crate::section_0907::ASCII_code_or_non_char;
}}
