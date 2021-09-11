//! ` `

// @<Complain about unknown unit...@>=
pub(crate) macro Complain_about_unknown_unit_and_goto_done2($globals:expr, $lbl_done2:lifetime) {{
    // begin print_err("Illegal unit of measure ("); print("pt inserted)");
    print_err!($globals, crate::strpool_str!("Illegal unit of measure ("));
    print($globals, crate::strpool_str!("pt inserted)").get() as _);
    // @.Illegal unit of measure@>
    // help6("Dimensions can be in units of em, ex, in, pt, pc,")@/
    //   ("cm, mm, dd, cc, bp, or sp; but yours is a new one!")@/
    //   ("I'll assume that you meant to say pt, for printer's points.")@/
    //   ("To recover gracefully from this error, it's best to")@/
    //   ("delete the erroneous units; e.g., type `2' to delete")@/
    //   ("two letters. (See Chapter 27 of The TeXbook.)");
    help6!(
        $globals,
        crate::strpool_str!("Dimensions can be in units of em, ex, in, pt, pc,"),
        crate::strpool_str!("cm, mm, dd, cc, bp, or sp; but yours is a new one!"),
        crate::strpool_str!("I'll assume that you meant to say pt, for printer's points."),
        crate::strpool_str!("To recover gracefully from this error, it's best to"),
        crate::strpool_str!("delete the erroneous units; e.g., type `2' to delete"),
        crate::strpool_str!("two letters. (See Chapter 27 of The TeXbook.)")
    );
    // @:TeXbook}{\sl The \TeX book@>
    // error; goto done2;
    error($globals)?;
    crate::goto_forward_label!($lbl_done2);
    // end
    use crate::section_0059::print;
    use crate::section_0073::print_err;
    use crate::section_0079::help6;
    use crate::section_0082::error;
}}
