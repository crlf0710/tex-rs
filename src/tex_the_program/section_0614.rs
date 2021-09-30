//! ` `

// @<Change buffered instruction to |z| or |x| and |goto found|@>=
pub(crate) macro Change_buffered_instruction_to_z_or_x_and_goto_found($globals:expr, $p:expr, $lbl_found:lifetime) {{
    /// index into `dvi_buf`, modulo `dvi_buf_size`
    let mut k: integer;
    // begin k:=location(p)-dvi_offset;
    k = location!($globals, $p) - $globals.dvi_offset;
    // if k<0 then k:=k+dvi_buf_size;
    if k < 0 {
        k = k + dvi_buf_size as integer;
    }
    // dvi_buf[k]:=dvi_buf[k]+z1-down1;
    $globals.dvi_buf[dvi_index::new(k as _)] = ($globals.dvi_buf[dvi_index::new(k as _)] as integer
        + z1.byte() as integer
        - down1.byte() as integer) as _;
    // info(p):=z_here; goto found;
    info_inner!($globals, $p) = z_here as _;
    crate::goto_forward_label!($lbl_found);
    // end
    use crate::pascal::integer;
    use crate::section_0011::dvi_buf_size;
    use crate::section_0118::info_inner;
    use crate::section_0586::down1;
    use crate::section_0586::z1;
    use crate::section_0594::dvi_index;
    use crate::section_0605::location;
    use crate::section_0608::z_here;
}}
