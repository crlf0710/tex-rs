//! ` `

// @<Scan for \(m)\.{mu} units and |goto attach_fraction|@>=
macro_rules! Scan_for_m_mu_units_and_goto_attach_fraction {
    ($globals:expr, $lbl_attach_fraction:lifetime) => {{
        // if scan_keyword("mu") then goto attach_fraction
        if scan_keyword($globals, strpool_str!("mu"))? {
            goto_forward_label!($lbl_attach_fraction);
        }
        // @.mu@>
        // else  begin print_err("Illegal unit of measure ("); print("mu inserted)");
        else {
            todo!("illegal unit");
            // @.Illegal unit of measure@>
            //   help4("The unit of measurement in math glue must be mu.")@/
            //     ("To recover gracefully from this error, it's best to")@/
            //     ("delete the erroneous units; e.g., type `2' to delete")@/
            //     ("two letters. (See Chapter 27 of The TeXbook.)");
            // @:TeXbook}{\sl The \TeX book@>
            //   error; goto attach_fraction;
            //   end
        }
    }}
}
