//! @ The user can force the equation number to go on a separate line
//! by causing its width to be zero.
//
// @<Squeeze the equation as much as possible...@>=
pub(crate) macro Squeeze_the_equation_as_much_as_possible__if_there_is_an_equation_number_that_should_go_on_a_separate_line_by_itself__set_e_to_0($globals:expr, $w:expr, $q:expr, $z:expr, $p:expr, $b:expr, $e:expr) {{
    // begin if (e<>0)and((w-total_shrink[normal]+q<=z)or@|
    //    (total_shrink[fil]<>0)or(total_shrink[fill]<>0)or
    //    (total_shrink[filll]<>0)) then
    if $e != scaled::zero() && (($w - $globals.total_shrink[glue_ord::normal] + $q <= $z) ||
        ($globals.total_shrink[glue_ord::fil] != scaled::zero()) ||
        ($globals.total_shrink[glue_ord::fill] != scaled::zero()) ||
        ($globals.total_shrink[glue_ord::filll] != scaled::zero())) {
        // begin free_node(b,box_node_size);
        // b:=hpack(p,z-q,exactly);
        // end
        todo!("e != 0");
    }
    // else  begin e:=0;
    else {
        $e = scaled::zero();
        // if w>z then
        if $w > $z {
            // begin free_node(b,box_node_size);
            free_node($globals, $b, box_node_size as _);
            // b:=hpack(p,z,exactly);
            $b = hpack($globals, $p, $z, exactly.into())?;
            // end;
        }
        // end;
    }
    // w:=width(b);
    $w = width!($globals, $b);
    // end
    use crate::section_0101::scaled;
    use crate::section_0130::free_node;
    use crate::section_0135::width;
    use crate::section_0135::box_node_size;
    use crate::section_0150::glue_ord;
    use crate::section_0644::exactly;
    use crate::section_0649::hpack;
}}
