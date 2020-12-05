//! @ Constructions like `\.{-\'77 pt}' are legal dimensions, so |scan_dimen|
//! may begin with |scan_int|. This explains why it is convenient to use
//! |scan_int| also for the integer part of a decimal fraction.
//!
//! Several branches of |scan_dimen| work with |cur_val| as an integer and
//! with an auxiliary fraction |f|, so that the actual quantity of interest is
//! $|cur_val|+|f|/2^{16}$. At the end of the routine, this ``unpacked''
//! representation is put into the single word |cur_val|, which suddenly
//! switches significance from |integer| to |scaled|.
//
// @d attach_fraction=88 {go here to pack |cur_val| and |f| into |cur_val|}
// @d attach_sign=89 {go here when |cur_val| is correct except perhaps for sign}
// @d scan_normal_dimen==scan_dimen(false,false,false)
#[allow(unused_macros)]
macro_rules! scan_normal_dimen {
    ($globals:expr) => {
        crate::section_0448::scan_dimen($globals, false, false, false)
    };
}

// @p procedure scan_dimen(@!mu,@!inf,@!shortcut:boolean);
//   {sets |cur_val| to a dimension}
/// sets `cur_val` to a dimension
#[allow(unused_variables)]
pub(crate) fn scan_dimen(globals: &mut TeXGlobals, mu: boolean, inf: boolean, shortcut: boolean) {
    todo!("scan dimen");
    // label done, done1, done2, found, not_found, attach_fraction, attach_sign;
    // var negative:boolean; {should the answer be negated?}
    // @!f:integer; {numerator of a fraction whose denominator is $2^{16}$}
    // @<Local variables for dimension calculations@>@;
    // begin f:=0; arith_error:=false; cur_order:=normal; negative:=false;
    // if not shortcut then
    //   begin @<Get the next non-blank non-sign...@>;
    //   if (cur_cmd>=min_internal)and(cur_cmd<=max_internal) then
    //     @<Fetch an internal dimension and |goto attach_sign|,
    //       or fetch an internal integer@>
    //   else  begin back_input;
    //     if cur_tok=continental_point_token then cur_tok:=point_token;
    //     if cur_tok<>point_token then scan_int
    //     else  begin radix:=10; cur_val:=0;
    //       end;
    //     if cur_tok=continental_point_token then cur_tok:=point_token;
    //     if (radix=10)and(cur_tok=point_token) then @<Scan decimal fraction@>;
    //     end;
    //   end;
    // if cur_val<0 then {in this case |f=0|}
    //   begin negative := not negative; negate(cur_val);
    //   end;
    // @<Scan units and set |cur_val| to $x\cdot(|cur_val|+f/2^{16})$, where there
    //   are |x| sp per unit; |goto attach_sign| if the units are internal@>;
    // @<Scan an optional space@>;
    // attach_sign: if arith_error or(abs(cur_val)>=@'10000000000) then
    //   @<Report that this dimension is out of range@>;
    // if negative then negate(cur_val);
    // end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
