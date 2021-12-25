//! ` `

// @<Append the display and perhaps also the equation number@>=
pub(crate) macro Append_the_display_and_perhaps_also_the_equation_number($globals:expr, $e:expr, $s:expr, $d:expr, $b:expr) {{
    // if e<>0 then
    if $e != scaled::zero() {
        // begin r:=new_kern(z-w-e-d);
        // if l then
        //   begin link(a):=r; link(r):=b; b:=a; d:=0;
        //   end
        // else  begin link(b):=r; link(r):=a;
        //   end;
        // b:=hpack(b,natural);
        // end;
        todo!("e!=0");
    }
    // shift_amount(b):=s+d; append_to_vlist(b)
    shift_amount!($globals, $b) = $s + $d;
    append_to_vlist($globals, $b)?;

    use crate::section_0101::scaled;
    use crate::section_0135::shift_amount;
    use crate::section_0679::append_to_vlist;
}}
