//! @ The |prepare_mag| subroutine is called whenever \TeX\ wants to use |mag|
//! for magnification.
//
// @p procedure prepare_mag;
pub(crate) fn prepare_mag(globals: &mut TeXGlobals) {
    // begin if (mag_set>0)and(mag<>mag_set) then
    if globals.mag_set > 0 && mag!(globals) != globals.mag_set {
        // begin print_err("Incompatible magnification ("); print_int(mag);
        // @.Incompatible magnification@>
        // print(");"); print_nl(" the previous value will be retained");
        // help2("I can handle only one magnification ratio per job. So I've")@/
        // ("reverted to the magnification you used earlier on this run.");@/
        // int_error(mag_set);
        // geq_word_define(int_base+mag_code,mag_set); {|mag:=mag_set|}
        // end;
    }
    // if (mag<=0)or(mag>32768) then
    if mag!(globals) <= 0 || mag!(globals) > 32768 {
        //   begin print_err("Illegal magnification has been changed to 1000");@/
        // @.Illegal magnification...@>
        //   help1("The magnification ratio must be between 1 and 32768.");
        //   int_error(mag); geq_word_define(int_base+mag_code,1000);
        //   end;
    }
    // mag_set:=mag;
    globals.mag_set = mag!(globals);
    // end;
}

use crate::section_0004::TeXGlobals;
