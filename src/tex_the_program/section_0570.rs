//! @ We want to make sure that there is no cycle of characters linked together
//! by |list_tag| entries, since such a cycle would get \TeX\ into an endless
//! loop. If such a cycle exists, the routine here detects it when processing
//! the largest character code in the cycle.
//
// @d check_byte_range(#)==begin if (#<bc)or(#>ec) then abort@+end
pub(crate) macro check_byte_range($globals:expr, $chr:expr, $bc:expr, $ec:expr, $lbl_bad_tfm:lifetime) {{
    if ($chr as integer) < $bc as integer || ($chr as integer) > $ec as integer {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    use crate::pascal::integer;
}}
// @d current_character_being_worked_on==k+bc-fmem_ptr
pub(crate) macro current_character_being_worked_on($globals:expr, $k:expr, $bc:expr) {
    ($k + $bc - $globals.fmem_ptr.get()) as crate::section_0113::quarterword
}

// @<Check for charlist cycle@>=
pub(crate) macro Check_for_charlist_cycle {
    ($globals:expr, $f:expr, $k:expr, $val:expr, $bc:expr, $ec:expr, $lbl_bad_tfm:lifetime) => {{
        let mut d = $val;
        // begin check_byte_range(d);
        check_byte_range!($globals, d, $bc, $ec, $lbl_bad_tfm);
        crate::region_forward_label!(
            |'not_found|
            {
                // while d<current_character_being_worked_on do
                while d < current_character_being_worked_on!($globals, $k, $bc) {
                    // begin qw:=char_info(f)(d);
                    // {N.B.: not |qi(d)|, since |char_base[f]| hasn't been adjusted yet}
                    /// N.B.: not `qi(d)`, since `char_base[f]` hasn't been adjusted yet
                    let i = char_info!($globals, $f, d);
                    // if char_tag(qw)<>list_tag then goto not_found;
                    if i.char_tag() != char_tag::list_tag {
                        crate::goto_forward_label!('not_found);
                    }
                    // d:=qo(rem_byte(qw)); {next character on the list}
                    d = qo!(i.rem_byte());
                    // end;
                }
                // if d=current_character_being_worked_on then abort; {yes, there's a cycle}
                if d == current_character_being_worked_on!($globals, $k, $bc) {
                    /// yes, there's a cycle
                    crate::goto_forward_label!($lbl_bad_tfm);
                }
            }
            // not_found:end
            'not_found <-
        );
        use crate::section_0112::qo;
        use crate::section_0544::char_tag;
        use crate::section_0554::char_info;
    }}
}
