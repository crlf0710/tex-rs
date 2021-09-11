//! @ The preliminary settings of the index-offset variables |char_base|,
//! |width_base|, |lig_kern_base|, |kern_base|, and |exten_base| will be
//! corrected later by subtracting |min_quarterword| from them; and we will
//! subtract 1 from |param_base| too. It's best to forget about such anomalies
//! until later.
//
// @<Use size fields to allocate font information@>=
pub(crate) macro Use_size_fields_to_allocate_font_information($globals:expr, $f:expr, $lf:expr, $lh:expr, $bc:expr, $ec:expr, $nw:expr, $nh:expr,
        $nd:expr, $ni:expr, $nl:expr, $nk:expr, $ne:expr, $np:expr) {{
    // lf:=lf-6-lh; {|lf| words should be loaded into |font_info|}
    /// `lf` words should be loaded into `font_info`
    const _: () = ();
    $lf = $lf - 6 - $lh;
    // if np<7 then lf:=lf+7-np; {at least seven parameters will appear}
    /// at least seven parameters will appear
    if $np < 7 {
        $lf = $lf + 7 - $np;
    }
    // if (font_ptr=font_max)or(fmem_ptr+lf>font_mem_size) then
    if $globals.font_ptr.get() == font_max as u16 || $globals.fmem_ptr + $lf > font_mem_size {
        //   @<Apologize for not loading the font, |goto done|@>;
        todo!("apologize");
    }
    // f:=font_ptr+1;
    $f = $globals.font_ptr + 1;
    // char_base[f]:=fmem_ptr-bc;
    $globals.char_base[$f] = $globals.fmem_ptr.get() as integer - $bc as integer;
    // width_base[f]:=char_base[f]+ec+1;
    $globals.width_base[$f] = $globals.char_base[$f] + $ec as integer + 1;
    // height_base[f]:=width_base[f]+nw;
    $globals.height_base[$f] = $globals.width_base[$f] + $nw as integer;
    // depth_base[f]:=height_base[f]+nh;
    $globals.depth_base[$f] = $globals.height_base[$f] + $nh as integer;
    // italic_base[f]:=depth_base[f]+nd;
    $globals.italic_base[$f] = $globals.depth_base[$f] + $nd as integer;
    // lig_kern_base[f]:=italic_base[f]+ni;
    $globals.lig_kern_base[$f] = $globals.italic_base[$f] + $ni as integer;
    // kern_base[f]:=lig_kern_base[f]+nl-kern_base_offset;
    $globals.kern_base[$f] = $globals.lig_kern_base[$f] + $nl as integer - kern_base_offset;
    // exten_base[f]:=kern_base[f]+kern_base_offset+nk;
    $globals.exten_base[$f] = $globals.kern_base[$f] + kern_base_offset + $nk as integer;
    // param_base[f]:=exten_base[f]+ne
    $globals.param_base[$f] = $globals.exten_base[$f] + $ne as integer;

    use crate::pascal::integer;
    use crate::section_0011::font_max;
    use crate::section_0011::font_mem_size;
    use crate::section_0557::kern_base_offset;
}}
