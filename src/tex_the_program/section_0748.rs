//! ` `
// @<Put the \(f)fraction into a box with its delimiters...@>=
pub(crate) macro Put_the_fraction_into_a_box_with_its_delimiters__and_make_new_hlist_q_point_to_it($globals:expr, $q:expr, $v:expr) {{
    /// temporary registers for box construction
    let (x, z);
    /// dimensions for box calculations
    let delta;
    // if cur_style<text_style then delta:=delim1(cur_size)
    if $globals.cur_style.get() < style_node_subtype::text_style.get() {
        delta = delim1!($globals, $globals.cur_size.get());
    }
    // else delta:=delim2(cur_size);
    else {
        delta = delim2!($globals, $globals.cur_size.get());
    }
    // x:=var_delimiter(left_delimiter(q), cur_size, delta); link(x):=v;@/
    x = var_delimiter($globals, left_delimiter!($q), $globals.cur_size, delta)?;
    link!($globals, x) = $v;
    // z:=var_delimiter(right_delimiter(q), cur_size, delta); link(v):=z;@/
    z = var_delimiter($globals, right_delimiter!($q), $globals.cur_size, delta)?;
    link!($globals, $v) = z;
    // new_hlist(q):=hpack(x,natural)
    new_hlist!($globals, $q) = hpack($globals, x, natural0!(), natural1!())? as _;
    use crate::section_0118::link;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0649::hpack;
    use crate::section_0683::left_delimiter;
    use crate::section_0683::right_delimiter;
    use crate::section_0688::style_node_subtype;
    use crate::section_0700::delim1;
    use crate::section_0700::delim2;
    use crate::section_0706::var_delimiter;
    use crate::section_0725::new_hlist;
}}
