//! @ The following code is executed when |scan_something_internal| was
//! called asking for |mu_val|, when we really wanted a ``mudimen'' instead
//! of ``muglue.''
//
// @<Coerce glue to a dimension@>=
pub(crate) macro Coerce_glue_to_a_dimension($globals:expr) {{
    // if cur_val_level>=glue_val then
    if $globals.cur_val_level >= cur_val_level_kind::glue_val {
        /// an internal dimension
        let v;
        // begin v:=width(cur_val); delete_glue_ref(cur_val); cur_val:=v;
        v = width!($globals, $globals.cur_val as pointer);
        delete_glue_ref($globals, $globals.cur_val as _);
        $globals.cur_val = v.inner() as _;
        // end
    }
    use crate::section_0115::pointer;
    use crate::section_0135::width;
    use crate::section_0201::delete_glue_ref;
    use crate::section_0410::cur_val_level_kind;
}}
