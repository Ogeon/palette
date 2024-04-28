# Changelog

## Version 0.7.6 - 2024-04-28

* [#390][390]: Add `From` implementations for changing `Rgb` component types between `u8`, `f32` and `f64`.
* [#342][342]: Implement CAM16. Closes [#199][199].
* [#386][386]: Fix angle conversion from `f32` to `u8`. Closes [#385][385].
* [#384][384]: Add traits for color schemes from traditional color theory.

## Version 0.7.5 - 2024-02-25

* [#380][380]: Avoid recursive trait resolution for `IntoIterator`. Closes [#283][283].

## Version 0.7.4 - 2024-01-28

* [#373][373]: Add an "alloc" feature and make tests work with any feature combination. Closes [#366][366].
* [#374][374]: Fix typo about max range of u8. Closes [#1234][1234], [#4321][4321].
* [#369][369]: Fix potential `NaN` from converting to `Okhsl` when the input is white or black. Closes [#368][368].
* [#355][355]: Add traits for delta E and the improved formulas from Huang et al.
* [#351][351]: Improve some documentation for beginners.
* [#347][347]: Implements Rgba::from_str().

## Version 0.7.3 - 2023-08-10

* [#345][345]: Add `ArraysAs`, `AsArrays`, and corresponding traits for components and uints.
* [#344][344]: Fix Oklab from Oklch hue conversion. Closes [#1234][1234], [#4321][4321].
* [#343][343]: Spelling fixes.
* [#338][338]: Add traits for casting collections of colors to and from other data types.

## Version 0.7.2 - 2023-05-21

* [#332][332]: Fix NaN values from `Okhsv` when `saturation` is > 0 and `value` == 0. Closes [#330][330].
* [#328][328]: Add the HyAB color difference metric. Closes [#318][318].
* [#326][326]: Add a `Wcag21RelativeContrast` trait and deprecate `RelativeContrast`.
* [#320][320]: Implement struct-of-arrays (SoA) utilities. Closes [#305][305].
* [#323][323]: Add saturating_add and saturating_sub for integer based colors. Closes [#322][322].
* [#321][321]: Spelling fixes.
* [#316][316]: Add `EuclideanDistance` and `Ciede2000` traits, deprecate `ColorDifference`. Closes [#288][288].

## Version 0.7.1 - 2023-04-16

* [#313][313]: Improve serializing. Closes [#130][130].
* [#312][312]: Remove  `impl<C> $op_trait<PreAlpha<C>> for {f32,f64}` `impl`s to work around issue #283. Closes [#283][283].

## Version 0.7.0 - 2023-04-10

* [#308][308]: Allow RGB/XYZ conversion matrices to be pre-defined.
* [#307][307]: Update `syn` and dev dependencies.
* [#302][302]: Lift linearity restriction for `Rgb` and `Luma` and update docs.
* [#301][301]: Remove the `Gradient` type and its module. Closes [#152][152], [#156][156].
* [#282][282]: Add `Okhsv`, `Okhsl` and `Okhwb`.
* [#297][297]: Expose doc comment for `Rgb::from_str`. Closes [#1234][1234], [#4321][4321].
* [#286][286]: Reduce the minimum compile time a bit.
* [#279][279]: Split the `TransferFn` trait and add lookup tables for sRGB. Closes [#126][126], [#245][245].
* [#278][278]: Implement SIMD support and add `wide` integration.
* [#273][273]: Split and rework the `Blend` trait and bump MSRV to 1.55.0. Closes [#243][243].
* [#272][272]: Fix broken link to SVG color names.
* [#270][270]: Correcting documentation link.
* [#269][269]: Rework component traits to be more granular and remove `num_traits`.
* [#257][257]: Add in-place conversion traits for slices and references.
* [#256][256]: Implement `FromColorUnclamped` and `FromColor` for `Vec<T>` and `Box<[T]>`.
* [#255][255]: Add unsigned integer casting to `cast` and make `Packed` general purpose.
* [#254][254]: Replace the `Pixel` trait with `ArrayCast` and cast functions and increase the MSRV to 1.51.0.
* [#251][251]: Split `Saturate` into `Saturate` and `Desaturate`.
* [#250][250]: Split the `Hue` trait into more specific traits.
* [#249][249]: Split `Shade` into `Lighten` and `Darken`, and add `*Assign` variants.
* [#248][248]: Add a `MixAssign` trait and remove the `Float` requirement from `Mix`.
* [#247][247]: Split `Clamp` into smaller traits and implement for `[T]`.
* [#246][246]: Make most operator traits take their input by value and change `TransferFn` to `TransferFn<T>`.
* [#240][240]: Add an Any white point. Closes [#194][194].
* [#239][239]: Make color constructors const and remove or replace all `with_wp`. Closes [#134][134].
* [#238][238]: Relax trait bounds for all color types.

## Version 0.6.1 - 2022-07-16

* [#290][290]: Update phf to 0.11.0. Closes [#289][289].

## Version 0.6.0 - 2021-07-12

* [#235][235]: Upgrade phf to 0.9 and enable named_from_str for no_std.
* [#200][200]: Add Oklab support. Closes [#222][222].
* [#231][231]: Update `approx` and `find-crate` dependencies.
* [#229][229]: Implement `bytemuck::Zeroable` and `bytemuck::Pod` for every color type.
* [#225][225]: Add Hsluv support. Closes [#112][112].
* [#223][223]: Add Lchuv support.
* [#221][221]: Add CIE Luv support.
* [#217][217]: Implement relative and absolute methods for Lighten/Darken, Saturate. Closes [#215][215].
* [#216][216]: Add doc alias, doc cleanups, remove trait from Packed struct.
* [#211][211]: Implement PartialEq/Eq for all colorspaces, Alpha, PreAlpha, and LabHue/RgbHue. Closes [#206][206].
* [#210][210]: Rename Limited trait to Clamp. Closes [#209][209].
* [#205][205]: Generalizing gradients and add constant gradients. Closes [#62][62].
* [#190][190]: Convert documentation to intra doc links, add default whitepoint for Lab/Lch, make code fixups. Closes [#177][177].
* [#189][189]: Correct scaling on random distribution of Lab/Lch.
* [#188][188]: Allow HSV, HSL and HWB to represent nonlinear RGB. Closes [#160][160], [#187][187].
* [#184][184]: Optimize into_component for float_to_uint, u8 to f32/f64.
* [#183][183]: Optimize matrix functions, color conversion performance.
* [#176][176]: Rewrite the conversion traits to work more like From and Into. Closes [#41][41], [#111][111].
* [#175][175]: Add feature "random" for random color generation using `rand` crate. Closes [#174][174].
* [#173][173]: Add functions to get min/max component values for all color types, alpha.
* [#170][170]: Add `{into,from}_u32` methods for RGB/A, Packed struct for u32 representations of RGBA. Closes [#144][144].
* [#164][164]: Implement WCAG contrast ratio calculations.
* [#162][162]: Implement CIEDE2000 color difference for Lab/Lch. Closes [#143][143].
* [#161][161]: Split the Component trait into more specific traits.
* [#157][157]: Implement `FromStr` method for `Rgb<S, u8>`. Closes [#148][148].
* [#158][158]: Make `Take` iterator for gradient inclusive of both end colors, add tests.
* [#154][154]: Add DoubleEndedIterator impl for gradient::Take. Closes [#153][153].
* [#137][137]: Add some missing `From` impls between `Srgb` and `LinSrgb` types.

## Version 0.5.0 - 2019-11-17

* [#149][149]: Use libm through num_traits, and update all dependencies.
* [#142][142]: Make libm optional. Closes [#116][116].
* [#138][138]: Fix no_std build failure.
* [#136][136]: Update dependencies and remove --release flag from feature tests.
* [#135][135]: Round to nearest instead of down when converting components to integers.
* [#127][127]: fix no_std. Closes [#125][125].
* [#124][124]: Update approx dependency to 0.3.
* [#119][119]: Remove the color enum. Closes [#72][72].
* [#118][118]: Implement assign ops. Closes [#94][94].
* [#110][110]: No std support. Closes [#108][108].
* [#106][106]: Add Extended Conversion Trait.
* [#104][104]: Update image and approx crate dependency. Closes [#101][101], [#100][100].

## Version 0.4.1 - 2018-08-02

* [#113][113]: Import everything from the parent scope in derives.

## Version 0.4.0 - 2018-05-26

* [#99][99]: Fix into and from component tuple conversion for Yxy.
* [#98][98]: Add conversion to and from component tuples. Closes [#87][87].
* [#97][97]: Add hexadecimal formatting to Alpha, Luma and Rgb. Closes [#80][80].
* [#96][96]: Reexport derives from the main library. Closes [#91][91].
* [#93][93]: Make it possible to derive Pixel. Closes [#85][85].
* [#92][92]: Add transparency support when deriving FromColor and IntoColor. Closes [#86][86].
* [#90][90]: Add serde support as an optional feature. Closes [#83][83].
* [#89][89]: Improve the hue types a bit. Closes [#75][75].
* [#84][84]: Make it possible to derive IntoColor and FromColor. Closes [#82][82].
* [#81][81]: Make a new system for converting to and from arrays and slices. Closes [#74][74].

## Version 0.3.0 - 2018-02-17

* [#78][78]: Upgrade dependencies.
* [#60][60]: Generalize the RGB types over RGB standards. Closes [#66][66], [#31][31], [#58][58].
* [#76][76]: Change dependency `num` to `num_traits` to shrink dependency tree.
* [#63][63]: Add rebeccapurple.
* [#61][61]: Restore the proper scale of Lab and Lch. Closes [#49][49].
* [#56][56]: Make color spaces white point aware. Closes [#14][14].

## Version 0.2.1 - 2016-02-23

* [#39][39]: Implement color blending. Closes [#3][3].
* [#54][54]: Faster Rgb to Hsl and Hsv conversions.
* [#52][52]: Add missing ApproxEq implementations.
* [#53][53]: Add hwb color. Closes [#32][32].
* [#51][51]: Implement approx eq trait.
* [#48][48]: Add tests for conversions. Closes [#44][44].
* [#47][47]: Change normalize to use floor and ceil formula. Closes [#46][46].
* [#43][43]: Add conversion trait.
* [#34][34]: Add color constants. Closes [#5][5].
* [#35][35]: use flt() function. Closes [#33][33].
* [#30][30]: Add Cie Yxy (xyY) Color Space.
* [#29][29]: Derive Clone for Take and MaybeSlice.

## Version 0.2.0 - 2016-01-30

* [#26][26]: Offer both 0 centered and positive hue -> float conversion. Closes [#15][15].
* [#25][25]: Fix or relax some color ranges and clamping. Closes [#19][19].
* [#22][22]: Extract the alpha component as a wrapper type. Closes [#11][11].
* [#24][24]: Separate sRGB and gamma encoded RGB from the Rgb type. Closes [#7][7].
* [#23][23]: Change Mix, Shade and Saturate to use an associated type.
* [#18][18]: Convert all colors to be generic over floats, f32 and f64. Closes [#13][13].

## Version 0.1.1 - 2016-01-21

* [#12][12]: Implement Gradient slicing and exact size iteration. Closes [#4][4].
* [#9][9]: Implement color arithmetics. Closes [#2][2].

## Version 0.1.0 - 2016-01-12

The first published version.

[9]: https://github.com/Ogeon/palette/pull/9
[12]: https://github.com/Ogeon/palette/pull/12
[18]: https://github.com/Ogeon/palette/pull/18
[22]: https://github.com/Ogeon/palette/pull/22
[23]: https://github.com/Ogeon/palette/pull/23
[24]: https://github.com/Ogeon/palette/pull/24
[25]: https://github.com/Ogeon/palette/pull/25
[26]: https://github.com/Ogeon/palette/pull/26
[29]: https://github.com/Ogeon/palette/pull/29
[30]: https://github.com/Ogeon/palette/pull/30
[34]: https://github.com/Ogeon/palette/pull/34
[35]: https://github.com/Ogeon/palette/pull/35
[39]: https://github.com/Ogeon/palette/pull/39
[43]: https://github.com/Ogeon/palette/pull/43
[47]: https://github.com/Ogeon/palette/pull/47
[48]: https://github.com/Ogeon/palette/pull/48
[51]: https://github.com/Ogeon/palette/pull/51
[52]: https://github.com/Ogeon/palette/pull/52
[53]: https://github.com/Ogeon/palette/pull/53
[54]: https://github.com/Ogeon/palette/pull/54
[56]: https://github.com/Ogeon/palette/pull/56
[60]: https://github.com/Ogeon/palette/pull/60
[61]: https://github.com/Ogeon/palette/pull/61
[63]: https://github.com/Ogeon/palette/pull/63
[76]: https://github.com/Ogeon/palette/pull/76
[78]: https://github.com/Ogeon/palette/pull/78
[81]: https://github.com/Ogeon/palette/pull/81
[84]: https://github.com/Ogeon/palette/pull/84
[89]: https://github.com/Ogeon/palette/pull/89
[90]: https://github.com/Ogeon/palette/pull/90
[92]: https://github.com/Ogeon/palette/pull/92
[93]: https://github.com/Ogeon/palette/pull/93
[96]: https://github.com/Ogeon/palette/pull/96
[97]: https://github.com/Ogeon/palette/pull/97
[98]: https://github.com/Ogeon/palette/pull/98
[99]: https://github.com/Ogeon/palette/pull/99
[104]: https://github.com/Ogeon/palette/pull/104
[106]: https://github.com/Ogeon/palette/pull/106
[110]: https://github.com/Ogeon/palette/pull/110
[113]: https://github.com/Ogeon/palette/pull/113
[118]: https://github.com/Ogeon/palette/pull/118
[119]: https://github.com/Ogeon/palette/pull/119
[124]: https://github.com/Ogeon/palette/pull/124
[127]: https://github.com/Ogeon/palette/pull/127
[135]: https://github.com/Ogeon/palette/pull/135
[136]: https://github.com/Ogeon/palette/pull/136
[137]: https://github.com/Ogeon/palette/pull/137
[138]: https://github.com/Ogeon/palette/pull/138
[142]: https://github.com/Ogeon/palette/pull/142
[149]: https://github.com/Ogeon/palette/pull/149
[154]: https://github.com/Ogeon/palette/pull/154
[157]: https://github.com/Ogeon/palette/pull/157
[158]: https://github.com/Ogeon/palette/pull/158
[161]: https://github.com/Ogeon/palette/pull/161
[162]: https://github.com/Ogeon/palette/pull/162
[164]: https://github.com/Ogeon/palette/pull/164
[170]: https://github.com/Ogeon/palette/pull/170
[173]: https://github.com/Ogeon/palette/pull/173
[175]: https://github.com/Ogeon/palette/pull/175
[176]: https://github.com/Ogeon/palette/pull/176
[183]: https://github.com/Ogeon/palette/pull/183
[184]: https://github.com/Ogeon/palette/pull/184
[188]: https://github.com/Ogeon/palette/pull/188
[189]: https://github.com/Ogeon/palette/pull/189
[190]: https://github.com/Ogeon/palette/pull/190
[200]: https://github.com/Ogeon/palette/pull/200
[205]: https://github.com/Ogeon/palette/pull/205
[210]: https://github.com/Ogeon/palette/pull/210
[211]: https://github.com/Ogeon/palette/pull/211
[216]: https://github.com/Ogeon/palette/pull/216
[217]: https://github.com/Ogeon/palette/pull/217
[221]: https://github.com/Ogeon/palette/pull/221
[223]: https://github.com/Ogeon/palette/pull/223
[225]: https://github.com/Ogeon/palette/pull/225
[229]: https://github.com/Ogeon/palette/pull/229
[231]: https://github.com/Ogeon/palette/pull/231
[235]: https://github.com/Ogeon/palette/pull/235
[238]: https://github.com/Ogeon/palette/pull/238
[239]: https://github.com/Ogeon/palette/pull/239
[240]: https://github.com/Ogeon/palette/pull/240
[246]: https://github.com/Ogeon/palette/pull/246
[247]: https://github.com/Ogeon/palette/pull/247
[248]: https://github.com/Ogeon/palette/pull/248
[249]: https://github.com/Ogeon/palette/pull/249
[250]: https://github.com/Ogeon/palette/pull/250
[251]: https://github.com/Ogeon/palette/pull/251
[254]: https://github.com/Ogeon/palette/pull/254
[255]: https://github.com/Ogeon/palette/pull/255
[256]: https://github.com/Ogeon/palette/pull/256
[257]: https://github.com/Ogeon/palette/pull/257
[269]: https://github.com/Ogeon/palette/pull/269
[270]: https://github.com/Ogeon/palette/pull/270
[272]: https://github.com/Ogeon/palette/pull/272
[273]: https://github.com/Ogeon/palette/pull/273
[278]: https://github.com/Ogeon/palette/pull/278
[279]: https://github.com/Ogeon/palette/pull/279
[282]: https://github.com/Ogeon/palette/pull/282
[286]: https://github.com/Ogeon/palette/pull/286
[290]: https://github.com/Ogeon/palette/pull/290
[297]: https://github.com/Ogeon/palette/pull/297
[301]: https://github.com/Ogeon/palette/pull/301
[302]: https://github.com/Ogeon/palette/pull/302
[307]: https://github.com/Ogeon/palette/pull/307
[308]: https://github.com/Ogeon/palette/pull/308
[312]: https://github.com/Ogeon/palette/pull/312
[313]: https://github.com/Ogeon/palette/pull/313
[316]: https://github.com/Ogeon/palette/pull/316
[320]: https://github.com/Ogeon/palette/pull/320
[321]: https://github.com/Ogeon/palette/pull/321
[323]: https://github.com/Ogeon/palette/pull/323
[326]: https://github.com/Ogeon/palette/pull/326
[328]: https://github.com/Ogeon/palette/pull/328
[332]: https://github.com/Ogeon/palette/pull/332
[338]: https://github.com/Ogeon/palette/pull/338
[342]: https://github.com/Ogeon/palette/pull/342
[343]: https://github.com/Ogeon/palette/pull/343
[344]: https://github.com/Ogeon/palette/pull/344
[345]: https://github.com/Ogeon/palette/pull/345
[347]: https://github.com/Ogeon/palette/pull/347
[351]: https://github.com/Ogeon/palette/pull/351
[355]: https://github.com/Ogeon/palette/pull/355
[369]: https://github.com/Ogeon/palette/pull/369
[373]: https://github.com/Ogeon/palette/pull/373
[374]: https://github.com/Ogeon/palette/pull/374
[380]: https://github.com/Ogeon/palette/pull/380
[384]: https://github.com/Ogeon/palette/pull/384
[386]: https://github.com/Ogeon/palette/pull/386
[390]: https://github.com/Ogeon/palette/pull/390
[2]: https://github.com/Ogeon/palette/issues/2
[3]: https://github.com/Ogeon/palette/issues/3
[4]: https://github.com/Ogeon/palette/issues/4
[5]: https://github.com/Ogeon/palette/issues/5
[7]: https://github.com/Ogeon/palette/issues/7
[11]: https://github.com/Ogeon/palette/issues/11
[13]: https://github.com/Ogeon/palette/issues/13
[14]: https://github.com/Ogeon/palette/issues/14
[15]: https://github.com/Ogeon/palette/issues/15
[19]: https://github.com/Ogeon/palette/issues/19
[31]: https://github.com/Ogeon/palette/issues/31
[32]: https://github.com/Ogeon/palette/issues/32
[33]: https://github.com/Ogeon/palette/issues/33
[41]: https://github.com/Ogeon/palette/issues/41
[44]: https://github.com/Ogeon/palette/issues/44
[46]: https://github.com/Ogeon/palette/issues/46
[49]: https://github.com/Ogeon/palette/issues/49
[58]: https://github.com/Ogeon/palette/issues/58
[62]: https://github.com/Ogeon/palette/issues/62
[66]: https://github.com/Ogeon/palette/issues/66
[72]: https://github.com/Ogeon/palette/issues/72
[74]: https://github.com/Ogeon/palette/issues/74
[75]: https://github.com/Ogeon/palette/issues/75
[80]: https://github.com/Ogeon/palette/issues/80
[82]: https://github.com/Ogeon/palette/issues/82
[83]: https://github.com/Ogeon/palette/issues/83
[85]: https://github.com/Ogeon/palette/issues/85
[86]: https://github.com/Ogeon/palette/issues/86
[87]: https://github.com/Ogeon/palette/issues/87
[91]: https://github.com/Ogeon/palette/issues/91
[94]: https://github.com/Ogeon/palette/issues/94
[100]: https://github.com/Ogeon/palette/issues/100
[101]: https://github.com/Ogeon/palette/issues/101
[108]: https://github.com/Ogeon/palette/issues/108
[111]: https://github.com/Ogeon/palette/issues/111
[112]: https://github.com/Ogeon/palette/issues/112
[116]: https://github.com/Ogeon/palette/issues/116
[125]: https://github.com/Ogeon/palette/issues/125
[126]: https://github.com/Ogeon/palette/issues/126
[130]: https://github.com/Ogeon/palette/issues/130
[134]: https://github.com/Ogeon/palette/issues/134
[143]: https://github.com/Ogeon/palette/issues/143
[144]: https://github.com/Ogeon/palette/issues/144
[148]: https://github.com/Ogeon/palette/issues/148
[152]: https://github.com/Ogeon/palette/issues/152
[153]: https://github.com/Ogeon/palette/issues/153
[156]: https://github.com/Ogeon/palette/issues/156
[160]: https://github.com/Ogeon/palette/issues/160
[174]: https://github.com/Ogeon/palette/issues/174
[177]: https://github.com/Ogeon/palette/issues/177
[187]: https://github.com/Ogeon/palette/issues/187
[194]: https://github.com/Ogeon/palette/issues/194
[199]: https://github.com/Ogeon/palette/issues/199
[206]: https://github.com/Ogeon/palette/issues/206
[209]: https://github.com/Ogeon/palette/issues/209
[215]: https://github.com/Ogeon/palette/issues/215
[222]: https://github.com/Ogeon/palette/issues/222
[243]: https://github.com/Ogeon/palette/issues/243
[245]: https://github.com/Ogeon/palette/issues/245
[283]: https://github.com/Ogeon/palette/issues/283
[288]: https://github.com/Ogeon/palette/issues/288
[289]: https://github.com/Ogeon/palette/issues/289
[305]: https://github.com/Ogeon/palette/issues/305
[318]: https://github.com/Ogeon/palette/issues/318
[322]: https://github.com/Ogeon/palette/issues/322
[330]: https://github.com/Ogeon/palette/issues/330
[366]: https://github.com/Ogeon/palette/issues/366
[368]: https://github.com/Ogeon/palette/issues/368
[385]: https://github.com/Ogeon/palette/issues/385
[1234]: https://github.com/Ogeon/palette/issues/1234
[4321]: https://github.com/Ogeon/palette/issues/4321
