//! ` `
// @<Compleat...@>=
pub(crate) macro Compleat_the_incompleat_noad($globals:expr, $p:expr, $q:expr) {{
    // begin math_type(denominator(incompleat_noad)):=sub_mlist;
    math_type!(
        $globals,
        denominator!(incompleat_noad!($globals) as pointer)
    ) = math_type_kind::sub_mlist as _;
    // info(denominator(incompleat_noad)):=link(head);
    info_inner!(
        $globals,
        denominator!(incompleat_noad!($globals) as pointer)
    ) = link!($globals, head!($globals));
    // if p=null then q:=incompleat_noad
    if $p == null {
        $q = incompleat_noad!($globals) as _;
    }
    // else  begin q:=info(numerator(incompleat_noad));
    else {
        $q = info_inner!($globals, numerator!(incompleat_noad!($globals) as pointer));
        // if type(q)<>left_noad then confusion("right");
        if r#type!($globals, $q) != left_noad {
            todo!("confusion");
            // @:this can't happen right}{\quad right@>
        }
        // info(numerator(incompleat_noad)):=link(q);
        info_inner!($globals, numerator!(incompleat_noad!($globals) as pointer)) =
            link!($globals, $q);
        // link(q):=incompleat_noad; link(incompleat_noad):=p;
        link!($globals, $q) = incompleat_noad!($globals) as _;

        // end;
    }
    // end

    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0213::head;
    use crate::section_0213::incompleat_noad;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0683::denominator;
    use crate::section_0683::numerator;
    use crate::section_0687::left_noad;
}}
