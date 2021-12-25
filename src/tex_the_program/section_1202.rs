//! @ We try first to center the display without regard to the existence of
//! the equation number. If that would make it too close (where ``too close''
//! means that the space between display and equation number is less than the
//! width of the equation number), we either center it in the remaining space
//! or move it as far from the equation number as possible. The latter alternative
//! is taken only if the display begins with glue, since we assume that the
//! user put glue there to control the spacing precisely.
//
// @<Determine the displacement, |d|, of the left edge of the equation...@>=
pub(crate) macro Determine_the_displacement__d__of_the_left_edge_of_the_equation__with_respect_to_the_line_size_z__assuming_that_l_is_false($globals:expr, $d:expr, $z:expr, $w:expr, $e:expr) {{
    // d:=half(z-w);
    $d = scaled::new_from_inner(half(($z - $w).inner()));
    // if (e>0)and(d<2*e) then {too close}
    if $e > scaled::zero() && $d < scaled::new_from_inner(2 * $e.inner()) {
        /// too close
        const _ : () = ();
        // begin d:=half(z-w-e);
        // if p<>null then if not is_char_node(p) then if type(p)=glue_node then d:=0;
        // end
        todo!("too close");
    }
    use crate::section_0100::half;
    use crate::section_0101::scaled;
}}
