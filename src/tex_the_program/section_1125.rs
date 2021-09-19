//! ` `
// The kern nodes appended here must be distinguished from other kerns, lest
// they be wiped away by the hyphenation algorithm or by a previous line break.
//
// The two kerns are computed with (machine-dependent) |real| arithmetic, but
// their sum is machine-independent; the net effect is machine-independent,
// because the user cannot remove these nodes nor access them via \.{\\lastkern}.
//
// @<Append the accent with appropriate kerns...@>=
pub(crate) macro Append_the_accent_with_appropriate_kerns__then_set_p_to_q($globals:expr, $p:expr, $q:expr, $r:expr, $s:expr, $t:expr, $f:expr, $a:expr, $h:expr, $x:expr, $w:expr) {
    /// character information
    let i;
    let delta: scaled;
    // begin t:=slant(f)/float_constant(65536);
    $t = slant!($globals, $f).inner_real() / float_constant!(65536);
    // @^real division@>
    // i:=char_info(f)(character(q));
    let character_q = character!($globals, $q);
    i = char_info!($globals, $f, character_q.numeric_value());
    // w:=char_width(f)(i); h:=char_height(f)(height_depth(i));
    $w = char_width!($globals, $f, i);
    $h = char_height!($globals, $f, i.height_depth());
    // if h<>x then {the accent must be shifted up or down}
    if $h != $x {
        /// the accent must be shifted up or down
        const _: () = ();
        // begin p:=hpack(p,natural); shift_amount(p):=x-h;
        $p = hpack($globals, $p, natural0!(), natural1!())?;
        shift_amount!($globals, $p) = $x - $h;
        // end;
    }
    // delta:=round((w-a)/float_constant(2)+h*t-x*s);
    delta = scaled::new_from_inner(
        (($w - $a).inner_real() / float_constant!(2) + $h.inner_real() * $t - $x.inner_real() * $s)
            .round() as integer,
    );
    // @^real multiplication@>
    // @^real addition@>
    // r:=new_kern(delta); subtype(r):=acc_kern; link(tail):=r; link(r):=p;
    $r = new_kern($globals, delta)?;
    subtype!($globals, $r) = kern_node_subtype::acc_kern as _;
    link!($globals, tail!($globals)) = $r;
    link!($globals, $r) = $p;
    // tail:=new_kern(-a-delta); subtype(tail):=acc_kern; link(p):=tail; p:=q;
    tail!($globals) = new_kern($globals, -$a - delta)?;
    subtype!($globals, tail!($globals)) = kern_node_subtype::acc_kern as _;
    link!($globals, $p) = tail!($globals);
    $p = $q;
    // end
    use crate::pascal::integer;
    use crate::section_0101::scaled;
    use crate::section_0109::float_constant;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0134::character;
    use crate::section_0135::shift_amount;
    use crate::section_0155::kern_node_subtype;
    use crate::section_0156::new_kern;
    use crate::section_0213::tail;
    use crate::section_0554::char_height;
    use crate::section_0554::char_info;
    use crate::section_0554::char_width;
    use crate::section_0558::slant;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0649::hpack;
}
