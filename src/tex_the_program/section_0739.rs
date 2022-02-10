//! ` `
// @<Make the height of box |y|...@>=
pub(crate) macro Make_the_height_of_box_y_equal_to_h($globals:expr, $y:expr, $h:expr, $p:expr) {{
    // begin p:=new_kern(h-height(y)); link(p):=list_ptr(y); list_ptr(y):=p;
    $p = new_kern($globals, $h - height!($globals, $y))?;
    link!($globals, $p) = list_ptr!($globals, $y);
    list_ptr!($globals, $y) = $p;
    // height(y):=h;
    height!($globals, $y) = $h;
    // end
    use crate::section_0118::link;
    use crate::section_0135::height;
    use crate::section_0135::list_ptr;
    use crate::section_0156::new_kern;
}}
