//! @ We want to make sure that there is no cycle of characters linked together
//! by |list_tag| entries, since such a cycle would get \TeX\ into an endless
//! loop. If such a cycle exists, the routine here detects it when processing
//! the largest character code in the cycle.
//
// @d check_byte_range(#)==begin if (#<bc)or(#>ec) then abort@+end
macro_rules! check_byte_range {
    ($globals:expr, $chr:expr, $bc:expr, $ec:expr, $lbl_bad_tfm:lifetime) => {{
        if ($chr as integer) < $bc as integer || ($chr as integer) > $ec as integer {
            goto_forward_label!($lbl_bad_tfm);
        }
    }}
}
// @d current_character_being_worked_on==k+bc-fmem_ptr
//
// @<Check for charlist cycle@>=
// begin check_byte_range(d);
// while d<current_character_being_worked_on do
//   begin qw:=char_info(f)(d);
//   {N.B.: not |qi(d)|, since |char_base[f]| hasn't been adjusted yet}
//   if char_tag(qw)<>list_tag then goto not_found;
//   d:=qo(rem_byte(qw)); {next character on the list}
//   end;
// if d=current_character_being_worked_on then abort; {yes, there's a cycle}
// not_found:end
//
