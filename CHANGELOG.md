# Changelog

## Version 0.5.0 - 2019-11-17

* [#149][149]: Use libm through num_traits, and update all dependencies.
* [#146][146]: bump syn, quote and proc-macro2 to version 1.0.
* [#142][142]: Make libm optional. Closes [#116][116].
* [#141][141]: Also test no_std with a target without std support.
* [#140][140]: Add a basic test crate for no_std. Closes [#139][139].
* [#138][138]: Fix no_std build failure.
* [#136][136]: Update dependencies and remove --release flag from feature tests.
* [#135][135]: Round to nearest instead of down when converting components to integers..
* [#132][132]: Add the dyn keyword where trait objects are used.
* [#127][127]: fix no_std. Closes [#125][125].
* [#124][124]: Update approx dependency to 0.3.
* [#122][122]: Update to syn 0.15.
* [#120][120]: Fix grammar in readme.
* [#119][119]: Remove the color enum. Closes [#72][72].
* [#118][118]: Implement assign ops. Closes [#94][94].
* [#110][110]: No std support. Closes [#108][108].
* [#106][106]: Add Extended Conversion Trait .
* [#104][104]: Update image and approx crate dependency. Closes [#101][101], [#100][100].
* [#103][103]: Use Generics::make_where_clause helper.

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

[149]: https://github.com/Ogeon/palette/pull/149
[146]: https://github.com/Ogeon/palette/pull/146
[142]: https://github.com/Ogeon/palette/pull/142
[141]: https://github.com/Ogeon/palette/pull/141
[140]: https://github.com/Ogeon/palette/pull/140
[138]: https://github.com/Ogeon/palette/pull/138
[136]: https://github.com/Ogeon/palette/pull/136
[135]: https://github.com/Ogeon/palette/pull/135
[132]: https://github.com/Ogeon/palette/pull/132
[127]: https://github.com/Ogeon/palette/pull/127
[124]: https://github.com/Ogeon/palette/pull/124
[122]: https://github.com/Ogeon/palette/pull/122
[120]: https://github.com/Ogeon/palette/pull/120
[119]: https://github.com/Ogeon/palette/pull/119
[118]: https://github.com/Ogeon/palette/pull/118
[110]: https://github.com/Ogeon/palette/pull/110
[113]: https://github.com/Ogeon/palette/pull/113
[106]: https://github.com/Ogeon/palette/pull/106
[104]: https://github.com/Ogeon/palette/pull/104
[103]: https://github.com/Ogeon/palette/pull/103
[99]: https://github.com/Ogeon/palette/pull/99
[98]: https://github.com/Ogeon/palette/pull/98
[97]: https://github.com/Ogeon/palette/pull/97
[96]: https://github.com/Ogeon/palette/pull/96
[93]: https://github.com/Ogeon/palette/pull/93
[92]: https://github.com/Ogeon/palette/pull/92
[90]: https://github.com/Ogeon/palette/pull/90
[89]: https://github.com/Ogeon/palette/pull/89
[84]: https://github.com/Ogeon/palette/pull/84
[81]: https://github.com/Ogeon/palette/pull/81
[78]: https://github.com/Ogeon/palette/pull/78
[60]: https://github.com/Ogeon/palette/pull/60
[76]: https://github.com/Ogeon/palette/pull/76
[63]: https://github.com/Ogeon/palette/pull/63
[61]: https://github.com/Ogeon/palette/pull/61
[56]: https://github.com/Ogeon/palette/pull/56
[39]: https://github.com/Ogeon/palette/pull/39
[54]: https://github.com/Ogeon/palette/pull/54
[52]: https://github.com/Ogeon/palette/pull/52
[53]: https://github.com/Ogeon/palette/pull/53
[51]: https://github.com/Ogeon/palette/pull/51
[48]: https://github.com/Ogeon/palette/pull/48
[47]: https://github.com/Ogeon/palette/pull/47
[43]: https://github.com/Ogeon/palette/pull/43
[34]: https://github.com/Ogeon/palette/pull/34
[35]: https://github.com/Ogeon/palette/pull/35
[30]: https://github.com/Ogeon/palette/pull/30
[29]: https://github.com/Ogeon/palette/pull/29
[26]: https://github.com/Ogeon/palette/pull/26
[25]: https://github.com/Ogeon/palette/pull/25
[22]: https://github.com/Ogeon/palette/pull/22
[24]: https://github.com/Ogeon/palette/pull/24
[23]: https://github.com/Ogeon/palette/pull/23
[18]: https://github.com/Ogeon/palette/pull/18
[12]: https://github.com/Ogeon/palette/pull/12
[9]: https://github.com/Ogeon/palette/pull/9
[116]: https://github.com/Ogeon/palette/issues/116
[139]: https://github.com/Ogeon/palette/issues/139
[125]: https://github.com/Ogeon/palette/issues/125
[72]: https://github.com/Ogeon/palette/issues/72
[94]: https://github.com/Ogeon/palette/issues/94
[108]: https://github.com/Ogeon/palette/issues/108
[101]: https://github.com/Ogeon/palette/issues/101
[100]: https://github.com/Ogeon/palette/issues/100
[87]: https://github.com/Ogeon/palette/issues/87
[80]: https://github.com/Ogeon/palette/issues/80
[91]: https://github.com/Ogeon/palette/issues/91
[85]: https://github.com/Ogeon/palette/issues/85
[86]: https://github.com/Ogeon/palette/issues/86
[83]: https://github.com/Ogeon/palette/issues/83
[75]: https://github.com/Ogeon/palette/issues/75
[82]: https://github.com/Ogeon/palette/issues/82
[74]: https://github.com/Ogeon/palette/issues/74
[66]: https://github.com/Ogeon/palette/issues/66
[31]: https://github.com/Ogeon/palette/issues/31
[58]: https://github.com/Ogeon/palette/issues/58
[49]: https://github.com/Ogeon/palette/issues/49
[14]: https://github.com/Ogeon/palette/issues/14
[3]: https://github.com/Ogeon/palette/issues/3
[32]: https://github.com/Ogeon/palette/issues/32
[44]: https://github.com/Ogeon/palette/issues/44
[46]: https://github.com/Ogeon/palette/issues/46
[5]: https://github.com/Ogeon/palette/issues/5
[33]: https://github.com/Ogeon/palette/issues/33
[15]: https://github.com/Ogeon/palette/issues/15
[19]: https://github.com/Ogeon/palette/issues/19
[11]: https://github.com/Ogeon/palette/issues/11
[7]: https://github.com/Ogeon/palette/issues/7
[13]: https://github.com/Ogeon/palette/issues/13
[4]: https://github.com/Ogeon/palette/issues/4
[2]: https://github.com/Ogeon/palette/issues/2
