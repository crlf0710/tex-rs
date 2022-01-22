//! @ When the following code is executed, |char_tag(q)| will be equal to
//! |ext_tag| if and only if a built-up symbol is supposed to be returned.
//
// @<Make variable |b| point to a box for |(f,c)|@>=
pub(crate) macro Make_variable_b_point_to_a_box_for_f_and_c($globals:expr, $b:expr, $q:expr, $f:expr, $c:expr) {{
    // if char_tag(q)=ext_tag then
    if $q.char_tag() == char_tag::ext_tag {
        // @<Construct an extensible character in a new box |b|,
        //   using recipe |rem_byte(q)| and font |f|@>
        todo!("Construct");
    }
    // else b:=char_box(f,c)
    else {
        $b = char_box($globals, $f, $c)?;
    }
    use crate::section_0544::char_tag;
    use crate::section_0709::char_box;
}}
