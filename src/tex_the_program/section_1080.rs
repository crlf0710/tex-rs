//! @ Note that the condition |not is_char_node(tail)| implies that |head<>tail|,
//! since |head| is a one-word node.
//
// @<If the current list ends with a box node, delete it...@>=
pub(crate) macro If_the_current_list_ends_with_a_box_node__delete_it_from_the_list_and_make_cur_box_point_to_it__otherwise_set_cur_box_to_null($globals:expr) {{
    // begin cur_box:=null;
    $globals.cur_box = null;
    // if abs(mode)=mmode then
    if mode!($globals).get().abs() == mmode {
        // begin you_cant; help1("Sorry; this \lastbox will be void."); error;
        // end
        todo!("you_cant");
    }
    // else if (mode=vmode)and(head=tail) then
    else if mode!($globals) == vmode && head!($globals) == tail!($globals) {
        // begin you_cant;
        // help2("Sorry...I usually can't take things from the current page.")@/
        //   ("This \lastbox will therefore be void."); error;
        // end
        todo!("you_cant");
    }
    // else  begin if not is_char_node(tail) then
    else {
        if !is_char_node!($globals, tail!($globals)) {
            // if (type(tail)=hlist_node)or(type(tail)=vlist_node) then
            if r#type!($globals, tail!($globals)) == hlist_node
                || r#type!($globals, tail!($globals)) == vlist_node
            {
                // @<Remove the last box, unless it's part of a discretionary@>;
                crate::section_1081::Remove_the_last_box__unless_its_part_of_a_discretionary!(
                    $globals
                );
            }
        }
        // end;
    }
    // end
    use crate::section_0115::null;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0135::hlist_node;
    use crate::section_0137::vlist_node;
    use crate::section_0211::mmode;
    use crate::section_0211::vmode;
    use crate::section_0213::head;
    use crate::section_0213::mode;
    use crate::section_0213::tail;
}}
