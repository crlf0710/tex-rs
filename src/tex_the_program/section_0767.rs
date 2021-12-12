//! @ We insert a penalty node after the hlist entries of noad |q| if |pen|
//! is not an ``infinite'' penalty, and if the node immediately following |q|
//! is not a penalty node or a |rel_noad| or absent entirely.
//
// @<Append any |new_hlist| entries for |q|, and any appropriate penalties@>=
pub(crate) macro Append_any_new_hlist_entries_for_q__and_any_appropriate_penalties($globals:expr, $q:expr, $p:expr, $pen:expr, $penalties:expr) {{
    // if new_hlist(q)<>null then
    if new_hlist!($globals, $q) != null as _ {
        // begin link(p):=new_hlist(q);
        link!($globals, $p) = new_hlist!($globals, $q) as _;
        // repeat p:=link(p);
        loop {
            $p = link!($globals, $p);
            // until link(p)=null;
            if link!($globals, $p) == null {
                break;
            }
        }
        // end;
    }
    // if penalties then if link(q)<>null then if pen<inf_penalty then
    if $penalties && link!($globals, $q) != null && $pen < inf_penalty {
        // begin r_type:=type(link(q));
        // if r_type<>penalty_node then if r_type<>rel_noad then
        //   begin z:=new_penalty(pen); link(p):=z; p:=z;
        //   end;
        // end
        todo!("penalties && ...");
    }
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0157::inf_penalty;
    use crate::section_0725::new_hlist;
}}
