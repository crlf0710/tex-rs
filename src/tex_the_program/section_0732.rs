//! @ Conditional math glue (`\.{\\nonscript}') results in a |glue_node|
//! pointing to |zero_glue|, with |subtype(q)=cond_math_glue|; in such a case
//! the node following will be eliminated if it is a glue or kern node and if the
//! current size is different from |text_size|. Unconditional math glue
//! (`\.{\\muskip}') is converted to normal glue by multiplying the dimensions
//! by |cur_mu|.
//! @!@:non_script_}{\.{\\nonscript} primitive@>
//
// @<Convert \(m)math glue to ordinary glue@>=
pub(crate) macro Convert_math_glue_to_ordinary_glue($globals:expr, $q:expr) {{
    // if subtype(q)=mu_glue then
    if subtype!($globals, $q) == glue_node_subtype::mu_glue as _ {
        // begin x:=glue_ptr(q);
        // y:=math_glue(x,cur_mu); delete_glue_ref(x); glue_ptr(q):=y;
        // subtype(q):=normal;
        // end
        todo!("mu_glue");
    }
    // else if (cur_size<>text_size)and(subtype(q)=cond_math_glue) then
    else if $globals.cur_size != text_size
        && subtype!($globals, $q) == glue_node_subtype::cond_math_glue as _
    {
        // begin p:=link(q);
        // if p<>null then if (type(p)=glue_node)or(type(p)=kern_node) then
        //   begin link(q):=link(p); link(p):=null; flush_node_list(p);
        //   end;
        // end
        todo!("cond_math_glue");
    }
    use crate::section_0133::subtype;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0699::text_size;
}}
