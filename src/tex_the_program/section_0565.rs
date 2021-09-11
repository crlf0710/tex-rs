//! ` `
// @<Read the {\.{TFM}} size fields@>=
pub(crate) macro Read_the_TFM_size_fields($globals:expr, $lf:expr, $lh:expr, $bc:expr, $ec:expr, $nw:expr, $nh:expr,
        $nd:expr, $ni:expr, $nl:expr, $nk:expr, $ne:expr, $np:expr, $lbl_bad_tfm:lifetime) {{
    // begin read_sixteen(lf);
    read_sixteen!($globals, $lf, $lbl_bad_tfm);
    // fget; read_sixteen(lh);
    fget!($globals);
    read_sixteen!($globals, $lh, $lbl_bad_tfm);
    // fget; read_sixteen(bc);
    fget!($globals);
    read_sixteen!($globals, $bc, $lbl_bad_tfm);
    // fget; read_sixteen(ec);
    fget!($globals);
    read_sixteen!($globals, $ec, $lbl_bad_tfm);
    // if (bc>ec+1)or(ec>255) then abort;
    if $bc > $ec + 1 || $ec > 255 {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // if bc>255 then {|bc=256| and |ec=255|}
    if $bc > 255 {
        /// `bc = 256` and `ec = 255`
        const _: () = ();
        // begin bc:=1; ec:=0;
        $bc = 1;
        $ec = 0;
        // end;
    }
    // fget; read_sixteen(nw);
    fget!($globals);
    read_sixteen!($globals, $nw, $lbl_bad_tfm);
    // fget; read_sixteen(nh);
    fget!($globals);
    read_sixteen!($globals, $nh, $lbl_bad_tfm);
    // fget; read_sixteen(nd);
    fget!($globals);
    read_sixteen!($globals, $nd, $lbl_bad_tfm);
    // fget; read_sixteen(ni);
    fget!($globals);
    read_sixteen!($globals, $ni, $lbl_bad_tfm);
    // fget; read_sixteen(nl);
    fget!($globals);
    read_sixteen!($globals, $nl, $lbl_bad_tfm);
    // fget; read_sixteen(nk);
    fget!($globals);
    read_sixteen!($globals, $nk, $lbl_bad_tfm);
    // fget; read_sixteen(ne);
    fget!($globals);
    read_sixteen!($globals, $ne, $lbl_bad_tfm);
    // fget; read_sixteen(np);
    fget!($globals);
    read_sixteen!($globals, $np, $lbl_bad_tfm);
    // if lf<>6+lh+(ec-bc+1)+nw+nh+nd+ni+nl+nk+ne+np then abort;
    if $lf != 6 + $lh + ($ec - $bc + 1) + $nw + $nh + $nd + $ni + $nl + $nk + $ne + $np {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // if (nw=0)or(nh=0)or(nd=0)or(ni=0) then abort;
    if $nw == 0 || $nh == 0 || $nd == 0 || $ni == 0 {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // end
    use crate::section_0564::fget;
    use crate::section_0564::read_sixteen;
}}
