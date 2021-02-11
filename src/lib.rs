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
// `TeX-rs` originates from the `TeX` program, which was
// created by Donald Knuth and released under his usual license:
// http://www.ctan.org/license/knuth.

// TeX is a trademark of the American Mathematical Society.
// METAFONT is a trademark of Addison-Wesley Publishing Company.

#![deny(warnings, missing_docs, missing_debug_implementations)]
#![allow(
    dead_code,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports,
    unused_doc_comments
)]
#![allow(unreachable_code)]
//! This is `TeX`-rs, a document compiler intended to produce typesetting of high quality.

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
        reversing_order_items!(
            {
                #[macro_use]
                mod section_0048;
            }
            {
                #[macro_use]
                mod section_0049;
            }
        );
        mod section_0050;
        reversing_order_items!(
            {
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
    }
);
mod section_0054;
#[macro_use]
mod section_0055;
mod section_0056;
mod section_0057;
macro_rules! forward_mod_a {
    () => {
        mod section_0058;
        mod section_0059;
    }
}
mod section_0060;
#[macro_use]
mod section_0061;
mod section_0062;
macro_rules! forward_mod_b {
    () => {
        mod section_0063;
    }
}
mod section_0064;
mod section_0065;
mod section_0066;
mod section_0067;
mod section_0068;
mod section_0069;
mod section_0070;
#[macro_use]
mod section_0071;
reversing_order_items!(
    {
        mod section_0072;
        #[macro_use]
        mod section_0073;
        mod section_0074;
        #[macro_use]
        mod section_0075;
        mod section_0076;
        mod section_0077;
        mod section_0078;
        #[macro_use]
        mod section_0079;
        mod section_0080;
    }
    {
        reversing_order_items!(
            {
                #[macro_use]
                mod section_0081;
            }
            {
                mod section_0082;
            }
            {
                #[macro_use]
                mod section_0083;
            }
            {
                #[macro_use]
                mod section_0084;
                mod section_0085_to_0086;
                mod section_0087;
                mod section_0088;
                mod section_0089;
                #[macro_use]
                mod section_0090;
            }
        );
    }
);
mod section_0091;
mod section_0092;
mod section_0093;
mod section_0094;
mod section_0095;
#[macro_use]
mod section_0096;
mod section_0097;
mod section_0098;
mod section_0099_to_0100;
mod section_0101;
mod section_0102;
mod section_0103;
mod section_0104;
#[macro_use]
mod section_0105;
mod section_0106;
mod section_0107;
mod section_0108;
#[macro_use]
mod section_0109;
mod section_0110;
mod section_0111;
#[macro_use]
mod section_0112;
mod section_0113;
mod section_0114;
mod section_0115;
mod section_0116;
mod section_0117;
#[macro_use]
mod section_0118;
mod section_0119;
mod section_0120;
#[macro_use]
mod section_0121;
#[macro_use]
mod section_0122;
mod section_0123;
#[macro_use]
mod section_0124;
reversing_order_items!(
    {
        mod section_0125;
    }
    {
        #[macro_use]
        mod section_0126;
        #[macro_use]
        mod section_0127;
    }
    {
        #[macro_use]
        mod section_0128;
    }
);
mod section_0129;
mod section_0130;
mod section_0131_to_0132;
#[macro_use]
mod section_0133;
#[macro_use]
mod section_0134;
#[macro_use]
mod section_0135;
mod section_0136;
mod section_0137;
mod section_0138;
mod section_0139;
mod section_0140;
mod section_0141;
mod section_0142;
#[macro_use]
mod section_0143;
mod section_0144;
mod section_0145;
mod section_0146;
mod section_0147;
mod section_0148;
#[macro_use]
mod section_0149;
#[macro_use]
mod section_0150;
mod section_0151;
macro_rules! forward_mod_c {
    () => {
        mod section_0152;
    }
}
mod section_0153;
mod section_0154;
mod section_0155;
mod section_0156;
#[macro_use]
mod section_0157;
mod section_0158;
mod section_0159;
mod section_0160;
mod section_0161;
mod section_0162;
mod section_0163;
macro_rules! forward_mod_d {
    () => {
        mod section_0164;
    }
}
mod section_0165;
mod section_0166;
reversing_order_items!(
    {
        mod section_0167;
    }
    {
        #[macro_use]
        mod section_0168;
        #[macro_use]
        mod section_0169;
        #[macro_use]
        mod section_0170;
        mod section_0171_to_0172;
    }
);
mod section_0173;
reversing_order_items!(
    {
        mod section_0174;
        mod section_0175;
        mod section_0176;
    }
    {
        mod section_0177;
        mod section_0178;
        mod section_0179;
        #[macro_use]
        mod section_0180;
        mod section_0181;
        macro_rules! forward_mod_e {
            () => {
                reversing_order_items!(
                    {
                        mod section_0182;
                    }
                    {
                        #[macro_use]
                        mod section_0183;
                    }
                );
            }
        }
        reversing_order_items!(
            {
                #[macro_use]
                mod section_0184;
            }
            {
                mod section_0185;
                #[macro_use]
                mod section_0186;
            }
        );
        mod section_0187_to_0188;
        #[macro_use]
        mod section_0189;
        mod section_0190;
        #[macro_use]
        mod section_0191;
        mod section_0192;
        #[macro_use]
        mod section_0193;
        mod section_0194_to_0197;
        reversing_order_items!(
            {
                mod section_0198;
            }
            {
                mod section_0199;
                #[macro_use]
                mod section_0200;
                #[macro_use]
                mod section_0201;
                macro_rules! forward_mod_f {
                    () => {
                        mod section_0202;
                    }
                }
                #[macro_use]
                mod section_0203;
                mod section_0204;
                mod section_0205_to_0206;
                mod section_0207;
                mod section_0208;
                mod section_0209;
                mod section_0210;
                mod section_0211;
                mod section_0212;
                #[macro_use]
                mod section_0213;
                #[macro_use]
                mod section_0214;
                macro_rules! forward_mod_g {
                    () => {
                        mod section_0215;
                    }
                }
                mod section_0216;
                mod section_0217;
                macro_rules! forward_mod_h {
                    () => {
                        mod section_0218;
                    }
                }
                #[macro_use]
                mod section_0219;
                mod section_0220;
                #[macro_use]
                mod section_0221;
                mod section_0222;
                mod section_0223;
                #[macro_use]
                mod section_0224;
                forward_mod_c!();
                mod section_0225;
                mod section_0226;
                #[macro_use]
                mod section_0227;
                mod section_0228_to_0229;
                #[macro_use]
                mod section_0230;
                #[macro_use]
                mod section_0231;
                mod section_0232;
                mod section_0233_to_0235;
                #[macro_use]
                mod section_0236;
            }
        );
        mod section_0237;
        mod section_0238;
        #[macro_use]
        mod section_0239;
        mod section_0240;
        mod section_0241;
        mod section_0242;
        #[macro_use]
        mod section_0243;
        forward_mod_b!();
        #[macro_use]
        mod section_0244;
        forward_mod_a!();
        mod section_0245;
        mod section_0246;
        #[macro_use]
        mod section_0247;
        mod section_0248;
        #[macro_use]
        mod section_0249;
        mod section_0250_to_0252;
        mod section_0253;
        mod section_0254;
        mod section_0255;
        #[macro_use]
        mod section_0256;
        mod section_0257;
        mod section_0258;
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
        #[macro_use]
        mod section_0266;
        #[macro_use]
        mod section_0267;
    }
);

