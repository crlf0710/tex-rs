//! @ The word to be hyphenated is now moved to the |hu| and |hc| arrays.
//
// @<Skip to node |hb|, putting letters...@>=
macro_rules! Skip_to_node_hb__putting_letters_into_hu_and_hc {
    ($globals:expr, $s:expr) => {{
        // hn:=0;
        $globals.hn = 0.into();
        region_forward_label!(
        |'done3|
        {
        // loop@+  begin if is_char_node(s) then
        loop {
            if is_char_node!($globals, $s) {
                let c;
                // begin if font(s)<>hf then goto done3;
                if font!($globals, $s) != $globals.hf {
                    goto_forward_label!('done3);
                }
                // hyf_bchar:=character(s); c:=qo(hyf_bchar);
                c = character!($globals, $s);
                $globals.hyf_bchar = c.numeric_value() as _;
                // if lc_code(c)=0 then goto done3;
                if lc_code!($globals, c) == 0 {
                    goto_forward_label!('done3);
                }
                // if hn=63 then goto done3;
                if $globals.hn == 63 {
                    goto_forward_label!('done3);
                }
                // hb:=s; incr(hn); hu[hn]:=c; hc[hn]:=lc_code(c); hyf_bchar:=non_char;
                $globals.hb = $s;
                incr!($globals.hn);
                $globals.hu[$globals.hn.get() as usize] = c;
                $globals.hc[$globals.hn.get() as usize] = ASCII_code::from(lc_code!($globals, c) as integer);
                $globals.hyf_bchar = non_char;
                // end
            }
            // else if type(s)=ligature_node then
            else if r#type!($globals, $s) != ligature_node {
                // @<Move the characters of a ligature node to |hu| and |hc|;
                //   but |goto done3| if they are not all letters@>
                todo!("move the characters");
            }
            // else if (type(s)=kern_node)and(subtype(s)=normal) then
            else if r#type!($globals, $s) == kern_node &&
                subtype!($globals, $s) as integer == kern_node_subtype::normal as integer {
                // begin hb:=s;
                $globals.hb = $s;
                // hyf_bchar:=font_bchar[hf];
                $globals.hyf_bchar = $globals.font_bchar[$globals.hf];
                // end
            }
            // else goto done3;
            else {
                goto_forward_label!('done3);
            }
            // s:=link(s);
            $s = link!($globals, $s);
            // end;
        }
        }
        // done3:
        'done3 <-
        );
        use crate::section_0018::ASCII_code;
        use crate::section_0549::non_char;
    }}
}
