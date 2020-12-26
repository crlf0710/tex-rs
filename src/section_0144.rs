//! @ The |new_ligature| function creates a ligature node having given
//! contents of the |font|, |character|, and |lig_ptr| fields. We also have
//! a |new_lig_item| function, which returns a two-word node having a given
//! |character| field. Such nodes are used for temporary processing as ligatures
//! are being created.
//!
//! @p function new_ligature(@!f,@!c:quarterword; @!q:pointer):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); type(p):=ligature_node;
//! font(lig_char(p)):=f; character(lig_char(p)):=c; lig_ptr(p):=q;
//! subtype(p):=0; new_ligature:=p;
//! end;
//! @#
//! function new_lig_item(@!c:quarterword):pointer;
//! var p:pointer; {the new node}
//! begin p:=get_node(small_node_size); character(p):=c; lig_ptr(p):=null;
//! new_lig_item:=p;
//! end;
//!
