//! @ When \.{\\halign} is used as a displayed formula, there should be
//! no other pieces of mlists present.
//
// @<Check for improper alignment in displayed math@>=
macro_rules! Check_for_improper_alignment_in_displayed_math {
    ($globals:expr) => {{
        // if (mode=mmode)and((tail<>head)or(incompleat_noad<>null)) then
        if mode!($globals) == mmode
            && (tail!($globals) != head!($globals) || incompleat_noad!($globals) as pointer != null)
        {
            todo!("improper alignment");
            //   begin print_err("Improper "); print_esc("halign"); print(" inside $$'s");
            // @.Improper \\halign...@>
            //   help3("Displays can use special alignments (like \eqalignno)")@/
            //   ("only if nothing but the alignment itself is between $$'s.")@/
            //   ("So I've deleted the formulas that preceded this alignment.");
            //   error; flush_math;
            //   end
        }
        use crate::section_0211::mmode;
    }};
}
