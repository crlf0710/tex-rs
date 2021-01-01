//! @ Only the first two words of the header are needed by \TeX82.
//
// @<Read the {\.{TFM}} header@>=
macro_rules! Read_the_TFM_header {
    ($globals:expr, $f:expr, $s:expr, $z:expr, $lh:expr, $lbl_bad_tfm:lifetime) => {{
        // begin if lh<2 then abort;
        if $lh < 2 {
            goto_forward_label!($lbl_bad_tfm);
        }
        // store_four_quarters(font_check[f]);
        store_four_quarters!($globals, $globals.font_check[$f]);
        // fget; read_sixteen(z); {this rejects a negative design size}
        /// this rejects a negative design size
        const _ : () = ();
        fget!($globals);
        let mut z0: u16;
        read_sixteen!($globals, z0, $lbl_bad_tfm);
        let mut z0: integer = z0 as integer;
        // fget; z:=z*@'400+fbyte; fget; z:=(z*@'20)+(fbyte div@'20);
        fget!($globals);
        z0 = z0 * 0o400 + fbyte!($globals) as integer;
        fget!($globals);
        z0 = z0 * 0o20 + (fbyte!($globals) / 0o20) as integer;
        $z = scaled::new_from_inner(z0);
        // if z<unity then abort;
        if $z < unity {
            goto_forward_label!($lbl_bad_tfm);
        }
        // while lh>2 do
        while $lh > 2 {
            // begin fget;fget;fget;fget;decr(lh); {ignore the rest of the header}
            /// ignore the rest of the header
            const _ : () = ();
            fget!($globals);
            fget!($globals);
            fget!($globals);
            fget!($globals);
            decr!($lh);
            // end;
        }
        // font_dsize[f]:=z;
        $globals.font_dsize[$f] = $z;
        // if s<>-1000 then
        if $s.inner() != -1000 {
            // if s>=0 then z:=s
            if $s.inner() >= 0 {
                $z = $s;
            }
            // else z:=xn_over_d(z,-s,1000);
            else {
                $z = xn_over_d($globals, $z, -$s.inner(), 1000);
            }
        }
        // font_size[f]:=z;
        $globals.font_size[$f] = $z;
        // end

        use crate::pascal::integer;
        use crate::section_0101::unity;
        use crate::section_0107::xn_over_d;
    }}
}
