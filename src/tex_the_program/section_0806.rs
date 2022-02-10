//! ` `
// @<Make the running dimensions in rule |q| extend...@>=
pub(crate) macro Make_the_running_dimensions_in_rule_q_extend_to_the_boundaries_of_the_alignment($globals:expr, $p:expr, $q:expr, $o:expr, $s:expr) {{
    // begin if is_running(width(q)) then width(q):=width(p);
    if is_running!(width!($globals, $q)) {
        width!($globals, $q) = width!($globals, $p);
    }
    // if is_running(height(q)) then height(q):=height(p);
    if is_running!(height!($globals, $q)) {
        height!($globals, $q) = height!($globals, $p);
    }
    // if is_running(depth(q)) then depth(q):=depth(p);
    if is_running!(depth!($globals, $q)) {
        depth!($globals, $q) = depth!($globals, $p);
    }
    // if o<>0 then
    if $o != scaled::zero() {
        /// registers for the list operations
        let r;
        // begin r:=link(q); link(q):=null; q:=hpack(q,natural);
        r = link!($globals, $q);
        link!($globals, $q) = null;
        $q = hpack($globals, $q, natural0!(), natural1!())?;
        // shift_amount(q):=o; link(q):=r; link(s):=q;
        shift_amount!($globals, $q) = $o;
        link!($globals, $q) = r;
        link!($globals, $s) = $q;
        // end;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0138::is_running;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0649::hpack;
}}
