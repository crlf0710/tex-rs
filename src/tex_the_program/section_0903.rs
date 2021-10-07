//! ` `
// If hyphens are in fact going to be inserted, \TeX\ first deletes the
// subsequence of nodes between |ha| and~|hb|. An attempt is made to
// preserve the effect that implicit boundary characters and punctuation marks
// had on ligatures inside the hyphenated word, by storing a left boundary or
// preceding character in |hu[0]| and by storing a possible right boundary
// in |bchar|. We set |j:=0| if |hu[0]| is to be part of the reconstruction;
// otherwise |j:=1|.
// The variable |s| will point to the tail of the current hlist, and
// |q| will point to the node following |hb|, so that
// things can be hooked up after we reconstitute the hyphenated word.
//
// @<Replace nodes |ha..hb| by a sequence of nodes...@>=
pub(crate) macro Replace_nodes_ha_to_hb_by_a_sequence_of_nodes_that_includes_the_discretionary_hyphens($globals:expr) {{
    /// temporary registers for list manipulation
    let (q, r, mut s);
    /// boundary character of hyphenated word, or `non_char`
    let bchar: ASCII_code_or_non_char;
    /// indices into `hc` or `hu`
    let mut j: u8_from_0_to_n<U65>;
    // q:=link(hb); link(hb):=null; r:=link(ha); link(ha):=null; bchar:=hyf_bchar;
    q = link!($globals, $globals.hb);
    link!($globals, $globals.hb) = null;
    r = link!($globals, $globals.ha);
    link!($globals, $globals.ha) = null;
    bchar = $globals.hyf_bchar;
    crate::region_forward_label! {
    |'common_ending|
    {
        crate::region_forward_label! {
        |'found2|
        {
            // if is_char_node(ha) then
            if is_char_node!($globals, $globals.ha) {
                // if font(ha)<>hf then goto found2
                // else begin init_list:=ha; init_lig:=false; hu[0]:=qo(character(ha));
                //   end
                todo!("char node");
            }
            // else if type(ha)=ligature_node then
            else if r#type!($globals, $globals.ha) == ligature_node {
                // if font(lig_char(ha))<>hf then goto found2
                // else begin init_list:=lig_ptr(ha); init_lig:=true; init_lft:=(subtype(ha)>1);
                //   hu[0]:=qo(character(lig_char(ha)));
                //   if init_list=null then if init_lft then
                //     begin hu[0]:=256; init_lig:=false;
                //     end; {in this case a ligature will be reconstructed from scratch}
                //   free_node(ha,small_node_size);
                //   end
                todo!("ligature node");
            }
            // else begin {no punctuation found; look for left boundary}
            else {
                /// no punctuation found; look for left boundary
                const _: () = ();
                // if not is_char_node(r) then if type(r)=ligature_node then
                //  if subtype(r)>1 then goto found2;
                if !is_char_node!($globals, r) && r#type!($globals, r) == ligature_node && subtype!($globals, r) > 1 {
                    crate::goto_forward_label!('found2);
                }
                // j:=1; s:=ha; init_list:=null; goto common_ending;
                j = 1.into();
                s = $globals.ha;
                $globals.init_list = null;
                crate::goto_forward_label!('common_ending);
                // end;
            }
            // s:=cur_p; {we have |cur_p<>ha| because |type(cur_p)=glue_node|}
            /// we have `cur_p<>ha` because `type(cur_p)=glue_node`
            const _: () = ();
            s = $globals.cur_p;
            // while link(s)<>ha do s:=link(s);
            while link!($globals, s) != $globals.ha {
                s = link!($globals, s);
            }
            // j:=0; goto common_ending;
            j = 0.into();
            crate::goto_forward_label!('common_ending);
        }
        // found2: s:=ha; j:=0; hu[0]:=256; init_lig:=false; init_list:=null;
        'found2 <-
        };
        s = $globals.ha;
        j = 0.into();
        $globals.hu[0] = non_char;
        $globals.init_lig = false;
        $globals.init_list = null;
    }
    // common_ending: flush_node_list(r);
    'common_ending <-
    };
    flush_node_list($globals, r)?;
    // @<Reconstitute nodes for the hyphenated word, inserting discretionary hyphens@>;
    crate::section_0913::Reconstitute_nodes_for_the_hyphenated_word__inserting_discretionary_hyphens!($globals, j, bchar, q, r, s);
    // flush_list(init_list)
    flush_list($globals, $globals.init_list);
    use typenum::U65;
    use crate::pascal::u8_from_0_to_n;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0123::flush_list;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0134::is_char_node;
    use crate::section_0143::ligature_node;
    use crate::section_0202::flush_node_list;
    use crate::section_0549::non_char;
    use crate::section_0907::ASCII_code_or_non_char;
}}
