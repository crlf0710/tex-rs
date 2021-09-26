//! ` `
//！ The synchronization algorithm begins with |l=i+1<=j|.
//
// @<Put the \(c)characters |hu[i+1..@,]| into |post_break(r)|...@>=
pub(crate) macro Put_the_characters_hu_i_plus_1_to_end_into_post_break_r__appending_to_this_list_and_to_major_tail_until_synchronization_has_been_achieved($globals:expr, $minor_tail:expr, $l:expr, $i:expr, $r:expr, $j:expr) {{
    /// where that character came from
    let c_loc;
    // minor_tail:=null; post_break(r):=null; c_loc:=0;
    $minor_tail = null;
    post_break!($globals, $r) = null;
    c_loc = 0;
    // if bchar_label[hf]<>non_address then {put left boundary at beginning of new line}
    if $globals.bchar_label[$globals.hf] != non_address {
        /// put left boundary at beginning of new line
        const _: () = ();
        // begin decr(l); c:=hu[l]; c_loc:=l; hu[l]:=256;
        // end;
        todo!("bchar_label[hf]<>non_address");
    }
    // while l<j do
    while $l < $j {
        // begin repeat l:=reconstitute(l,hn,bchar,non_char)+1;
        // if c_loc>0 then
        //   begin hu[c_loc]:=c; c_loc:=0;
        //   end;
        // if link(hold_head)>null then
        //   begin if minor_tail=null then post_break(r):=link(hold_head)
        //   else link(minor_tail):=link(hold_head);
        //   minor_tail:=link(hold_head);
        //   while link(minor_tail)>null do minor_tail:=link(minor_tail);
        //   end;
        // until l>=j;
        // while l>j do
        //   @<Append characters of |hu[j..@,]| to |major_tail|, advancing~|j|@>;
        // end
        todo!("l < j");
    }
    use crate::section_0115::null;
    use crate::section_0145::post_break;
    use crate::section_0549::non_address;
}}
