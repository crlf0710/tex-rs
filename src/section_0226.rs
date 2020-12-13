//! @ The symbolic names for glue parameters are put into \TeX's hash table
//! by using the routine called |primitive|, defined below. Let us enter them
//! now, so that we don't have to list all those parameter names anywhere else.
//!
//! @<Put each of \TeX's primitives into the hash table@>=
//! primitive("lineskip",assign_glue,glue_base+line_skip_code);@/
//! @!@:line_skip_}{\.{\\lineskip} primitive@>
//! primitive("baselineskip",assign_glue,glue_base+baseline_skip_code);@/
//! @!@:baseline_skip_}{\.{\\baselineskip} primitive@>
//! primitive("parskip",assign_glue,glue_base+par_skip_code);@/
//! @!@:par_skip_}{\.{\\parskip} primitive@>
//! primitive("abovedisplayskip",assign_glue,glue_base+above_display_skip_code);@/
//! @!@:above_display_skip_}{\.{\\abovedisplayskip} primitive@>
//! primitive("belowdisplayskip",assign_glue,glue_base+below_display_skip_code);@/
//! @!@:below_display_skip_}{\.{\\belowdisplayskip} primitive@>
//! primitive("abovedisplayshortskip",
//!   assign_glue,glue_base+above_display_short_skip_code);@/
//! @!@:above_display_short_skip_}{\.{\\abovedisplayshortskip} primitive@>
//! primitive("belowdisplayshortskip",
//!   assign_glue,glue_base+below_display_short_skip_code);@/
//! @!@:below_display_short_skip_}{\.{\\belowdisplayshortskip} primitive@>
//! primitive("leftskip",assign_glue,glue_base+left_skip_code);@/
//! @!@:left_skip_}{\.{\\leftskip} primitive@>
//! primitive("rightskip",assign_glue,glue_base+right_skip_code);@/
//! @!@:right_skip_}{\.{\\rightskip} primitive@>
//! primitive("topskip",assign_glue,glue_base+top_skip_code);@/
//! @!@:top_skip_}{\.{\\topskip} primitive@>
//! primitive("splittopskip",assign_glue,glue_base+split_top_skip_code);@/
//! @!@:split_top_skip_}{\.{\\splittopskip} primitive@>
//! primitive("tabskip",assign_glue,glue_base+tab_skip_code);@/
//! @!@:tab_skip_}{\.{\\tabskip} primitive@>
//! primitive("spaceskip",assign_glue,glue_base+space_skip_code);@/
//! @!@:space_skip_}{\.{\\spaceskip} primitive@>
//! primitive("xspaceskip",assign_glue,glue_base+xspace_skip_code);@/
//! @!@:xspace_skip_}{\.{\\xspaceskip} primitive@>
//! primitive("parfillskip",assign_glue,glue_base+par_fill_skip_code);@/
//! @!@:par_fill_skip_}{\.{\\parfillskip} primitive@>
//! primitive("thinmuskip",assign_mu_glue,glue_base+thin_mu_skip_code);@/
//! @!@:thin_mu_skip_}{\.{\\thinmuskip} primitive@>
//! primitive("medmuskip",assign_mu_glue,glue_base+med_mu_skip_code);@/
//! @!@:med_mu_skip_}{\.{\\medmuskip} primitive@>
//! primitive("thickmuskip",assign_mu_glue,glue_base+thick_mu_skip_code);@/
//! @!@:thick_mu_skip_}{\.{\\thickmuskip} primitive@>
//!
