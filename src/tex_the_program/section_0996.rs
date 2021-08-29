//! ` `
// @<Update the values of |last_glue|...@>=
macro_rules! Update_the_values_of_last_glue_last_penalty_and_last_kern {
    ($globals:expr, $p:expr) => {{
        // if last_glue<>max_halfword then delete_glue_ref(last_glue);
        if $globals.last_glue != max_halfword {
            delete_glue_ref($globals, $globals.last_glue);
        }
        // last_penalty:=0; last_kern:=0;
        $globals.last_penalty = 0;
        $globals.last_kern = scaled::zero();
        // if type(p)=glue_node then
        if r#type!($globals, $p) == glue_node {
            // begin last_glue:=glue_ptr(p); add_glue_ref(last_glue);
            $globals.last_glue = glue_ptr!($globals, $p);
            add_glue_ref!($globals, $globals.last_glue);
            // end
        }
        // else  begin last_glue:=max_halfword;
        else {
            $globals.last_glue = max_halfword;
            // if type(p)=penalty_node then last_penalty:=penalty(p)
            if r#type!($globals, $p) == penalty_node {
                $globals.last_penalty = penalty!($globals, $p);
            }
            // else if type(p)=kern_node then last_kern:=width(p);
            else if r#type!($globals, $p) == kern_node {
                $globals.last_kern = width!($globals, $p);
            }
            // end
        }
        use crate::section_0101::scaled;
        use crate::section_0110::max_halfword;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
        use crate::section_0157::penalty_node;
        use crate::section_0201::delete_glue_ref;
    }}
}
