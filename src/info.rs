//! Unlimited copying and redistribution of the original `tex.web` file is
//! authorized are permitted as long as this file is not modified.
//! Modifications are permitted, but only if the resulting file is not named
//! tex.web. (The WEB system provides for alterations via an auxiliary file;
//! the master file should stay intact.) See Appendix H of the WEB manual
//! for hints on how to install this program. And see Appendix A of the TRIP manual
//! for details about how to validate it.
//!
//! Version 0 was released in September 1982 after it passed a variety of tests.
//! Version 1 was released in November 1983 after thorough testing.
//! Version 1.1 fixed ``disappearing font identifiers'' et alia (July 1984).
//! Version 1.2 allowed `0' in response to an error, et alia (October 1984).
//! Version 1.3 made memory allocation more flexible and local (November 1984).
//! Version 1.4 fixed accents right after line breaks, et alia (April 1985).
//! Version 1.5 fixed \the\toks after other expansion in \edefs (August 1985).
//! Version 2.0 (almost identical to 1.5) corresponds to "Volume B" (April 1986).
//! Version 2.1 corrected anomalies in discretionary breaks (January 1987).
//! Version 2.2 corrected "(Please type...)" with null \endlinechar (April 1987).
//! Version 2.3 avoided incomplete page in premature termination (August 1987).
//! Version 2.4 fixed \noaligned rules in indented displays (August 1987).
//! Version 2.5 saved cur_order when expanding tokens (September 1987).
//! Version 2.6 added 10sp slop when shipping leaders (November 1987).
//! Version 2.7 improved rounding of negative-width characters (November 1987).
//! Version 2.8 fixed weird bug if no \patterns are used (December 1987).
//! Version 2.9 made \csname\endcsname's "relax" local (December 1987).
//! Version 2.91 fixed \outer\def\a0{}\a\a bug (April 1988).
//! Version 2.92 fixed \patterns, also file names with complex macros (May 1988).
//! Version 2.93 fixed negative halving in allocator when mem_min<0 (June 1988).
//! Version 2.94 kept open_log_file from calling fatal_error (November 1988).
//! Version 2.95 solved that problem a better way (December 1988).
//! Version 2.96 corrected bug in "Infinite shrinkage" recovery (January 1989).
//! Version 2.97 corrected blunder in creating 2.95 (February 1989).
//! Version 2.98 omitted save_for_after at outer level (March 1989).
//! Version 2.99 caught $$\begingroup\halign..$$ (June 1989).
//! Version 2.991 caught .5\ifdim.6... (June 1989).
//! Version 2.992 introduced major changes for 8-bit extensions (September 1989).
//! Version 2.993 fixed a save_stack synchronization bug et alia (December 1989).
//! Version 3.0 fixed unusual displays; was more \output robust (March 1990).
//! Version 3.1 fixed nullfont, disabled \write{\the\prevgraf} (September 1990).
//! Version 3.14 fixed unprintable font names and corrected typos (March 1991).
//! Version 3.141 more of same; reconstituted ligatures better (March 1992).
//! Version 3.1415 preserved nonexplicit kerns, tidied up (February 1993).
//! Version 3.14159 allowed fontmemsize to change; bulletproofing (March 1995).
//! Version 3.141592 fixed \xleaders, glueset, weird alignments (December 2002).
//! Version 3.1415926 was a general cleanup with minor fixes (February 2008).
//! Version 3.14159265 was similar (January 2014).
//! Version 3.141592653 was similar but more extensive (January 2021).
//!
//! A reward of $327.68 will be paid to the first finder of any remaining bug of
//! the original TeX program.

#![allow(unused_macros)]

pub(crate) macro migration_complete() {}

pub(crate) macro documentation_adjusted() {}

pub(crate) macro moved_to_inner_scope() {}

pub(crate) macro region_forward_label
(|$lbl_:lifetime| {$($s: stmt)*} $lbl:lifetime <- ) {
    #[allow(redundant_semicolons, unused_labels, unreachable_code)]
    $lbl : loop {
        $($s)*;
        break;
    }
}

pub(crate) macro region_backward_label
    ($lbl:lifetime <- {$($s: stmt)*} |$lbl_:lifetime| ) {
        #[allow(redundant_semicolons, unused_labels, unreachable_code)]
        $lbl : loop {
            $($s)*;
            break;
        }
    }

pub(crate) macro goto_forward_label($lbl:lifetime) {
    break $lbl
}

pub(crate) macro goto_backward_label
    ($lbl:lifetime) {
        continue $lbl
    }

