//! ` `
//! Now the preamble list has been converted to a list of alternating unset
//! boxes and tabskip glue, where the box widths are equal to the final
//! column sizes. In case of \.{\\valign}, we change the widths to heights,
//! so that a correct error message will be produced if the alignment is
//! overfull or underfull.
//
// @<Package the preamble list...@>=
pub(crate) macro Package_the_preamble_list__to_determine_the_actual_tabskip_glue_amounts__and_let_p_point_to_this_prototype_box($globals:expr, $p:expr) {{
    // save_ptr:=save_ptr-2; pack_begin_line:=-mode_line;
    $globals.save_ptr -= 2;
    $globals.pack_begin_line = -mode_line!($globals);
    // if mode=-vmode then
    if mode!($globals) == -vmode {
        // begin rule_save:=overfull_rule;
        // overfull_rule:=0; {prevent rule from being packaged}
        // p:=hpack(preamble,saved(1),saved(0)); overfull_rule:=rule_save;
        // end
        todo!("mode = -vmode");
    }
    // else  begin q:=link(preamble);
    else {
        /// registers for the list operations
        let (mut q,);

        q = link!($globals, preamble!($globals));
        // repeat height(q):=width(q); width(q):=0; q:=link(link(q));
        loop {
            height!($globals, q) = width!($globals, q);
            width!($globals, q) = scaled::zero();
            q = link!($globals, link!($globals, q));
            // until q=null;
            if q == null {
                break;
            }
        }
        // p:=vpack(preamble,saved(1),saved(0));
        $p = vpack(
            $globals,
            preamble!($globals),
            scaled::new_from_inner(saved!($globals, 1)),
            small_number::new(saved!($globals, 0) as _),
        )?;
        // q:=link(preamble);
        q = link!($globals, preamble!($globals));
        // repeat width(q):=height(q); height(q):=0; q:=link(link(q));
        loop {
            width!($globals, q) = height!($globals, q);
            height!($globals, q) = scaled::zero();
            q = link!($globals, link!($globals, q));
            // until q=null;
            if q == null {
                break;
            }
            // end;
        }
    }
    // pack_begin_line:=0
    $globals.pack_begin_line = 0;

    use crate::section_0101::scaled;
    use crate::section_0101::small_number;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
    use crate::section_0213::mode_line;
    use crate::section_0274::saved;
    use crate::section_0668::vpack;
    use crate::section_0770::preamble;
}}
