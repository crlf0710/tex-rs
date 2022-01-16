//! ` `
//! We now have a completed alignment, in the list that starts at |head|
//! and ends at |tail|. This list will be merged with the one that encloses
//! it. (In case the enclosing mode is |mmode|, for displayed formulas,
//! we will need to insert glue before and after the display; that part of the
//! program will be deferred until we're more familiar with such operations.)
//!
//! In restricted horizontal mode, the |clang| part of |aux| is undefined;
//! an over-cautious \PASCAL\ runtime system may complain about this.
//! @^dirty \PASCAL@>
//
// @<Insert the \(c)current list into its environment@>=
pub(crate) macro Insert_the_current_list_into_its_environment($globals:expr) {{
    /// registers for the list operations
    let (p, q);
    /// temporary storage for `aux`
    let aux_save: memory_word;

    // aux_save:=aux; p:=link(head); q:=tail; pop_nest;
    aux_save = aux!($globals);
    p = link!($globals, head!($globals));
    q = tail!($globals);
    pop_nest($globals);
    // if mode=mmode then @<Finish an alignment in a display@>
    if mode!($globals) == mmode {
        crate::section_1206::Finish_an_alignment_in_a_display!($globals, p, q, aux_save);
    }
    // else  begin aux:=aux_save; link(tail):=p;
    else {
        aux!($globals) = aux_save;
        link!($globals, tail!($globals)) = p;
        // if p<>null then tail:=q;
        if p != null {
            tail!($globals) = q;
        }
        // if mode=vmode then build_page;
        if mode!($globals) == vmode {
            build_page($globals)?;
        }
        // end
    }
    use crate::section_0113::memory_word;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0211::mmode;
    use crate::section_0211::vmode;
    use crate::section_0213::aux;
    use crate::section_0213::head;
    use crate::section_0213::mode;
    use crate::section_0213::tail;
    use crate::section_0217::pop_nest;
    use crate::section_0994::build_page;
}}
