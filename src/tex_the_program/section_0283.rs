//! @ A global definition, which sets the level to |level_one|,
//! @^global definitions@>
//! will not be undone by |unsave|. If at least one global definition of
//! |eqtb[p]| has been carried out within the group that just ended, the
//! last such definition will therefore survive.
//
// @<Store \(s)|save...@>=
pub(crate) macro Store_s_save_stack_save_ptr_in_eqtb_p__unless_eqtb_p_holds_a_global_value($globals:expr, $p:expr, $l:expr) {{
    // if p<int_base then
    if $p < int_base as pointer {
        // if eq_level(p)=level_one then
        if eq_level!($globals, $p) == level_one {
            // begin eq_destroy(save_stack[save_ptr]); {destroy the saved value}
            /// destroy the saved value
            eq_destroy($globals, $globals.save_stack[$globals.save_ptr])?;
            // @!stat if tracing_restores>0 then restore_trace(p,"retaining");@+tats@;@/
            crate::region_stat! {
                if tracing_restores!($globals) > 0 {
                    restore_trace($globals, $p, crate::strpool_str!("retaining"));
                }
                use crate::section_0236::tracing_restores;
                use crate::section_0284::restore_trace;
            }
            // end
        }
        // else  begin eq_destroy(eqtb[p]); {destroy the current value}
        else {
            /// destroy the current value
            eq_destroy($globals, $globals.eqtb[$p])?;
            // eqtb[p]:=save_stack[save_ptr]; {restore the saved value}
            /// restore the saved value
            const _: () = ();
            $globals.eqtb[$p] = $globals.save_stack[$globals.save_ptr];
            // @!stat if tracing_restores>0 then restore_trace(p,"restoring");@+tats@;@/
            crate::region_stat! {
                if tracing_restores!($globals) > 0 {
                    restore_trace($globals, $p, crate::strpool_str!("restoring"));
                }
                use crate::section_0236::tracing_restores;
                use crate::section_0284::restore_trace;
            }
            // end
        }
    }
    // else if xeq_level[p]<>level_one then
    else if $globals.xeq_level[$p] != level_one {
        // begin eqtb[p]:=save_stack[save_ptr]; xeq_level[p]:=l;
        $globals.eqtb[$p] = $globals.save_stack[$globals.save_ptr];
        $globals.xeq_level[$p] = $l;
        // @!stat if tracing_restores>0 then restore_trace(p,"restoring");@+tats@;@/
        crate::region_stat! {
            if tracing_restores!($globals) > 0 {
                restore_trace($globals, $p, crate::strpool_str!("restoring"));
            }
            use crate::section_0236::tracing_restores;
            use crate::section_0284::restore_trace;
        }
        // end
    }
    // else begin
    else {
        // @!stat if tracing_restores>0 then restore_trace(p,"retaining");@+tats@;@/
        crate::region_stat! {
            if tracing_restores!($globals) > 0 {
                restore_trace($globals, $p, crate::strpool_str!("retaining"));
            }
            use crate::section_0236::tracing_restores;
            use crate::section_0284::restore_trace;
        }
        // end
    }
    crate::region_non_stat! {
        crate::submit_strpool_str!("restoring");
        crate::submit_strpool_str!("retaining");
    };
    use crate::section_0115::pointer;
    use crate::section_0221::eq_level;
    use crate::section_0221::level_one;
    use crate::section_0230::int_base;
    use crate::section_0275::eq_destroy;
}}
