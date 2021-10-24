#![allow(irrefutable_let_patterns)]

#[path = "auxiliary/vfs.rs"]
mod vfs;

use vfs::prepare_pool;
use vfs::TeXTestVFS;

const TRIPFILE_TRIP_PL: &[u8] = include_bytes!("../tests_data/trip/trip.pl");
const TRIPFILE_TRIP_TEX: &[u8] = include_bytes!("../tests_data/trip/trip.tex");
const TRIPFILE_TRIPIN_LOG: &[u8] = include_bytes!("../tests_data/trip/tripin.log");

const TRIPRECORD_TRIP_TFM: &[u8] = include_bytes!("../tests_data/trip/trip.tfm");
const TRIPRECORD_TRIPIN_FOT: &[u8] = include_bytes!("../tests_data/trip/tripin.fot");
const TRIPRECORD_TRIPIN_LOG: &[u8] = include_bytes!("../tests_data/trip/tripin.log");
const TRIPRECORD_TRIP_FOT: &[u8] = include_bytes!("../tests_data/trip/trip.fot");
const TRIPRECORD_TRIP_LOG: &[u8] = include_bytes!("../tests_data/trip/trip.log");
const TRIPRECORD_TRIP_TYP: &[u8] = include_bytes!("../tests_data/trip/trip.typ");

#[test]
#[cfg_attr(not(feature = "statistics"), ignore)]
#[ignore]
fn trip() {
    let (
        tripin_fot,
        tripin_log,
        trip_fmt,
        // dump
        trip_fot,
        trip_log,
        trip_dvi,
        tripos_tex,
        eight_terminal_tex,
    ) = prepare_pool().install(|| {
        TeXTestVFS::default()
            .and_then_prepare_file("trip.pl", TRIPFILE_TRIP_PL)
            .and_then_prepare_file("trip.tex", TRIPFILE_TRIP_TEX)
            .install_as_tex_io_handler();
        // step 1.
        // FIXME: currently we don't have PLtoTF and TFtoPL. We'll skip this test and upload the result
        // supplied within knuthdist package.
        TeXTestVFS::with_current(|vfs| vfs.prepare_file("trip.tfm", TRIPRECORD_TRIP_TFM));
        // step 2
        // FIXME: Verify tex configuration.
        // step 3
        let (tripin_fot, tripin_log, trip_fmt) = TeXTestVFS::with_current(|vfs| {
            vfs.prepare_termin(concat!("\n", "\\input trip\n", "\\end\n").as_bytes());
            if let mut globals = tex::TeXGlobals::default() {
                use tex::configure::TeXConfiguration;
                globals.set_error_line(64);
                globals.set_half_error_line(32);
                globals.set_max_print_line(72);
                tex::entry(&mut globals);
            }
            let tripin_fot = vfs.dump_current_term_out().unwrap();
            let tripin_log = vfs.dump_file("trip.log").unwrap();
            //FIXME: let trip_fmt = vfs.dump_file("trip.fmt").unwrap();
            let trip_fmt = Vec::<u8>::new();
            (tripin_fot, tripin_log, trip_fmt)
        });
        // step 4
        let (trip_fot, trip_log, trip_dvi, tripos_tex, eight_terminal_tex) =
            TeXTestVFS::with_current(|vfs| {
                vfs.prepare_termin(concat!(" &trip  trip \n").as_bytes());
                let trip_fot = vfs.dump_current_term_out().unwrap();
                let trip_log = vfs.dump_file("trip.log").unwrap();
                //FIXME: let trip_dvi = vfs.dump_file("trip.dvi").unwrap();
                //FIXME: let tripos_tex = vfs.dump_file("tripos.tex").unwrap();
                //FIXME: let eight_terminal_tex = vfs.dump_file("8terminal.tex").unwrap();
                let trip_dvi = Vec::<u8>::new();
                let tripos_tex = Vec::<u8>::new();
                let eight_terminal_tex = Vec::<u8>::new();
                (trip_fot, trip_log, trip_dvi, tripos_tex, eight_terminal_tex)
            });
        (
            tripin_fot,
            tripin_log,
            trip_fmt,
            // dump
            trip_fot,
            trip_log,
            trip_dvi,
            tripos_tex,
            eight_terminal_tex,
        )
    });
    assert_eq!(
        String::from_utf8_lossy(&tripin_log).as_ref(),
        String::from_utf8_lossy(TRIPRECORD_TRIPIN_LOG).as_ref(),
    );
    assert_eq!(
        String::from_utf8_lossy(&tripin_fot).as_ref(),
        String::from_utf8_lossy(TRIPRECORD_TRIPIN_FOT).as_ref(),
    );
}
