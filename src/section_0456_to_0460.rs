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
//! @ @<Adjust \(f)for the magnification ratio@>=
//! begin prepare_mag;
//! if mag<>1000 then
//!   begin cur_val:=xn_over_d(cur_val,1000,mag);
//!   f:=(1000*f+@'200000*remainder) div mag;
//!   cur_val:=cur_val+(f div @'200000); f:=f mod @'200000;
//!   end;
//! end
//!
//! @ The necessary conversion factors can all be specified exactly as
//! fractions whose numerator and denominator sum to 32768 or less.
//! According to the definitions here, $\rm2660\,dd\approx1000.33297\,mm$;
//! this agrees well with the value $\rm1000.333\,mm$ cited by Bosshard
//! @^Bosshard, Hans Rudolf@>
//! in {\sl Technische Grundlagen zur Satzherstellung\/} (Bern, 1980).
//!
//! @d set_conversion_end(#)== denom:=#; end
//! @d set_conversion(#)==@+begin num:=#; set_conversion_end
//!
//! @<Scan for \(a)all other units and adjust |cur_val| and |f|...@>=
//! if scan_keyword("in") then set_conversion(7227)(100)
//! @.in@>
//! else if scan_keyword("pc") then set_conversion(12)(1)
//! @.pc@>
//! else if scan_keyword("cm") then set_conversion(7227)(254)
//! @.cm@>
//! else if scan_keyword("mm") then set_conversion(7227)(2540)
//! @.mm@>
//! else if scan_keyword("bp") then set_conversion(7227)(7200)
//! @.bp@>
//! else if scan_keyword("dd") then set_conversion(1238)(1157)
//! @.dd@>
//! else if scan_keyword("cc") then set_conversion(14856)(1157)
//! @.cc@>
//! else if scan_keyword("sp") then goto done
//! @.sp@>
//! else @<Complain about unknown unit and |goto done2|@>;
//! cur_val:=xn_over_d(cur_val,num,denom);
//! f:=(num*f+@'200000*remainder) div denom;@/
//! cur_val:=cur_val+(f div @'200000); f:=f mod @'200000;
//! done2:
//!
//! @ @<Complain about unknown unit...@>=
//! begin print_err("Illegal unit of measure ("); print("pt inserted)");
//! @.Illegal unit of measure@>
//! help6("Dimensions can be in units of em, ex, in, pt, pc,")@/
//!   ("cm, mm, dd, cc, bp, or sp; but yours is a new one!")@/
//!   ("I'll assume that you meant to say pt, for printer's points.")@/
//!   ("To recover gracefully from this error, it's best to")@/
//!   ("delete the erroneous units; e.g., type `2' to delete")@/
//!   ("two letters. (See Chapter 27 of The TeXbook.)");
//! @:TeXbook}{\sl The \TeX book@>
//! error; goto done2;
//! end
//!
//!
//! @ @<Report that this dimension is out of range@>=
//! begin print_err("Dimension too large");
//! @.Dimension too large@>
//! help2("I can't work with sizes bigger than about 19 feet.")@/
//!   ("Continue and I'll use the largest value I can.");@/
//! error; cur_val:=max_dimen; arith_error:=false;
//! end
//!
