//! ` `

// @<Append the display and perhaps also the equation number@>=
pub(crate) macro Append_the_display_and_perhaps_also_the_equation_number($globals:expr, $e:expr, $s:expr, $d:expr, $a:expr, $b:expr, $z:expr, $w:expr, $l:expr) {{
    // if e<>0 then
    if $e != scaled::zero() {
        /// kern node used to position the display
        let r;
        // begin r:=new_kern(z-w-e-d);
        r = new_kern($globals, $z - $w - $e - $d)?;
        // if l then
        if $l {
            // begin link(a):=r; link(r):=b; b:=a; d:=0;
            link!($globals, $a) = r;
            link!($globals, r) = $b;
            $b = $a;
            $d = scaled::zero();
            // end
        }
        // else  begin link(b):=r; link(r):=a;
        else {
            link!($globals, $b) = r;
            link!($globals, r) = $a;
            // end;
        }
        // b:=hpack(b,natural);
        $b = hpack($globals, $b, natural0!(), natural1!())?;
        // end;
    }
    // shift_amount(b):=s+d; append_to_vlist(b)
    shift_amount!($globals, $b) = $s + $d;
    append_to_vlist($globals, $b)?;

    use crate::section_0101::scaled;
    use crate::section_0118::link;
    use crate::section_0135::shift_amount;
    use crate::section_0156::new_kern;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0649::hpack;
    use crate::section_0679::append_to_vlist;
}}