pub(crate) macro region_multipart {
    (($lbl_block:lifetime, $part_idx:expr) {
        $($part:pat => {$($s: stmt)*},)*
    }) => {
        $lbl_block: loop {
            #[allow(unreachable_patterns)]
            match $part_idx {
                $($part => {
                    $($s)*;
                    continue $lbl_block;
                })*
                _ => {
                    break $lbl_block;
                }
            }
        }
    }
}

pub(crate) macro goto_part_label
    ($lbl:lifetime, $status:expr, $label_val:expr) {
        $status = $label_val;
        continue $lbl;
    }

pub(crate) macro region_multipart_autoincr
    (($lbl_block:lifetime, $part_idx:expr) {
        $($part:pat => {$($s: stmt)*},)*
    }) {
        $lbl_block: loop {
            #[allow(unreachable_patterns)]
            match $part_idx {
                $($part => {
                    $($s)*;
                    $part_idx += 1;
                    continue $lbl_block;
                })*
                _ => {
                    break $lbl_block;
                }
            }
        }
    }

pub(crate) macro region_initex
    ($($statements:tt)* ) {
        #[cfg(feature = "initex")]
        {
            $($statements)*
        }
    }

pub(crate) macro region_debug
    ($($statements:tt)* ) {
        #[cfg(all(feature = "debugging", debug_assertions))]
        {
            $($statements)*
        }
    }

pub(crate) macro region_stat
    ($($statements:tt)* ) {
        #[cfg(feature = "statistics")]
        {
            $($statements)*
        }
    }

pub(crate) macro region_non_stat
    ($($statements:tt)* ) {
        #[cfg(not(feature = "statistics"))]
        {
            $($statements)*
        }
    }

pub(crate) macro impl_debug_with_literal {
    ($impl_type:ident, $literal: expr) => {
        impl core::fmt::Debug for $impl_type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $literal)?;
                Ok(())
            }
        }
    },
    ($impl_type:ident [ $($generics:tt)* ] , $literal: expr) => {
        impl<$($generics)*> core::fmt::Debug for $impl_type<$($generics)*> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $literal)?;
                Ok(())
            }
        }
    }
}

pub(crate) macro trace_span($span_info:expr) {
    #[cfg(feature = "trace")]
    let span = ::tracing::span!(::tracing::Level::TRACE, $span_info);
    #[cfg(feature = "trace")]
    let __ = span.enter();
}

pub(crate) macro trace_span_verbose($span_info:expr) {
    #[cfg(feature = "trace_verbose")]
    let span = ::tracing::span!(::tracing::Level::TRACE, $span_info);
    #[cfg(feature = "trace_verbose")]
    let __ = span.enter();
}

pub(crate) macro trace_debug_span($span_info:expr) {
    #[cfg(feature = "trace")]
    let span = ::tracing::span!(::tracing::Level::DEBUG, $span_info);
    #[cfg(feature = "trace")]
    let __ = span.enter();
}

pub(crate) macro trace_error_span($span_info:expr) {
    #[cfg(feature = "trace")]
    let span = ::tracing::span!(::tracing::Level::ERROR, $span_info);
    #[cfg(feature = "trace")]
    let __ = span.enter();
}

pub(crate) macro trace_expr
    ($($x:tt)*) {
        #[cfg(feature = "trace")]
        {
            tracing::trace!($($x)*);
        }
    }

pub(crate) macro trace_expr_verbose
    ($($x:tt)*) {
        #[cfg(feature = "trace_verbose")]
        {
            tracing::trace!($($x)*);
        }
    }

#[allow(unused_macros)]
pub(crate) macro trace_debug_expr
    ($($x:tt)*) {
        #[cfg(feature = "trace")]
        {
            tracing::debug!($($x)*);
        }
    }

#[allow(unused_macros)]
pub(crate) macro trace_debug_expr_verbose
    ($($x:tt)*) {
        #[cfg(feature = "trace_verbose")]
        {
            tracing::debug!($($x)*);
        }
    }

#[allow(unused_macros)]
pub(crate) macro trace_error_expr
    ($($x:tt)*) {
        #[cfg(feature = "trace")]
        {
            tracing::error!($($x)*);
        }
    }

pub(crate) macro return_nojump {
    () => {
        return Ok(());
    },
    ($val: expr) => {
        return Ok($val);
    }
}

pub(crate) macro ok_nojump {
    () => {
        Ok::<_, crate::section_0081::JumpOutToEndOfTEX>(())
    },
    ($val: expr) => {
        Ok::<_, crate::section_0081::JumpOutToEndOfTEX>($val)
    }
}
