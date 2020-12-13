//! @ The following code is executed when |scan_something_internal| was
//! called asking for |mu_val|, when we really wanted a ``mudimen'' instead
//! of ``muglue.''
//!
//! @<Coerce glue to a dimension@>=
//! if cur_val_level>=glue_val then
//!   begin v:=width(cur_val); delete_glue_ref(cur_val); cur_val:=v;
//!   end
//!
