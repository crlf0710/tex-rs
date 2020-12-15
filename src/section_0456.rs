//! @ @<Scan for \(m)\.{mu} units and |goto attach_fraction|@>=
//! if scan_keyword("mu") then goto attach_fraction
//! @.mu@>
//! else  begin print_err("Illegal unit of measure ("); print("mu inserted)");
//! @.Illegal unit of measure@>
//!   help4("The unit of measurement in math glue must be mu.")@/
//!     ("To recover gracefully from this error, it's best to")@/
//!     ("delete the erroneous units; e.g., type `2' to delete")@/
//!     ("two letters. (See Chapter 27 of The TeXbook.)");
//! @:TeXbook}{\sl The \TeX book@>
//!   error; goto attach_fraction;
//!   end
//!