//! @ The hash table is initialized with `\.{\\count}', `\.{\\dimen}', `\.{\\skip}',
//! and `\.{\\muskip}' all having |register| as their command code; they are
//! distinguished by the |chr_code|, which is either |int_val|, |dimen_val|,
//! |glue_val|, or |mu_val|.
//!
//! @<Put each...@>=
//! primitive("count",register,int_val);
//! @!@:count_}{\.{\\count} primitive@>
//! primitive("dimen",register,dimen_val);
//! @!@:dimen_}{\.{\\dimen} primitive@>
//! primitive("skip",register,glue_val);
//! @!@:skip_}{\.{\\skip} primitive@>
//! primitive("muskip",register,mu_val);
//! @!@:mu_skip_}{\.{\\muskip} primitive@>
//!
