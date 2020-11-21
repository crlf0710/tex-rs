//! @ Here we are at |main_loop_move_lig|.
//! When we begin this code we have |cur_q=tail| and |cur_l=character(lig_stack)|.
//!
//! @<Move the cursor past a pseudo-ligature...@>=
//! main_p:=lig_ptr(lig_stack);
//! if main_p>null then tail_append(main_p); {append a single character}
//! temp_ptr:=lig_stack; lig_stack:=link(temp_ptr);
//! free_node(temp_ptr,small_node_size);
//! main_i:=char_info(main_f)(cur_l); ligature_present:=true;
//! if lig_stack=null then
//!   if main_p>null then goto main_loop_lookahead
//!   else cur_r:=bchar
//! else cur_r:=character(lig_stack);
//! goto main_lig_loop
//!
