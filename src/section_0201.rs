//! @ Similarly, |delete_glue_ref| is called when a pointer to a glue
//! specification is being withdrawn.
//! @^reference counts@>
//! @d fast_delete_glue_ref(#)==@t@>@;@/
//!   begin if glue_ref_count(#)=null then free_node(#,glue_spec_size)
//!   else decr(glue_ref_count(#));
//!   end
//!
//! @p procedure delete_glue_ref(@!p:pointer); {|p| points to a glue specification}
//! fast_delete_glue_ref(p);
//!
