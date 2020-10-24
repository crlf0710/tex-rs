//! @ Control sequence names are scanned only when they appear in some line of
//! a file; once they have been scanned the first time, their |eqtb| location
//! serves as a unique identification, so \TeX\ doesn't need to refer to the
//! original name any more except when it prints the equivalent in symbolic form.
//!
//! The program that scans a control sequence has been written carefully
//! in order to avoid the blowups that might otherwise occur if a malicious
//! user tried something like `\.{\\catcode\'15=0}'. The algorithm might
//! look at |buffer[limit+1]|, but it never looks at |buffer[limit+2]|.
//!
//! If expanded characters like `\.{\^\^A}' or `\.{\^\^df}'
//! appear in or just following
//! a control sequence name, they are converted to single characters in the
//! buffer and the process is repeated, slowly but surely.

// @<Scan a control...@>=
macro_rules! Scan_a_control_sequence_and_set_state_skip_blanks_or_mid_line {
    ($globals:expr) => {
        // begin if loc>limit then cur_cs:=null_cs {|state| is irrelevant in this case}
        // else  begin start_cs: k:=loc; cur_chr:=buffer[k]; cat:=cat_code(cur_chr);
        //   incr(k);
        //   if cat=letter then state:=skip_blanks
        //   else if cat=spacer then state:=skip_blanks
        //   else state:=mid_line;
        //   if (cat=letter)and(k<=limit) then
        //     @<Scan ahead in the buffer until finding a nonletter;
        //     if an expanded code is encountered, reduce it
        //     and |goto start_cs|; otherwise if a multiletter control
        //     sequence is found, adjust |cur_cs| and |loc|, and
        //     |goto found|@>
        //   else @<If an expanded code is present, reduce it and |goto start_cs|@>;
        //   cur_cs:=single_base+buffer[loc]; incr(loc);
        //   end;
        // found: cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
        // if cur_cmd>=outer_call then check_outer_validity;
        // end
        todo!();
    }
}
