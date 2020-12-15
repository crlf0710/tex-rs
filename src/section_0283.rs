//! @ A global definition, which sets the level to |level_one|,
//! @^global definitions@>
//! will not be undone by |unsave|. If at least one global definition of
//! |eqtb[p]| has been carried out within the group that just ended, the
//! last such definition will therefore survive.
//
// @<Store \(s)|save...@>=
macro_rules! Store_s_save_stack_save_ptr_in_eqtb_p__unless_eqtb_p_holds_a_global_value {
    ($globals:expr, $p:expr) => {{
        // if p<int_base then
        if $p < int_base as pointer {
            // if eq_level(p)=level_one then
            if eq_level!($globals, $p) == level_one {
                // begin eq_destroy(save_stack[save_ptr]); {destroy the saved value}
                // @!stat if tracing_restores>0 then restore_trace(p,"retaining");@+tats@;@/
                // end
                todo!("store 1a");
            }
            // else  begin eq_destroy(eqtb[p]); {destroy the current value}
            else {
                /// destroy the current value
                eq_destroy($globals, $globals.eqtb[$p]);
                // eqtb[p]:=save_stack[save_ptr]; {restore the saved value}
                /// restore the saved value
                const _ : () = ();
                $globals.eqtb[$p] = $globals.save_stack[$globals.save_ptr];
                // @!stat if tracing_restores>0 then restore_trace(p,"restoring");@+tats@;@/
                region_stat! {
                    if $globals.tracing_restores > 0 {
                        restore_trace($globals, p, strpool_str!("restoring"));
                    }
                }
                // end
            }
        }
        // else if xeq_level[p]<>level_one then
        else if $globals.xeq_level[$p] != level_one {
            todo!("store 2");
            // begin eqtb[p]:=save_stack[save_ptr]; xeq_level[p]:=l;
            // @!stat if tracing_restores>0 then restore_trace(p,"restoring");@+tats@;@/
            // end
        }
        // else  begin
        else {
            todo!("store 3");
            // @!stat if tracing_restores>0 then restore_trace(p,"retaining");@+tats@;@/
            // end
        }

        use crate::section_0230::int_base;
        use crate::section_0275::eq_destroy;
    }}
}