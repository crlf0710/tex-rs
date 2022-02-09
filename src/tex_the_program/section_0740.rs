//! ` `

// @<Switch to a larger accent if available and appropriate@>=
pub(crate) macro Switch_to_a_larger_accent_if_available_and_appropriate($globals:expr, $i:expr, $c:expr, $f:expr, $w:expr) {{
    crate::region_forward_label! {
        |'done|
        {
            // loop@+  begin if char_tag(i)<>list_tag then goto done;
            loop {
                let y;

                if $i.char_tag() != char_tag::list_tag {
                    crate::goto_forward_label!('done);
                }
                // y:=rem_byte(i);
                y = ASCII_code::from($i.rem_byte() as integer);
                // i:=char_info(f)(y);
                $i = char_info!($globals, $f, y.numeric_value());
                // if not char_exists(i) then goto done;
                if !$i.char_exists() {
                    crate::goto_forward_label!('done);
                }
                // if char_width(f)(i)>w then goto done;
                if char_width!($globals, $f, $i) > $w {
                    crate::goto_forward_label!('done);
                }
                // c:=y;
                $c = y;
                // end;
            }
        }
        // done:
        'done <-
    }
    use crate::pascal::integer;
    use crate::section_0018::ASCII_code;
    use crate::section_0544::char_tag;
    use crate::section_0554::char_info;
    use crate::section_0554::char_width;
}}
