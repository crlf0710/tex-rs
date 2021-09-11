//! @ Now to wrap it up, we have checked all the necessary things about the \.{TFM}
//! file, and all we need to do is put the finishing touches on the data for
//! the new font.
//
// @d adjust(#)==#[f]:=qo(#[f])
//   {correct for the excess |min_quarterword| that was added}
/// correct for the excess `min_quarterword` that was added
pub(crate) macro adjust($selector:expr, $f:expr) {
    $selector[$f] = $selector[$f] - crate::section_0110::min_quarterword as crate::pascal::integer
}

// @<Make final adjustments...@>=
pub(crate) macro Make_final_adjustments_and_goto_done($globals:expr, $f:expr, $g:expr, $bch_label:expr, $bchar:expr, $lf:expr, $bc:expr, $ec:expr, $nl:expr, $np:expr,
        $nom:expr, $aire:expr, $lbl_done:lifetime) {{
    // if np>=7 then font_params[f]:=np@+else font_params[f]:=7;
    if $np >= 7 {
        $globals.font_params[$f] = $np.into();
    } else {
        $globals.font_params[$f] = 7.into();
    }
    // hyphen_char[f]:=default_hyphen_char; skew_char[f]:=default_skew_char;
    $globals.hyphen_char[$f] = default_hyphen_char!($globals);
    $globals.skew_char[$f] = default_skew_char!($globals);
    // if bch_label<nl then bchar_label[f]:=bch_label+lig_kern_base[f]
    if $bch_label < $nl as integer {
        $globals.bchar_label[$f] = font_index::new(($bch_label + $globals.lig_kern_base[$f]) as _);
    }
    // else bchar_label[f]:=non_address;
    else {
        $globals.bchar_label[$f] = non_address;
    }
    /// NOTE: `qi` is no longered used here because of the type chagne.
    const _: () = ();
    // font_bchar[f]:=qi(bchar);
    $globals.font_bchar[$f] = $bchar;
    // font_false_bchar[f]:=qi(bchar);
    $globals.font_false_bchar[$f] = $bchar;
    // if bchar<=ec then if bchar>=bc then
    if $bchar < $ec as ASCII_code_or_non_char && $bchar >= $bc as ASCII_code_or_non_char {
        // begin qw:=char_info(f)(bchar); {N.B.: not |qi(bchar)|}
        /// N.B.: not `qi(bchar)`
        const _: () = ();
        let i = char_info!($globals, $f, $bchar);
        // if char_exists(qw) then font_false_bchar[f]:=non_char;
        if i.char_exists() {
            $globals.font_false_bchar[$f] = non_char;
        }
        // end;
    }
    // font_name[f]:=nom;
    $globals.font_name[$f] = $nom;
    // font_area[f]:=aire;
    $globals.font_area[$f] = $aire;
    // font_bc[f]:=bc; font_ec[f]:=ec; font_glue[f]:=null;
    $globals.font_bc[$f] = ASCII_code::from($bc as integer);
    $globals.font_ec[$f] = ASCII_code::from($ec as integer);
    $globals.font_glue[$f] = null;
    // adjust(char_base); adjust(width_base); adjust(lig_kern_base);
    adjust!($globals.char_base, $f);
    adjust!($globals.width_base, $f);
    adjust!($globals.lig_kern_base, $f);
    // adjust(kern_base); adjust(exten_base);
    adjust!($globals.kern_base, $f);
    adjust!($globals.exten_base, $f);
    // decr(param_base[f]);
    decr!($globals.param_base[$f]);
    // fmem_ptr:=fmem_ptr+lf; font_ptr:=f; g:=f; goto done
    $globals.fmem_ptr = $globals.fmem_ptr + $lf;
    $globals.font_ptr = $f;
    $g = $f;
    crate::goto_forward_label!($lbl_done);

    use crate::pascal::integer;
    use crate::section_0016::decr;
    use crate::section_0018::ASCII_code;
    use crate::section_0115::null;
    use crate::section_0236::default_hyphen_char;
    use crate::section_0236::default_skew_char;
    use crate::section_0548::font_index;
    use crate::section_0549::non_address;
    use crate::section_0549::non_char;
    use crate::section_0554::char_info;
    use crate::section_0907::ASCII_code_or_non_char;
}}
