//! @ The width of an extensible character is the width of the repeatable
//! module. If this module does not have positive height plus depth,
//! we don't use any copies of it, otherwise we use as few as possible
//! (in groups of two if there is a middle part).
//
// @<Compute the minimum suitable height, |w|, and...@>=
pub(crate) macro Compute_the_minimum_suitable_height__w__and_the_corresponding_number_of_extension_steps__n__also_set_width_b($globals:expr, $b:expr, $q:expr, $v:expr, $f:expr, $c:expr, $r:expr, $w:expr, $n:expr) {{
    /// height-plus-depth of a tentative character
    let u;
    // c:=ext_rep(r); u:=height_plus_depth(f,c);
    $c = ASCII_code::from($r.ext_rep() as integer);
    u = height_plus_depth($globals, $f, $c);
    // w:=0; q:=char_info(f)(c); width(b):=char_width(f)(q)+char_italic(f)(q);@/
    $w = scaled::zero();
    $q = char_info!($globals, $f, $c.numeric_value());
    width!($globals, $b) = char_width!($globals, $f, $q) + char_italic!($globals, $f, $q);
    // c:=ext_bot(r);@+if c<>min_quarterword then w:=w+height_plus_depth(f,c);
    $c = ASCII_code::from($r.ext_bot() as integer);
    if $c.numeric_value() != min_quarterword as _ {
        $w += height_plus_depth($globals, $f, $c);
    }
    // c:=ext_mid(r);@+if c<>min_quarterword then w:=w+height_plus_depth(f,c);
    $c = ASCII_code::from($r.ext_mid() as integer);
    if $c.numeric_value() != min_quarterword as _ {
        $w += height_plus_depth($globals, $f, $c);
    }
    // c:=ext_top(r);@+if c<>min_quarterword then w:=w+height_plus_depth(f,c);
    $c = ASCII_code::from($r.ext_top() as integer);
    if $c.numeric_value() != min_quarterword as _ {
        $w += height_plus_depth($globals, $f, $c);
    }
    // n:=0;
    $n = 0;
    // if u>0 then while w<v do
    if u > scaled::zero() {
        while $w < $v {
            // begin w:=w+u; incr(n);
            $w += u;
            incr!($n);
            // if ext_mid(r)<>min_quarterword then w:=w+u;
            if $r.ext_mid() != min_quarterword {
                $w += u;
            }
            // end
        }
    }
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0101::scaled;
    use crate::section_0110::min_quarterword;
    use crate::section_0135::width;
    use crate::section_0554::char_info;
    use crate::section_0554::char_italic;
    use crate::section_0554::char_width;
    use crate::section_0712::height_plus_depth;
}}
