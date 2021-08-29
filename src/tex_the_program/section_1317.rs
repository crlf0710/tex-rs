//! ` `
// @<Undump regions 1 to 6 of |eqtb|@>=
macro_rules! Undump_regions_1_to_6_of_eqtb {
    ($globals:expr, $lbl_bad_fmt:lifetime) => {{
        /// all-purpose indices
        let mut k: integer;
        // k:=active_base;
        k = active_base as _;
        // repeat undump_int(x);
        loop {
            /// something undumped
            let mut x: integer;
            undump_int!($globals, x);
            // if (x<1)or(k+x>eqtb_size+1) then goto bad_fmt;
            if x < 1 || k + x > eqtb_size as integer + 1 {
                goto_forward_label!($lbl_bad_fmt);
            }
            // for j:=k to k+x-1 do undump_wd(eqtb[j]);
            for j in k ..= k + x - 1 {
                undump_wd!($globals, $globals.eqtb[j as pointer]);
            }
            // k:=k+x;
            k += x;
            // undump_int(x);
            undump_int!($globals, x);
            // if (x<0)or(k+x>eqtb_size+1) then goto bad_fmt;
            if x < 0 || k + x > eqtb_size as integer + 1 {
                goto_forward_label!($lbl_bad_fmt);
            }
            // for j:=k to k+x-1 do eqtb[j]:=eqtb[k-1];
            for j in k ..= k + x - 1 {
                $globals.eqtb[j as pointer] = $globals.eqtb[k as pointer - 1];
            }
            // k:=k+x;
            k += x;
            // until k>eqtb_size
            if k > eqtb_size as integer {
                break;
            }
        }
        use crate::pascal::integer;
        use crate::section_0115::pointer;
        use crate::section_0222::active_base;
        use crate::section_0247::eqtb_size;
    }}
}