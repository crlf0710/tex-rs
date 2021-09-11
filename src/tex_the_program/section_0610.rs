//! ` `
// @<Generate a |down| or |right|...@>=
pub(crate) macro Generate_a_down_or_right_command_for_w_and_return
    ($globals:expr, $q:expr, $w:expr, $o:expr) {{
        // info(q):=yz_OK;
        info_inner!($globals, $q) = yz_OK as _;
        crate::region_forward_label!(
        |'label1|
        {
        crate::region_forward_label!(
        |'label2|
        {
        // if abs(w)>=@'40000000 then
        if $w.inner().abs() >= 0o40000000 {
            // begin dvi_out(o+3); {|down4| or |right4|}
            /// `down4` or `right4`
            dvi_out!($globals, $o.byte() + 3);
            // dvi_four(w); return;
            dvi_four($globals, $w.inner());
            crate::return_nojump!();
            // end;
        }
        // if abs(w)>=@'100000 then
        if $w.inner().abs() >= 0o100000 {
            // begin dvi_out(o+2); {|down3| or |right3|}
            /// `down3` or `right3`
            dvi_out!($globals, $o.byte() + 2);
            // if w<0 then w:=w+@'100000000;
            if $w < scaled::zero() {
                $w += scaled::new_from_inner(0o100000000);
            }
            // dvi_out(w div @'200000); w:=w mod @'200000; goto 2;
            dvi_out!($globals, $w.inner() / 0o200000);
            $w %= scaled::new_from_inner(0o200000);
            crate::goto_forward_label!('label2);
            // end;
        }
        // if abs(w)>=@'200 then
        if $w.inner().abs() >= 0o200 {
            // begin dvi_out(o+1); {|down2| or |right2|}
            /// `down2` or `right2`
            dvi_out!($globals, $o.byte() + 1);
            // if w<0 then w:=w+@'200000;
            if $w < scaled::zero() {
                $w += scaled::new_from_inner(0o200000);
            }
            // goto 2;
            crate::goto_forward_label!('label2);
            // end;
        }
        // dvi_out(o); {|down1| or |right1|}
        /// `down1` or `right1`
        dvi_out!($globals, $o.byte());
        // if w<0 then w:=w+@'400;
        if $w < scaled::zero() {
            $w += scaled::new_from_inner(0o400);
        }
        // goto 1;
        crate::goto_forward_label!('label1);
        }
        // 2: dvi_out(w div @'400);
        'label2 <-
        );
        dvi_out!($globals, $w.inner() / 0o400);
        }
        // 1: dvi_out(w mod @'400); return
        'label1 <-
        );
        dvi_out!($globals, $w.inner() % 0o400);
        crate::return_nojump!();
        use crate::section_0101::scaled;
        use crate::section_0600::dvi_four;
        use  crate::section_0598::dvi_out;
        use crate::section_0118::info_inner;
        use crate::section_0608::yz_OK;
    }}
