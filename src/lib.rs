// Copyright 2020 TeX-rs Author(s)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// ------------------------OR----------------------------
// Copyright 2020 TeX-rs Author(s)
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

// -------------------------------------------------------
// `TeX-rs` is a port of the `TeX` program to Rust.
// The original `TeX` program is copyright (C) 1982
// by D. E. Knuth; all rights are reserved.

// TeX is a trademark of the American Mathematical Society.
// METAFONT is a trademark of Addison-Wesley Publishing Company.

// Notes from the original author:
//
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
reversing_order_items!(
    {
        #[macro_use]
        mod section_0004;
        mod section_0005;
        mod section_0006;
        mod section_0007;
    }
    {
        #[macro_use]
        mod section_0008;
        mod section_0009_to_0010;
        mod section_0011;
        mod section_0012;
        mod section_0013;
        mod section_0014;
        mod section_0015;
        #[macro_use]
        mod section_0016;
        mod section_0017;
    }
    {
        #[macro_use]
        mod section_0018;
    }
);
mod section_0019;
mod section_0020;
mod section_0021;
mod section_0022;
mod section_0023;
mod section_0024;
mod section_0025;
mod section_0026;
mod section_0027;
mod section_0028;
mod section_0029;
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
mod section_0039;
mod section_0040;
#[macro_use]
mod section_0041;
mod section_0042;
mod section_0043;
mod section_0044;
mod section_0045;
mod section_0046;
reversing_order_items!(
    {
        mod section_0047;
    }
    {
        #[macro_use]
        mod section_0048;
    }
    {
        #[macro_use]
        mod section_0049;
        mod section_0050;
        #[macro_use]
        mod section_0051;
    }
    {
        #[macro_use]
        mod section_0052;
    }
    {
        #[macro_use]
        mod section_0053;
    }
);
mod section_0054;
#[macro_use]
mod section_0055;
mod section_0056;
mod section_0057;
mod section_0058;
mod section_0059;
mod section_0060;
#[macro_use]
mod section_0061;
mod section_0062;
mod section_0063_to_0070;
reversing_order_items!(
    {
        #[macro_use]
        mod section_0071;
        mod section_0072;
        #[macro_use]
        mod section_0073;
        mod section_0074;
        mod section_0075;
        mod section_0076;
        mod section_0077_to_0080;
    }
    {
        #[macro_use]
        mod section_0081;
    }
);
mod section_0082_to_0086;
mod section_0087;
mod section_0088_to_0092;
mod section_0093;
mod section_0094;
mod section_0095;
#[macro_use]
mod section_0096;
mod section_0097;
mod section_0098;
mod section_0099_to_0100;
mod section_0101;
mod section_0102_to_0108;
mod section_0109;
mod section_0110;
mod section_0111;
#[macro_use]
mod section_0112;
mod section_0113;
mod section_0114;
mod section_0115;
mod section_0116;
mod section_0117_to_0132;
mod section_0133;
mod section_0134;
mod section_0135;
mod section_0136_to_0161;
mod section_0162;
mod section_0163_to_0164;
mod section_0165;
mod section_0166;
mod section_0167;
mod section_0168_to_0172;
mod section_0173_to_0198;
mod section_0199_to_0202;
mod section_0203_to_0206;
mod section_0207;
mod section_0208;
mod section_0209;
mod section_0210;
mod section_0211;
mod section_0212;
#[macro_use]
mod section_0213;
mod section_0214;
reversing_order_items!(
    {
        mod section_0215;
    }
    {
        mod section_0216_to_0219;
        mod section_0220;
        #[macro_use]
        mod section_0221;
        mod section_0222;
        mod section_0223;
        mod section_0224;
        mod section_0225_to_0229;
        #[macro_use]
        mod section_0230;
        mod section_0231;
        mod section_0232;
        mod section_0233_to_0235;
        #[macro_use]
        mod section_0236;
        mod section_0237_to_0239;
        mod section_0240;
        mod section_0241;
        mod section_0242_to_0246;
        mod section_0247;
        mod section_0248_to_0252;
        mod section_0253;
        mod section_0254;
        mod section_0255;
        #[macro_use]
        mod section_0256;
        mod section_0257_to_0258;
        reversing_order_items!(
            {
                mod section_0259;
            }
            {
                #[macro_use]
                mod section_0260;
            }
            {
                #[macro_use]
                mod section_0261;
            }
        );
        mod section_0262;
        mod section_0263;
        mod section_0264;
        mod section_0265;
        mod section_0266;
        mod section_0267;
        mod section_0268_to_0270;
        mod section_0271;
        mod section_0272;
        mod section_0273;
        mod section_0274;
        mod section_0275;
        mod section_0276;
        mod section_0277;
        mod section_0278;
        mod section_0279;
        mod section_0280_to_0288;
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
        #[macro_use]
        mod section_0307;
        mod section_0308;
        mod section_0309;
        mod section_0310;
        mod section_0311_to_0320;
        #[macro_use]
        mod section_0321;
        #[macro_use]
        mod section_0322;
        mod section_0323;
        mod section_0324;
        mod section_0325;
        mod section_0326_to_0327;
        mod section_0328;
        mod section_0329;
        mod section_0330;
        #[macro_use]
        mod section_0331;
        mod section_0332;
        mod section_0333;
        mod section_0334;
        mod section_0335;
        mod section_0336;
        mod section_0337_to_0340;
        reversing_order_items!(
            {
                mod section_0341;
            }
            {
                #[macro_use]
                mod section_0342;
            }
            {
                #[macro_use]
                mod section_0343;
            }
            {
                #[macro_use]
                mod section_0344;
            }
            {
                #[macro_use]
                mod section_0345;
            }
            {
                mod section_0346;
                #[macro_use]
                mod section_0347;
            }
            {
                #[macro_use]
                mod section_0348;
                #[macro_use]
                mod section_0349;
                #[macro_use]
                mod section_0350;
                #[macro_use]
                mod section_0351;
                mod section_0352_to_0353;
                #[macro_use]
                mod section_0354;
                #[macro_use]
                mod section_0355;
            }
            {
                #[macro_use]
                mod section_0356;
            }
            {
                #[macro_use]
                mod section_0357;
            }
            {
                mod section_0358_to_0359;
                #[macro_use]
                mod section_0360;
            }
            {
                mod section_0361;
                #[macro_use]
                mod section_0362;
            }
        );

        mod section_0363;
        mod section_0364;
        mod section_0365;
        reversing_order_items! (
            {
                mod section_0366;
            }
            {
                #[macro_use]
                mod section_0367;
            }
            {
                mod section_0368_to_0369;
                #[macro_use]
                mod section_0370;
            }
        );
        mod section_0371_to_0379;
        mod section_0380;
        mod section_0381_to_0401;
        mod section_0402_to_0403;
        #[macro_use]
        mod section_0404;
        reversing_order_items! (
            {
                mod section_0405;
            }
            {
                #[macro_use]
                mod section_0406;
            }
        );
        mod section_0407_to_0409;
        mod section_0410;
        mod section_0411;
        mod section_0412;
        mod section_0413;
        mod section_0414;
        mod section_0415_to_0433;
        mod section_0434;
        mod section_0435_to_0437;
        mod section_0438;
        mod section_0439;
        reversing_order_items! (
            {
                mod section_0440;
            }
            {
                #[macro_use]
                mod section_0441;
            }
            {
                #[macro_use]
                mod section_0442;        
            }
            {
                mod section_0443;
                #[macro_use]
                mod section_0444; 
            }
        );
        mod section_0445_to_0463;
        mod section_0464_to_0486;
        mod section_0487_to_0510;
        mod section_0511;
        mod section_0512;
        mod section_0513;
        mod section_0514;
        mod section_0515;
        mod section_0516;
        mod section_0517;
        mod section_0518;
        mod section_0519;
        mod section_0520_to_0524;
        mod section_0525;
        mod section_0526;
        mod section_0527;
        mod section_0528;
        mod section_0529;
        reversing_order_items!(
            {
                mod section_0530;
            }
            {
                #[macro_use]
                mod section_0531;
            }
        );
        mod section_0532_to_0536;
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
        mod section_0583;
        mod section_0584;
        mod section_0585;
        mod section_0586;
        mod section_0587;
        mod section_0588;
        mod section_0589;
        mod section_0590;
        mod section_0591;
        mod section_0592_to_0599;
        mod section_0600_to_0643;
        mod section_0644_to_0679;
        mod section_0680_to_0698;
        mod section_0699_to_0718;
        mod section_0719_to_0767;
        mod section_0768;
    }
    {
        #[macro_use]
        mod section_0769;
        mod section_0770;
        mod section_0771_to_0788;
        #[macro_use]
        mod section_0789;
    }
    {
        mod section_0790_to_0812;
        mod section_0813_to_0861;
        mod section_0862_to_0890;
        mod section_0891_to_0899;
        mod section_0900_to_0918;
        mod section_0919_to_0941;
        mod section_0942_to_0966;
        mod section_0967_to_0979;
        mod section_0980_to_0993;
        mod section_0994;
    }
    {
        #[macro_use]
        mod section_0995;
    }
);
mod section_0996_to_1028;
mod section_1029;
reversing_order_items!(
    {
        mod section_1030;
    }
    {
        #[macro_use]
        mod section_1031;
    }
    {
        reversing_order_items!(
            {
                mod section_1032_to_1044;
                #[macro_use]
                mod section_1045;
            }
            {
                #[macro_use]
                mod section_1046;
            }
        );
    }
    {
        mod section_1047;
        #[macro_use]
        mod section_1048;
    }
    {
        mod section_1049;
        mod section_1050;
        mod section_1051;
        mod section_1052;
        mod section_1053;
        mod section_1054;
        mod section_1055_to_1069;
        mod section_1070;
        mod section_1071_to_1089;
        #[macro_use]
        mod section_1090;
        mod section_1091;
        mod section_1092_to_1093;
        #[macro_use]
        mod section_1094;
    }
    {
        mod section_1095_to_1135;
        mod section_1136_to_1198;
        mod section_1199_to_1207;
        mod section_1208;
        mod section_1209;
        #[macro_use]
        mod section_1210;
    }
    {
        mod section_1211;
        mod section_1212;
        mod section_1213;
    }
    {
        #[macro_use]
        mod section_1214;
        mod section_1215_to_1229;
        mod section_1230;
        mod section_1231;
    }
    {
        #[macro_use]
        mod section_1232;
    }
    {
        #[macro_use]
        mod section_1233;
    }
    {
        mod section_1234_to_1275;
        #[macro_use]
        mod section_1276;
    }
);
mod section_1277;
mod section_1278;
mod section_1279;
mod section_1280_to_1298;
mod section_1299;
mod section_1300;
mod section_1301;
mod section_1302_to_1329;
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

mod string_pool;
mod unicode_support;

pub use section_0004::TeXGlobals;
pub use section_1332::entry;
