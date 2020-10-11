// This program is copyright (C) 1982 by D. E. Knuth; all rights are reserved.

// TeX is a trademark of the American Mathematical Society.
// METAFONT is a trademark of Addison-Wesley Publishing Company.

// Although considerable effort has been expended to make the TeX program
// correct and reliable, no warranty is implied; the author disclaims any
// obligation or liability for damages, including but not limited to
// special, indirect, or consequential damages arising out of or in
// connection with the use or performance of this software. This work has
// been a ``labor of love'' and the author hopes that users enjoy it.

#![deny(warnings, missing_docs, missing_debug_implementations)]
#![allow(
    dead_code,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports,
    unused_doc_comments
)]
//! This is `TeX`, a document compiler intended to produce typesetting of high quality.

#[macro_use]
mod info;
#[macro_use]
mod pascal;

mod section_0001;
mod section_0002;
mod section_0003;
#[macro_use]
mod section_0004;
mod section_0005;
mod section_0006_to_0010;
mod section_0011;
mod section_0012;
mod section_0013_to_0015;
#[macro_use]
mod section_0016;
mod section_0017;
#[macro_use]
mod section_0018;
mod section_0019;
mod section_0020;
mod section_0021;
mod section_0022_to_0024;
mod section_0025;
mod section_0026_to_0029;
mod section_0030;
mod section_0031;
mod section_0032;
mod section_0033;
mod section_0034;
mod section_0035;
#[macro_use]
mod section_0036;
mod section_0037;
mod section_0038;
mod section_0039_to_0053;
mod section_0054;
mod section_0055;
mod section_0056;
mod section_0057;
mod section_0058_to_0059;
mod section_0060;
#[macro_use]
mod section_0061;
mod section_0062_to_0071;
mod section_0072;
mod section_0073;
mod section_0074;
mod section_0075;
mod section_0076;
mod section_0077_to_0098;
mod section_0099_to_0100;
mod section_0101;
mod section_0102_to_0108;
mod section_0109;
mod section_0110;
mod section_0111;
mod section_0112;
mod section_0113;
mod section_0114;
mod section_0115;
mod section_0116_to_0132;
mod section_0133_to_0161;
mod section_0162_to_0172;
mod section_0173_to_0198;
mod section_0199_to_0202;
mod section_0203_to_0206;
mod section_0207;
mod section_0208;
mod section_0209;
mod section_0210;
mod section_0211_to_0219;
mod section_0220;
#[macro_use]
mod section_0221;
mod section_0222;
mod section_0223;
mod section_0224;
mod section_0225_to_0229;
#[macro_use]
mod section_0230;
mod section_0231_to_0235;
mod section_0236;
mod section_0237_to_0246;
mod section_0247;
mod section_0248_to_0252;
mod section_0253;
mod section_0254;
mod section_0255;
mod section_0256_to_0263;
mod section_0264;
mod section_0265;
mod section_0266;
mod section_0267;
mod section_0268_to_0288;
mod section_0289;
mod section_0290_to_0296;
mod section_0297;
mod section_0298;
mod section_0299;
mod section_0300;
mod section_0301;
#[macro_use]
mod section_0302;
mod section_0303;
#[macro_use]
mod section_0304;
mod section_0305_to_0306;
mod section_0307;
mod section_0308_to_0320;
mod section_0321_to_0323;
mod section_0324;
mod section_0325_to_0330;
#[macro_use]
mod section_0331;
mod section_0332_to_0340;
reversing_order_items!(
    {
        mod section_0341;
        mod section_0342;
    }
    {
        #[macro_use]
        mod section_0343;
    }
    {
        mod section_0344_to_0356;
    }
    {
        #[macro_use]
        mod section_0357;
    }
);
mod section_0358_to_0365;
mod section_0366_to_0379;
mod section_0380;
mod section_0381_to_0401;
mod section_0402_to_0463;
mod section_0464_to_0486;
mod section_0487_to_0510;
mod section_0511_to_0536;
reversing_order_items!(
    {
        mod section_0537;
    }
    {
        #[macro_use]
        mod section_0538;
    }
);
mod section_0539_to_0582;
mod section_0583_to_0591;
mod section_0592_to_0599;
mod section_0600_to_0643;
mod section_0644_to_0679;
mod section_0680_to_0698;
mod section_0699_to_0718;
mod section_0719_to_0767;
mod section_0768_to_0812;
mod section_0813_to_0861;
mod section_0862_to_0890;
mod section_0891_to_0899;
mod section_0900_to_0918;
mod section_0919_to_0941;
mod section_0942_to_0966;
mod section_0967_to_0979;
mod section_0980_to_1028;
mod section_1029;
mod section_1030;
mod section_1031_to_1054;
mod section_1055_to_1135;
mod section_1136_to_1198;
mod section_1199_to_1207;
mod section_1208_to_1298;
mod section_1299;
mod section_1300_to_1329;
mod section_1330;
mod section_1331;
reversing_order_items!(
{
    mod section_1332;
    mod section_1333;
    mod section_1334;
    mod section_1335;
    mod section_1336;
}
{
    #[macro_use]
    mod section_1337;
}
);
mod section_1338_to_1339;
mod section_1340_to_1377;
mod section_1378;
mod section_1379;
mod section_1380;

pub use section_1332::entry;