reversing_order_items!({
    #[macro_use]
    mod section_0268;
    mod section_0269;
    mod section_0270;
    mod section_0271;
    mod section_0272;
    #[macro_use]
    mod section_0273;
    #[macro_use]
    mod section_0274;
    mod section_0275;
    mod section_0276;
    mod section_0277;
    mod section_0278;
    mod section_0279;
    mod section_0280;
    reversing_order_items!(
        {
            mod section_0281;
        }
        {
            #[macro_use]
            mod section_0282;
        }
        {
            #[macro_use]
            mod section_0283;
        }
    );
    mod section_0284_to_0288;
    mod section_0289;
    mod section_0290;
    mod section_0291;
    reversing_order_items!(
        {
            mod section_0292;
        }
        {
            #[macro_use]
            mod section_0293;
            #[macro_use]
            mod section_0294;
        }
        {
            mod section_0295;
            mod section_0296;
            mod section_0297;
            macro_rules! forward_mod_i {
                () => {
                    mod section_0298;
                }
            }
            mod section_0299;
            mod section_0300;
            mod section_0301;
            #[macro_use]
            mod section_0302;
            mod section_0303;
            #[macro_use]
            mod section_0304;
            mod section_0305;
            mod section_0306;
            #[macro_use]
            mod section_0307;
            mod section_0308;
            mod section_0309;
            mod section_0310;
            reversing_order_items!(
                {
                    mod section_0311;
                }
                {
                    #[macro_use]
                    mod section_0312;
                }
                {
                    #[macro_use]
                    mod section_0313;
                    #[macro_use]
                    mod section_0314;
                    mod section_0315;
                    #[macro_use]
                    mod section_0316;
                    #[macro_use]
                    mod section_0317;
                    #[macro_use]
                    mod section_0318;
                    #[macro_use]
                    mod section_0319;
                }
            );
            #[macro_use]
            mod section_0320;
        }
    );
    reversing_order_items!(
        {
            #[macro_use]
            mod section_0321;
            #[macro_use]
            mod section_0322;
            #[macro_use]
            mod section_0323;
            mod section_0324;
            mod section_0325;
            mod section_0326;
            mod section_0327;
            mod section_0328;
            mod section_0329;
            mod section_0330;
            #[macro_use]
            mod section_0331;
            mod section_0332;
            mod section_0333;
            mod section_0334;
            #[macro_use]
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
                    reversing_order_items!(
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
                            #[macro_use]
                            mod section_0352;
                            #[macro_use]
                            mod section_0353;
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
                            #[macro_use]
                            mod section_0358;
                            #[macro_use]
                            mod section_0359;
                        }
                        {
                            #[macro_use]
                            mod section_0360;
                        }
                        {
                            mod section_0361;
                            #[macro_use]
                            mod section_0362;
                        }
                    );
                    reversing_order_items!(
                        {
                            mod section_0363;
                            mod section_0364;
                            mod section_0365;
                        }
                        {
                            reversing_order_items! (
                                {
                                    mod section_0366;
                                }
                                {
                                    #[macro_use]
                                    mod section_0367;
                                }
                            );
                        }
                        {
                            reversing_order_items! (
                                {
                                    #[macro_use]
                                    mod section_0368;
                                    #[macro_use]
                                    mod section_0369;
                                    #[macro_use]
                                    mod section_0370;
                                }
                            );
                            #[macro_use]
                            mod section_0371;
                            reversing_order_items! (
                                {
                                    #[macro_use]
                                    mod section_0372;
                                }
                                {
                                    mod section_0373;
                                    #[macro_use]
                                    mod section_0374;
                                }
                            );
                            mod section_0375;
                            mod section_0376;
                            #[macro_use]
                            mod section_0377;
                            #[macro_use]
                            mod section_0378;
                            mod section_0379;
                            mod section_0380;
                            mod section_0381;
                            mod section_0382_to_0386;
                            mod section_0387;
                            mod section_0388;
                            reversing_order_items! (
                                {
                                    mod section_0389;
                                }
                                {
                                    #[macro_use]
                                    mod section_0390;
                                }
                                {
                                    #[macro_use]
                                    mod section_0391;
                                    #[macro_use]
                                    mod section_0392;
                                    #[macro_use]
                                    mod section_0393;
                                }
                                {
                                    #[macro_use]
                                    mod section_0394;
                                    mod section_0395_to_0396;
                                    #[macro_use]
                                    mod section_0397;
                                }
                                {
                                    #[macro_use]
                                    mod section_0398;
                                    #[macro_use]
                                    mod section_0399;
                                    #[macro_use]
                                    mod section_0400;
                                    #[macro_use]
                                    mod section_0401;
                                }
                            );
                            mod section_0402;
                            reversing_order_items! (
                                {
                                    mod section_0403;
                                }
                                {
                                    #[macro_use]
                                    mod section_0404;
                                }
                                {
                                    mod section_0405;
                                }
                                {
                                    #[macro_use]
                                    mod section_0406;
                                }
                                {
                                    mod section_0407;
                                    mod section_0408;
                                    mod section_0409;
                                    mod section_0410;
                                    mod section_0411;
                                    #[macro_use]
                                    mod section_0412;
                                    mod section_0413;
                                }
                                {
                                    #[macro_use]
                                    mod section_0414;
                                    #[macro_use]
                                    mod section_0415;
                                    mod section_0416;
                                    #[macro_use]
                                    mod section_0417;
                                    mod section_0418;
                                    mod section_0419;
                                    #[macro_use]
                                    mod section_0420;
                                    #[macro_use]
                                    mod section_0421;
                                    mod section_0422_to_0423;
                                    #[macro_use]
                                    mod section_0424;
                                    #[macro_use]
                                    mod section_0425;
                                    #[macro_use]
                                    mod section_0426;
                                    #[macro_use]
                                    mod section_0427;
                                }
                                {
                                    #[macro_use]
                                    mod section_0428;
                                    #[macro_use]
                                    mod section_0429;
                                    #[macro_use]
                                    mod section_0430;
                                }
                            );
                            mod section_0431;
                            mod section_0432;
                            mod section_0433;
                            mod section_0434;
                            mod section_0435;
                            mod section_0436;
                            mod section_0437;
                            mod section_0438;
                            mod section_0439;
                            reversing_order_items! (
                                {
                                    mod section_0440;
                                }
                                {
                                    #[macro_use]
                                    mod section_0441;
                                    #[macro_use]
                                    mod section_0442;
                                    #[macro_use]
                                    mod section_0443;
                                    reversing_order_items! (
                                        {
                                            #[macro_use]
                                            mod section_0444;
                                        }
                                        {
                                            #[macro_use]
                                            mod section_0445;
                                            #[macro_use]
                                            mod section_0446;
                                        }
                                    );
                                }
                            );
                            mod section_0447;
                            reversing_order_items! (
                                {
                                    #[macro_use]
                                    mod section_0448;
                                }
                                {
                                    #[macro_use]
                                    mod section_0449;
                                    mod section_0450;
                                    mod section_0451;
                                    #[macro_use]
                                    mod section_0452;
                                    macro_rules! forward_mod_j {
                                        () => {
                                            #[macro_use]
                                            mod section_0453;
                                        }
                                    }
                                    #[macro_use]
                                    mod section_0454;
                                    #[macro_use]
                                    mod section_0455;
                                    #[macro_use]
                                    mod section_0456;
                                    mod section_0457;
                                    #[macro_use]
                                    mod section_0458;
                                    forward_mod_j!();
                                }
                                {
                                    #[macro_use]
                                    mod section_0459;
                                }
                            );
                            reversing_order_items! (
                                {
                                   mod section_0460;
                                    reversing_order_items!(
                                        {
                                            mod section_0461;
                                        }
                                        {
                                            #[macro_use]
                                            mod section_0462;
                                            mod section_0463;
                                            mod section_0464;
                                            mod section_0465;
                                        }
                                        {
                                            #[macro_use]
                                            mod section_0466;
                                        }
                                    );
                                    mod section_0467;
                                    mod section_0468;
                                    #[macro_use]
                                    mod section_0469;
                                    reversing_order_items!(
                                        {
                                            mod section_0470;
                                        }
                                        {
                                            #[macro_use]
                                            mod section_0471;
                                            #[macro_use]
                                            mod section_0472;
                                        }
                                    );
                                    mod section_0473;
                                }
                                {
                                    #[macro_use]
                                    mod section_0474;
                                    mod section_0475;
                                    #[macro_use]
                                    mod section_0476;
                                    #[macro_use]
                                    mod section_0477;
                                }
                                {
                                    #[macro_use]
                                    mod section_0478;
                                }
                                {
                                    #[macro_use]
                                    mod section_0479;
                                }
                            );
                            mod section_0480;
                            mod section_0481;
                            reversing_order_items! (
                                {
                                    mod section_0482;
                                }
                                {
                                    #[macro_use]
                                    mod section_0483;
                                }
                                {
                                    mod section_0484;
                                    #[macro_use]
                                    mod section_0485;
                                    #[macro_use]
                                    mod section_0486;
                                }
                            );
                            mod section_0487;
                            #[macro_use]
                            mod section_0488;
                            #[macro_use]
                            mod section_0489;
                            mod section_0490;
                            mod section_0491;
                            #[macro_use]
                            mod section_0492;
                            mod section_0493;
                            mod section_0494;
                            #[macro_use]
                            mod section_0495;
                            #[macro_use]
                            mod section_0496;
                            mod section_0497;
                            reversing_order_items! (
                                {
                                    mod section_0498;
                                }
                                {
                                    mod section_0499;
                                    #[macro_use]
                                    mod section_0500;
                                    reversing_order_items! (
                                        {
                                            #[macro_use]
                                            mod section_0501;
                                        }
                                        {
                                            #[macro_use]
                                            mod section_0502;
                                            #[macro_use]
                                            mod section_0503;
                                            mod section_0504_to_0505;
                                            #[macro_use]
                                            mod section_0506;
                                            #[macro_use]
                                            mod section_0507;
                                        }
                                        {
                                            #[macro_use]
                                            mod section_0508;
                                            #[macro_use]
                                            mod section_0509;
                                        }
                                    );
                                }
                            );
                            #[macro_use]
                            mod section_0510;
                        }
                    );
                    mod section_0511;
                    mod section_0512;
                    mod section_0513;
                    #[macro_use]
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
                    mod section_0532;
                    mod section_0533;
                    reversing_order_items!(
                        {
                            mod section_0534;
                        }
                        {
                            mod section_0535;
                            #[macro_use]
                            mod section_0536;
                        }
                    );
                    reversing_order_items!(
                        {
                            mod section_0537;
                        }
                        {
                            #[macro_use]
                            mod section_0538;
                        }
                    );
        
                    mod section_0539;
                    mod section_0540_to_0543;
                    mod section_0544;
                    mod section_0545;
                    mod section_0546;
                    mod section_0547;
                    mod section_0548;
                    #[macro_use]
                    mod section_0549;
                    mod section_0550;
                    mod section_0551;
                    mod section_0552;
                    mod section_0553;
                    #[macro_use]
                    mod section_0554;
                    mod section_0555_to_0556;
                    #[macro_use]
                    mod section_0557;
                    #[macro_use]
                    mod section_0558;
                    mod section_0559;
                    reversing_order_items!(
                        {
                            mod section_0560;
                        }
                        {
                            #[macro_use]
                            mod section_0561;
                            #[macro_use]
                            mod section_0562;
                        }
                        {
                            #[macro_use]
                            mod section_0563;
                        }
                        {
                            #[macro_use]
                            mod section_0564;
                            #[macro_use]
                            mod section_0565;
                        }
                        {
                            #[macro_use]
                            mod section_0566;
                            mod section_0567;
                            #[macro_use]
                            mod section_0568;
                            #[macro_use]
                            mod section_0569;
                            #[macro_use]
                            mod section_0570;
                            #[macro_use]
                            mod section_0571;
                            #[macro_use]
                            mod section_0572;
                            #[macro_use]
                            mod section_0573;
                            #[macro_use]
                            mod section_0574;
                            #[macro_use]
                            mod section_0575;
                            #[macro_use]
                            mod section_0576;
                            mod section_0577;
                            mod section_0578;
                        }
                        {
                            #[macro_use]
                            mod section_0579;
                            #[macro_use]
                            mod section_0580;
                        }
                    );
                    mod section_0581;
                    mod section_0582;
                    mod section_0583;
                    mod section_0584;
                    mod section_0585;
                    mod section_0586;
                    mod section_0587;
                    mod section_0588;
                    mod section_0589;
                    mod section_0590;
                    mod section_0591;
                    mod section_0592;
                    mod section_0593_to_0599;
                    mod section_0600_to_0641;
                    #[macro_use]
                    mod section_0642;
                    mod section_0643;
                    mod section_0644;
                    mod section_0645;
                    mod section_0646;
                    mod section_0647;
                    mod section_0648;
                    reversing_order_items!(
                        {
                            mod section_0649;
                        }
                        {
                            #[macro_use]
                            mod section_0650;
                            #[macro_use]
                            mod section_0651;
                        }
                        {
                            #[macro_use]
                            mod section_0652;
                            #[macro_use]
                            mod section_0653;
                            #[macro_use]
                            mod section_0654;
                            mod section_0655;
                            #[macro_use]
                            mod section_0656;
                            #[macro_use]
                            mod section_0657;
                            mod section_0658_to_0660;
                            mod section_0661;
                            mod section_0662;
                            #[macro_use]
                            mod section_0663;
                        }
                    );
                    mod section_0664_to_0667;
                    reversing_order_items!(
                        {
                            mod section_0668;
                        }
                        {
                            #[macro_use]
                            mod section_0669;
                        }
                        {
                            #[macro_use]
                            mod section_0670;
                            #[macro_use]
                            mod section_0671;
                            #[macro_use]
                            mod section_0672;
                            mod section_0673_to_0674;
                            #[macro_use]
                            mod section_0675;
                        }
                    );
                    mod section_0676_to_0679;
                    mod section_0680_to_0698;
                    mod section_0699;
                    mod section_0700_to_0718;
                    mod section_0719_to_0767;
                    mod section_0768;
                    #[macro_use]
                    mod section_0769;
                    #[macro_use]
                    mod section_0770;
                    mod section_0771;
                    mod section_0772;
                    mod section_0773;
                    reversing_order_items!(
                        {
                            mod section_0774;
                        }
                        {
                            #[macro_use]
                            mod section_0775;
                            #[macro_use]
                            mod section_0776;
                            #[macro_use]
                            mod section_0777;
                        }
                        {
                            #[macro_use]
                            mod section_0778;
                            #[macro_use]
                            mod section_0779;
                        }
                        {
                            mod section_0780;
                            #[macro_use]
                            mod section_0781;
                            mod section_0782;
                            #[macro_use]
                            mod section_0783;
                            #[macro_use]
                            mod section_0784;
                        }
                    );
                    mod section_0785;
                    mod section_0786;
                    mod section_0787;
                    mod section_0788;
                    #[macro_use]
                    mod section_0789;
                }
            );
        }
        {
            mod section_0790;
            mod section_0791;
            mod section_0792_to_0798;
            mod section_0799;
            mod section_0800_to_0812;
            mod section_0813;
            mod section_0814;
            reversing_order_items! (
                {
                    mod section_0815;
                }
                {
                    mod section_0816;
                    mod section_0817;
                    mod section_0818;
                    #[macro_use]
                    mod section_0819;
                    #[macro_use]
                    mod section_0820;
                    #[macro_use]
                    mod section_0821;
                    mod section_0822;
                    #[macro_use]
                    mod section_0823;
                    mod section_0824_to_0827;
                    mod section_0828;
                    reversing_order_items!(
                        {
                            mod section_0829;
                        }
                        {
                            mod section_0830;
                            #[macro_use]
                            mod section_0831;
                            #[macro_use]
                            mod section_0832;
                            mod section_0833;
                            mod section_0834;
                            #[macro_use]
                            mod section_0835;
                        }
                        {
                            #[macro_use]
                            mod section_0836;
                            #[macro_use]
                            mod section_0837;
                            mod section_0838_to_0842;
                            #[macro_use]
                            mod section_0843;
                            #[macro_use]
                            mod section_0844;
                            #[macro_use]
                            mod section_0845;
                            mod section_0846;
                            mod section_0847;
                            #[macro_use]
                            mod section_0848;
                            mod section_0849;
                            #[macro_use]
                            mod section_0850;
                            #[macro_use]
                            mod section_0851;
                        }
                        {
                            #[macro_use]
                            mod section_0852;
                            #[macro_use]
                            mod section_0853;
                            #[macro_use]
                            mod section_0854;
                            #[macro_use]
                            mod section_0855;
                        }
                        {
                            mod section_0856_to_0858;
                            #[macro_use]
                            mod section_0859;
                            #[macro_use]
                            mod section_0860;
                        }
                        {
                            #[macro_use]
                            mod section_0861;
                        }
                    );
                    mod section_0862;
                    reversing_order_items!(
                        {
                            #[macro_use]
                            mod section_0863;
                        }
                        {
                            #[macro_use]
                            mod section_0864;
                            #[macro_use]
                            mod section_0865;
                            mod section_0866;
                            mod section_0867_to_0871;
                            mod section_0872;
                            reversing_order_items! (
                                {
                                    #[macro_use]
                                    mod section_0873;
                                }
                                {
                                    #[macro_use]
                                    mod section_0874;
                                }
                            );
                            #[macro_use]
                            mod section_0875;
                            #[macro_use]
                            mod section_0876;
                            mod section_0877;
                            mod section_0878_to_0890;
                            #[macro_use]
                            mod section_0891;
                        }
                    );
                }
            );
            mod section_0892;
            mod section_0893_to_0899;
            mod section_0900_to_0906;
            mod section_0907;
            mod section_0908_to_0918;
            mod section_0919;
            mod section_0920;
            #[macro_use]
            mod section_0921;
            mod section_0922_to_0924;
            mod section_0925;
            mod section_0926;
            mod section_0927_to_0933;
            reversing_order_items!(
                {
                    #[macro_use]
                    mod section_0934;
                }
                {
                    #[macro_use]
                    mod section_0935;
                }
                {
                    mod section_0936;
                    #[macro_use]
                    mod section_0937;
                }
                {
                    #[macro_use]
                    mod section_0938;
                }
                {
                    #[macro_use]
                    mod section_0939;
                }
                {
                    #[macro_use]
                    mod section_0940;
                }
            );
            mod section_0941;
            mod section_0942;
            #[macro_use]
            mod section_0943;
            mod section_0944;
            #[macro_use]
            mod section_0945;
            mod section_0946;
            #[macro_use]
            mod section_0947;
            mod section_0948;
            mod section_0949;
            #[macro_use]
            mod section_0950;
            mod section_0951;
            #[macro_use]
            mod section_0952;
            reversing_order_items!(
                {
                    mod section_0953;
                }
                {
                    #[macro_use]
                    mod section_0954;
                    #[macro_use]
                    mod section_0955;
                    #[macro_use]
                    mod section_0956;
                }
            );
            mod section_0957;
            mod section_0958;
            mod section_0959;
            reversing_order_items!(
                {
                    mod section_0960;
                }
                {
                    #[macro_use]
                    mod section_0961;
                }
                {
                    #[macro_use]
                    mod section_0962;
                    #[macro_use]
                    mod section_0963;
                    #[macro_use]
                    mod section_0964;
                    #[macro_use]
                    mod section_0965;      
                }
            );
            mod section_0966;
            mod section_0967_to_0979;
            mod section_0980;
            mod section_0981;
            forward_mod_d!();
            #[macro_use]
            mod section_0982;
            mod section_0983;
            #[macro_use]
            mod section_0984;
            mod section_0985;
            #[macro_use]
            mod section_0986;
            forward_mod_h!();
            mod section_0987;
            mod section_0988;
            mod section_0989;
            mod section_0990;
            #[macro_use]
            mod section_0991;
            mod section_0992_to_0993;
            mod section_0994;
            #[macro_use]
            mod section_0995;
            forward_mod_g!();
        }
    );
});

