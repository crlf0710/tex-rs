//! @ The following macro is used to test if there is room for up to six more
//! entries on |save_stack|. By making a conservative test like this, we can
//! get by with testing for overflow in only a few places.
//
// @d check_full_save_stack==if save_ptr>max_save_stack then
pub(crate) macro check_full_save_stack($globals:expr) {{
    // begin max_save_stack:=save_ptr;
    $globals.max_save_stack = $globals.save_ptr;
    // if max_save_stack>save_size-6 then overflow("save size",save_size);
    if $globals.max_save_stack > save_size - 6 {
        todo!("not yet implemented in {}", file!());
    }
    // @:TeX capacity exceeded save size}{\quad save size@>
    // end
    use crate::section_0011::save_size;
}}
