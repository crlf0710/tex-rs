//! @ We have already printed a lot of statistics, so we set |tracing_stats:=0|
//! to prevent them from appearing again.
//
// @<Dump a couple more things and the closing check word@>=
macro_rules! Dump_a_couple_more_things_and_the_closing_check_word {
    ($globals:expr) => {{
        // dump_int(interaction); dump_int(format_ident); dump_int(69069);
        dump_int!($globals, $globals.interaction.get() as _);
        dump_int!($globals, $globals.format_ident.get() as _);
        dump_int!($globals, 69069);
        // tracing_stats:=0
        tracing_stats!($globals) = 0;
    }}
}