mod section_0996_to_1028;
mod section_1029;

reversing_order_items!(
    {
        mod section_1030;
    }
    {
        #[macro_use]
        mod section_1031;
        mod section_1032;
        mod section_1033;
        reversing_order_items!(
            {
                #[macro_use]
                mod section_1034;
            }
            {
                #[macro_use]
                mod section_1035;
                #[macro_use]
                mod section_1036;
                #[macro_use]
                mod section_1037;   
            }
        );
        #[macro_use]
        mod section_1038;
        reversing_order_items!(
            {
                #[macro_use]
                mod section_1039;
            }
            {
                #[macro_use]
                mod section_1040;
            }
        );
        #[macro_use]
        mod section_1041;
        #[macro_use]
        mod section_1042;
        reversing_order_items!(
            {
                mod section_1043;
            }
            {
                #[macro_use]
                mod section_1044;
            }
            {
                #[macro_use]
                mod section_1045;
            }
            {
                #[macro_use]
                mod section_1046;
                mod section_1047;
                #[macro_use]
                mod section_1048;
                mod section_1049;
                mod section_1050;
                mod section_1051;
                mod section_1052;
                #[macro_use]
                mod section_1053;
                mod section_1054;
                mod section_1055;
                #[macro_use]
                mod section_1056;
                #[macro_use]
                mod section_1057;
                mod section_1058;
                #[macro_use]
                mod section_1059;
                mod section_1060;
                mod section_1061;
                mod section_1062;
                #[macro_use]
                mod section_1063;
                mod section_1064;
                mod section_1065;
                mod section_1066;
                #[macro_use]
                mod section_1067;
                reversing_order_items!(
                    {
                        mod section_1068;
                    }
                    {
                        mod section_1069;
                        mod section_1070;
                        mod section_1071;
                        #[macro_use]
                        mod section_1072;
                        mod section_1073;
                        mod section_1074;
                        mod section_1075;
                        mod section_1076;
                    }
                    {
                        #[macro_use]
                        mod section_1077;
                        mod section_1078;
                        reversing_order_items!(
                            {
                                mod section_1079;
                            }
                            {
                                mod section_1080_to_1082;
                                #[macro_use]
                                mod section_1083;
                            }
                        );
                        mod section_1084;
                        #[macro_use]
                        mod section_1085;
                        mod section_1086;
                    }
                    {
                        #[macro_use]
                        mod section_1087;        
                    }
                    {
                        mod section_1088;
                        mod section_1089;
                        #[macro_use]
                        mod section_1090;
                        mod section_1091;
                        mod section_1092_to_1093;
                        #[macro_use]
                        mod section_1094;
                        mod section_1095;
                        mod section_1096;
                        mod section_1097_to_1101;
                        #[macro_use]
                        mod section_1102;
                        mod section_1103;
                        mod section_1104;
                        mod section_1105;
                        mod section_1106;
                        mod section_1107;
                        #[macro_use]
                        mod section_1108;
                        #[macro_use]
                        mod section_1109;
                        mod section_1110;
                        mod section_1111_to_1125;
                        #[macro_use]
                        mod section_1126;
                        reversing_order_items!(
                            {
                                mod section_1127;
                            }
                            {
                                #[macro_use]
                                mod section_1128;        
                            }
                        );
                        mod section_1129;
                        #[macro_use]
                        mod section_1130;
                        mod section_1131;
                        #[macro_use]
                        mod section_1132;
                    }
                );
                mod section_1133_to_1135;
                mod section_1136_to_1198;
                mod section_1199_to_1207;
                mod section_1208;
                #[macro_use]
                mod section_1209;
                #[macro_use]
                mod section_1210;
                reversing_order_items!(
                    {
                        mod section_1211;
                    }
                    {
                        #[macro_use]
                        mod section_1212;
                        #[macro_use]
                        mod section_1213;
                        #[macro_use]
                        mod section_1214;
                        mod section_1215;
                        mod section_1216;
                        #[macro_use]
                        mod section_1217;
                        #[macro_use]
                        mod section_1218;
                        mod section_1219;
                        #[macro_use]
                        mod section_1220;
                        #[macro_use]
                        mod section_1221;
                        mod section_1222;
                        #[macro_use]
                        mod section_1223;
                        #[macro_use]
                        mod section_1224;
                        #[macro_use]
                        mod section_1225;
                        #[macro_use]
                        mod section_1226;
                        mod section_1227;
                        #[macro_use]
                        mod section_1228;
                        mod section_1229;
                        mod section_1230;
                        #[macro_use]
                        mod section_1231;
                        reversing_order_items!(
                            {
                                #[macro_use]
                                mod section_1232;
                            }
                            {
                                #[macro_use]
                                mod section_1233;
                            }
                        );
                        #[macro_use]
                        mod section_1234;
                        #[macro_use]
                        mod section_1235;
                        reversing_order_items!(
                            {
                                mod section_1236;
                            }
                            {
                                #[macro_use]
                                mod section_1237;
                                #[macro_use]
                                mod section_1238;
                                #[macro_use]
                                mod section_1239;
                                #[macro_use]
                                mod section_1240;
                            }
                        );
                        #[macro_use]
                        mod section_1241;
                        #[macro_use]
                        mod section_1242;
                        mod section_1243_to_1246;
                        mod section_1247;
                        mod section_1248;
                        mod section_1249;
                        mod section_1250;
                        #[macro_use]
                        mod section_1251;
                        #[macro_use]
                        mod section_1252;
                        #[macro_use]
                        mod section_1253;
                        mod section_1254;
                        #[macro_use]
                        mod section_1255;
                        #[macro_use]
                        mod section_1256;
                        mod section_1257;
                    }
                    {
                        #[macro_use]
                        mod section_1258;
                    }
                    {
                        #[macro_use]
                        mod section_1259;
                        #[macro_use]
                        mod section_1260;
                    }
                    {
                        #[macro_use]
                        mod section_1261;
                        mod section_1262;
                        #[macro_use]
                        mod section_1263;
                        #[macro_use]
                        mod section_1264;
                        mod section_1265;
                        mod section_1266;
                        mod section_1267;
                        mod section_1268;
                        #[macro_use]
                        mod section_1269;
                    }
                );
                mod section_1270;
                mod section_1271;
                mod section_1272;
                #[macro_use]
                mod section_1273;
                #[macro_use]
                mod section_1274;
                mod section_1275;
                #[macro_use]
                mod section_1276;
                mod section_1277;
                #[macro_use]
                mod section_1278;
                reversing_order_items!(
                    {
                        mod section_1279;
                    }
                    {
                        #[macro_use]
                        mod section_1280;
                    }
                    {
                        mod section_1281;
                        mod section_1282;
                        #[macro_use]
                        mod section_1283;
                    }
                );
                mod section_1284;
                #[macro_use]
                mod section_1285;
                mod section_1286;
                #[macro_use]
                mod section_1287;
                reversing_order_items!(
                    {
                        #[macro_use]
                        mod section_1288;
                    }
                    {
                        #[macro_use]
                        mod section_1289;
                    }
                );
                #[macro_use]
                mod section_1290;
                mod section_1291;
                #[macro_use]
                mod section_1292;
                reversing_order_items! (
                    {
                        mod section_1293;
                    }
                    {
                        #[macro_use]
                        mod section_1294;
                        #[macro_use]
                        mod section_1295;
                        #[macro_use]
                        mod section_1296;
                        #[macro_use]
                        mod section_1297;
                        #[macro_use]
                        mod section_1298;
                    }
                );
                mod section_1299;
                mod section_1300;
                mod section_1301;
                mod section_1302_to_1329;
                mod section_1330;
                mod section_1331;
                reversing_order_items!(
                    {
                        mod section_1332;
                    }
                    {
                        macro_rules! forward_mod_k {
                            () => {
                                mod section_1333;
                            }
                        }
                        mod section_1334;
                        mod section_1335;
                        mod section_1336;
                        #[macro_use]
                        mod section_1337;
                    }
                );
                mod section_1338;
                mod section_1339;
                mod section_1340;
                #[macro_use]
                mod section_1341;
                mod section_1342;
                mod section_1343;
                mod section_1344;
                mod section_1345;
                #[macro_use]
                mod section_1346;
                forward_mod_i!();
                #[macro_use]
                mod section_1347;
            }
        );
    }
);
reversing_order_items!(
    {
        mod section_1348;
    }
    {
        mod section_1349;
        mod section_1350;
        #[macro_use]
        mod section_1351;
        #[macro_use]
        mod section_1352;
        #[macro_use]
        mod section_1353;
        mod section_1354_to_1355;
        #[macro_use]
        mod section_1356;
        forward_mod_e!();
        #[macro_use]
        mod section_1357;
        #[macro_use]
        mod section_1358;
        forward_mod_f!();
        mod section_1359_to_1369;
        reversing_order_items!(
            {
                mod section_1370;
            }
            {
                #[macro_use]
                mod section_1371;
            }
        );
        mod section_1372;
        reversing_order_items!(
            {
                mod section_1373;
            }
            {
                #[macro_use]
                mod section_1374;
            }
        );
        #[macro_use]
        mod section_1375;
        mod section_1376;
        #[macro_use]
        mod section_1377;
        #[macro_use]
        mod section_1378;
        forward_mod_k!();
        mod section_1379;
        mod section_1380;
    }
);


mod string_pool;
#[cfg(feature = "unicode_support")]
mod unicode_support;
#[cfg(feature = "latex_support")]
mod latex_support;

pub use section_0004::TeXGlobals;
pub use section_1332::entry;
