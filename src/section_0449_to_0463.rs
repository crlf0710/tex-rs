//! @ @<Fetch an internal dimension and |goto attach_sign|...@>=
//! if mu then
//!   begin scan_something_internal(mu_val,false);
//!   @<Coerce glue to a dimension@>;
//!   if cur_val_level=mu_val then goto attach_sign;
//!   if cur_val_level<>int_val then mu_error;
//!   end
//! else  begin scan_something_internal(dimen_val,false);
//!   if cur_val_level=dimen_val then goto attach_sign;
//!   end
//!
//! @ @<Local variables for dimension calculations@>=
//! @!num,@!denom:1..65536; {conversion ratio for the scanned units}
//! @!k,@!kk:small_number; {number of digits in a decimal fraction}
//! @!p,@!q:pointer; {top of decimal digit stack}
//! @!v:scaled; {an internal dimension}
//! @!save_cur_val:integer; {temporary storage of |cur_val|}
//!
//! @ The following code is executed when |scan_something_internal| was
//! called asking for |mu_val|, when we really wanted a ``mudimen'' instead
//! of ``muglue.''
//!
//! @<Coerce glue to a dimension@>=
//! if cur_val_level>=glue_val then
//!   begin v:=width(cur_val); delete_glue_ref(cur_val); cur_val:=v;
//!   end
//!
//! @ When the following code is executed, we have |cur_tok=point_token|, but this
//! token has been backed up using |back_input|; we must first discard it.
//!
//! It turns out that a decimal point all by itself is equivalent to `\.{0.0}'.
//! Let's hope people don't use that fact.
//!
//! @<Scan decimal fraction@>=
//! begin k:=0; p:=null; get_token; {|point_token| is being re-scanned}
//! loop@+  begin get_x_token;
//!   if (cur_tok>zero_token+9)or(cur_tok<zero_token) then goto done1;
//!   if k<17 then {digits for |k>=17| cannot affect the result}
//!     begin q:=get_avail; link(q):=p; info(q):=cur_tok-zero_token;
//!     p:=q; incr(k);
//!     end;
//!   end;
//! done1: for kk:=k downto 1 do
//!   begin dig[kk-1]:=info(p); q:=p; p:=link(p); free_avail(q);
//!   end;
//! f:=round_decimals(k);
//! if cur_cmd<>spacer then back_input;
//! end
//!
//! @ Now comes the harder part: At this point in the program, |cur_val| is a
//! nonnegative integer and $f/2^{16}$ is a nonnegative fraction less than 1;
//! we want to multiply the sum of these two quantities by the appropriate
//! factor, based on the specified units, in order to produce a |scaled|
//! result, and we want to do the calculation with fixed point arithmetic that
//! does not overflow.
//!
//! @<Scan units and set |cur_val| to $x\cdot(|cur_val|+f/2^{16})$...@>=
//! if inf then @<Scan for \(f)\.{fil} units; |goto attach_fraction| if found@>;
//! @<Scan for \(u)units that are internal dimensions;
//!   |goto attach_sign| with |cur_val| set if found@>;
//! if mu then @<Scan for \(m)\.{mu} units and |goto attach_fraction|@>;
//! if scan_keyword("true") then @<Adjust \(f)for the magnification ratio@>;
//! @.true@>
//! if scan_keyword("pt") then goto attach_fraction; {the easy case}
//! @.pt@>
//! @<Scan for \(a)all other units and adjust |cur_val| and |f| accordingly;
//!   |goto done| in the case of scaled points@>;
//! attach_fraction: if cur_val>=@'40000 then arith_error:=true
//! else cur_val:=cur_val*unity+f;
//! done:
//!
//! @ A specification like `\.{filllll}' or `\.{fill L L L}' will lead to two
//! error messages (one for each additional keyword \.{"l"}).
//!
//! @<Scan for \(f)\.{fil} units...@>=
//! if scan_keyword("fil") then
//! @.fil@>
//!   begin cur_order:=fil;
//!   while scan_keyword("l") do
//!     begin if cur_order=filll then
//!       begin print_err("Illegal unit of measure (");
//! @.Illegal unit of measure@>
//!       print("replaced by filll)");
//!       help1("I dddon't go any higher than filll."); error;
//!       end
//!     else incr(cur_order);
//!     end;
//!   goto attach_fraction;
//!   end
//!
//! @ @<Scan for \(u)units that are internal dimensions...@>=
//! save_cur_val:=cur_val;
//! @<Get the next non-blank non-call...@>;
//! if (cur_cmd<min_internal)or(cur_cmd>max_internal) then back_input
//! else  begin if mu then
//!     begin scan_something_internal(mu_val,false); @<Coerce glue...@>;
//!     if cur_val_level<>mu_val then mu_error;
//!     end
//!   else scan_something_internal(dimen_val,false);
//!   v:=cur_val; goto found;
//!   end;
//! if mu then goto not_found;
//! if scan_keyword("em") then v:=(@<The em width for |cur_font|@>)
//! @.em@>
//! else if scan_keyword("ex") then v:=(@<The x-height for |cur_font|@>)
//! @.ex@>
//! else goto not_found;
//! @<Scan an optional space@>;
//! found:cur_val:=nx_plus_y(save_cur_val,v,xn_over_d(v,f,@'200000));
//! goto attach_sign;
//! not_found:
//!
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
//! @ The final member of \TeX's value-scanning trio is |scan_glue|, which
//! makes |cur_val| point to a glue specification. The reference count of that
//! glue spec will take account of the fact that |cur_val| is pointing to~it.
//!
//! The |level| parameter should be either |glue_val| or |mu_val|.
//!
//! Since |scan_dimen| was so much more complex than |scan_int|, we might expect
//! |scan_glue| to be even worse. But fortunately, it is very simple, since
//! most of the work has already been done.
//!
//! @p procedure scan_glue(@!level:small_number);
//!   {sets |cur_val| to a glue spec pointer}
//! label exit;
//! var negative:boolean; {should the answer be negated?}
//! @!q:pointer; {new glue specification}
//! @!mu:boolean; {does |level=mu_val|?}
//! begin mu:=(level=mu_val); @<Get the next non-blank non-sign...@>;
//! if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
//!   begin scan_something_internal(level,negative);
//!   if cur_val_level>=glue_val then
//!     begin if cur_val_level<>level then mu_error;
//!     return;
//!     end;
//!   if cur_val_level=int_val then scan_dimen(mu,false,true)
//!   else if level=mu_val then mu_error;
//!   end
//! else  begin back_input; scan_dimen(mu,false,false);
//!   if negative then negate(cur_val);
//!   end;
//! @<Create a new glue specification whose width is |cur_val|; scan for its
//!   stretch and shrink components@>;
//! exit:end;
//!
//! @ @<Create a new glue specification whose width is |cur_val|...@>=
//! q:=new_spec(zero_glue); width(q):=cur_val;
//! if scan_keyword("plus") then
//! @.plus@>
//!   begin scan_dimen(mu,true,false);
//!   stretch(q):=cur_val; stretch_order(q):=cur_order;
//!   end;
//! if scan_keyword("minus") then
//! @.minus@>
//!   begin scan_dimen(mu,true,false);
//!   shrink(q):=cur_val; shrink_order(q):=cur_order;
//!   end;
//! cur_val:=q
//!
//! @ Here's a similar procedure that returns a pointer to a rule node. This
//! routine is called just after \TeX\ has seen \.{\\hrule} or \.{\\vrule};
//! therefore |cur_cmd| will be either |hrule| or |vrule|. The idea is to store
//! the default rule dimensions in the node, then to override them if
//! `\.{height}' or `\.{width}' or `\.{depth}' specifications are
//! found (in any order).
//!
//! @d default_rule=26214 {0.4\thinspace pt}
//!
//! @p function scan_rule_spec:pointer;
//! label reswitch;
//! var q:pointer; {the rule node being created}
//! begin q:=new_rule; {|width|, |depth|, and |height| all equal |null_flag| now}
//! if cur_cmd=vrule then width(q):=default_rule
//! else  begin height(q):=default_rule; depth(q):=0;
//!   end;
//! reswitch: if scan_keyword("width") then
//! @.width@>
//!   begin scan_normal_dimen; width(q):=cur_val; goto reswitch;
//!   end;
//! if scan_keyword("height") then
//! @.height@>
//!   begin scan_normal_dimen; height(q):=cur_val; goto reswitch;
//!   end;
//! if scan_keyword("depth") then
//! @.depth@>
//!   begin scan_normal_dimen; depth(q):=cur_val; goto reswitch;
//!   end;
//! scan_rule_spec:=q;
//! end;
//!
