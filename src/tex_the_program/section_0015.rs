//! @ Labels are given symbolic names by the following definitions, so that
//! occasional |goto| statements will be meaningful. We insert the label
//! `|exit|' just before the `\ignorespaces|end|\unskip' of a procedure in
//! which we have used the `|return|' statement defined below; the label
//! `|restart|' is occasionally used at the very beginning of a procedure; and
//! the label `|reswitch|' is occasionally used just prior to a |case|
//! statement in which some cases change the conditions and we wish to branch
//! to the newly applicable case.  Loops that are set up with the |loop|
//! construction defined below are commonly exited by going to `|done|' or to
//! `|found|' or to `|not_found|', and they are sometimes repeated by going to
//! `|continue|'.  If two or more parts of a subroutine start differently but
//! end up the same, the shared code may be gathered together at
//! `|common_ending|'.
//!
//! Incidentally, this program never declares a label that isn't actually used,
//! because some fussy \PASCAL\ compilers will complain about redundant labels.
//
// @d exit=10 {go here to leave a procedure}
// @d restart=20 {go here to start a procedure again}
// @d reswitch=21 {go here to start a case statement again}
// @d continue=22 {go here to resume a loop}
// @d done=30 {go here to exit a loop}
// @d done1=31 {like |done|, when there is more than one loop}
// @d done2=32 {for exiting the second loop in a long block}
// @d done3=33 {for exiting the third loop in a very long block}
// @d done4=34 {for exiting the fourth loop in an extremely long block}
// @d done5=35 {for exiting the fifth loop in an immense block}
// @d done6=36 {for exiting the sixth loop in a block}
// @d found=40 {go here when you've found it}
// @d found1=41 {like |found|, when there's more than one per routine}
// @d found2=42 {like |found|, when there's more than two per routine}
// @d not_found=45 {go here when you've found nothing}
// @d common_ending=50 {go here when you want to merge with another branch}
//
