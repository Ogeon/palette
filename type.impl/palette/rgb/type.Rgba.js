(function() {var type_impls = {
"palette":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#581-637\">source</a><a href=\"#impl-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_u32\" class=\"method\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#603-608\">source</a><h4 class=\"code-header\">pub fn <a href=\"palette/rgb/type.Rgba.html#tymethod.into_u32\" class=\"fn\">into_u32</a>&lt;O&gt;(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u32.html\">u32</a><div class=\"where\">where\n    O: <a class=\"trait\" href=\"palette/cast/trait.ComponentOrder.html\" title=\"trait palette::cast::ComponentOrder\">ComponentOrder</a>&lt;<a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u32.html\">u32</a>&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Convert to a packed <code>u32</code> with with specifiable component order.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>palette::{rgb, Srgba};\n\n<span class=\"kw\">let </span>integer = Srgba::new(<span class=\"number\">96u8</span>, <span class=\"number\">127</span>, <span class=\"number\">0</span>, <span class=\"number\">255</span>).into_u32::&lt;rgb::channels::Argb&gt;();\n<span class=\"macro\">assert_eq!</span>(<span class=\"number\">0xFF607F00</span>, integer);</code></pre></div>\n<p>It’s also possible to use <code>From</code> and <code>Into</code>, which defaults to the\n<code>0xRRGGBBAA</code> component order:</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>palette::Srgba;\n\n<span class=\"kw\">let </span>integer = u32::from(Srgba::new(<span class=\"number\">96u8</span>, <span class=\"number\">127</span>, <span class=\"number\">0</span>, <span class=\"number\">255</span>));\n<span class=\"macro\">assert_eq!</span>(<span class=\"number\">0x607F00FF</span>, integer);</code></pre></div>\n<p>See <a href=\"palette/cast/struct.Packed.html\" title=\"struct palette::cast::Packed\">Packed</a> for more details.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_u32\" class=\"method\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#631-636\">source</a><h4 class=\"code-header\">pub fn <a href=\"palette/rgb/type.Rgba.html#tymethod.from_u32\" class=\"fn\">from_u32</a>&lt;O&gt;(color: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u32.html\">u32</a>) -&gt; Self<div class=\"where\">where\n    O: <a class=\"trait\" href=\"palette/cast/trait.ComponentOrder.html\" title=\"trait palette::cast::ComponentOrder\">ComponentOrder</a>&lt;<a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u32.html\">u32</a>&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Convert from a packed <code>u32</code> with specifiable component order.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>palette::{rgb, Srgba};\n\n<span class=\"kw\">let </span>rgba = Srgba::from_u32::&lt;rgb::channels::Argb&gt;(<span class=\"number\">0xFF607F00</span>);\n<span class=\"macro\">assert_eq!</span>(Srgba::new(<span class=\"number\">96u8</span>, <span class=\"number\">127</span>, <span class=\"number\">0</span>, <span class=\"number\">255</span>), rgba);</code></pre></div>\n<p>It’s also possible to use <code>From</code> and <code>Into</code>, which defaults to the\n<code>0xRRGGBBAA</code> component order:</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>palette::Srgba;\n\n<span class=\"kw\">let </span>rgba = Srgba::from(<span class=\"number\">0x607F00FF</span>);\n<span class=\"macro\">assert_eq!</span>(Srgba::new(<span class=\"number\">96u8</span>, <span class=\"number\">127</span>, <span class=\"number\">0</span>, <span class=\"number\">255</span>), rgba);</code></pre></div>\n<p>See <a href=\"palette/cast/struct.Packed.html\" title=\"struct palette::cast::Packed\">Packed</a> for more details.</p>\n</div></details></div></details>",0,"palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CAlpha%3CRgb%3CS%3E,+f32%3E%3E-for-Alpha%3CRgb%3CS,+f64%3E,+f64%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1306-1311\">source</a><a href=\"#impl-From%3CAlpha%3CRgb%3CS%3E,+f32%3E%3E-for-Alpha%3CRgb%3CS,+f64%3E,+f64%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"palette/alpha/struct.Alpha.html\" title=\"struct palette::alpha::Alpha\">Alpha</a>&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f32.html\">f32</a>&gt;&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1308-1310\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(color: <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f32.html\">f32</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Alpha<Rgb<S>, f32>>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CAlpha%3CRgb%3CS%3E,+f32%3E%3E-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1264-1269\">source</a><a href=\"#impl-From%3CAlpha%3CRgb%3CS%3E,+f32%3E%3E-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"palette/alpha/struct.Alpha.html\" title=\"struct palette::alpha::Alpha\">Alpha</a>&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f32.html\">f32</a>&gt;&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1266-1268\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(color: <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f32.html\">f32</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Alpha<Rgb<S>, f32>>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CAlpha%3CRgb%3CS,+f64%3E,+f64%3E%3E-for-Alpha%3CRgb%3CS%3E,+f32%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1320-1325\">source</a><a href=\"#impl-From%3CAlpha%3CRgb%3CS,+f64%3E,+f64%3E%3E-for-Alpha%3CRgb%3CS%3E,+f32%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"palette/alpha/struct.Alpha.html\" title=\"struct palette::alpha::Alpha\">Alpha</a>&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f32.html\">f32</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1322-1324\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(color: <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Alpha<Rgb<S, f64>, f64>>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CAlpha%3CRgb%3CS,+f64%3E,+f64%3E%3E-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1292-1297\">source</a><a href=\"#impl-From%3CAlpha%3CRgb%3CS,+f64%3E,+f64%3E%3E-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"palette/alpha/struct.Alpha.html\" title=\"struct palette::alpha::Alpha\">Alpha</a>&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1294-1296\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(color: <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Alpha<Rgb<S, f64>, f64>>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CAlpha%3CRgb%3CS,+u8%3E,+u8%3E%3E-for-Alpha%3CRgb%3CS%3E,+f32%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1250-1255\">source</a><a href=\"#impl-From%3CAlpha%3CRgb%3CS,+u8%3E,+u8%3E%3E-for-Alpha%3CRgb%3CS%3E,+f32%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"palette/alpha/struct.Alpha.html\" title=\"struct palette::alpha::Alpha\">Alpha</a>&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f32.html\">f32</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1252-1254\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(color: <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Alpha<Rgb<S, u8>, u8>>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CAlpha%3CRgb%3CS,+u8%3E,+u8%3E%3E-for-Alpha%3CRgb%3CS,+f64%3E,+f64%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1278-1283\">source</a><a href=\"#impl-From%3CAlpha%3CRgb%3CS,+u8%3E,+u8%3E%3E-for-Alpha%3CRgb%3CS,+f64%3E,+f64%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"palette/alpha/struct.Alpha.html\" title=\"struct palette::alpha::Alpha\">Alpha</a>&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.f64.html\">f64</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1280-1282\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(color: <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Alpha<Rgb<S, u8>, u8>>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CPacked%3CO,+P%3E%3E-for-Alpha%3CRgb%3CS,+T%3E,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1205-1213\">source</a><a href=\"#impl-From%3CPacked%3CO,+P%3E%3E-for-Alpha%3CRgb%3CS,+T%3E,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S, T, O, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"palette/cast/struct.Packed.html\" title=\"struct palette::cast::Packed\">Packed</a>&lt;O, P&gt;&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"palette/cast/trait.ComponentOrder.html\" title=\"trait palette::cast::ComponentOrder\">ComponentOrder</a>&lt;<a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, T&gt;, P&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1210-1212\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(packed: <a class=\"struct\" href=\"palette/cast/struct.Packed.html\" title=\"struct palette::cast::Packed\">Packed</a>&lt;O, P&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Packed<O, P>>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3Cu32%3E-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1222-1227\">source</a><a href=\"#impl-From%3Cu32%3E-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u32.html\">u32</a>&gt; for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1224-1226\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(color: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u32.html\">u32</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<u32>","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FromStr-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1145-1172\">source</a><a href=\"#impl-FromStr-for-Alpha%3CRgb%3CS,+u8%3E,+u8%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.0/core/str/traits/trait.FromStr.html\" title=\"trait core::str::traits::FromStr\">FromStr</a> for <a class=\"type\" href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\">Rgba</a>&lt;S, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.u8.html\">u8</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_str\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/palette/rgb/rgb.rs.html#1150-1171\">source</a><a href=\"#method.from_str\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.0/core/str/traits/trait.FromStr.html#tymethod.from_str\" class=\"fn\">from_str</a>(hex: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.0/std/primitive.str.html\">str</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.80.0/core/str/traits/trait.FromStr.html#associatedtype.Err\" title=\"type core::str::traits::FromStr::Err\">Err</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Parses a color hex code of format ‘#ff00bbff’ or ‘#abcd’ (with or without the leading ‘#’) into a\n<a href=\"palette/rgb/type.Rgba.html\" title=\"type palette::rgb::Rgba\"><code>Rgba&lt;S, u8&gt;</code></a> instance.</p>\n</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Err\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Err\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.80.0/core/str/traits/trait.FromStr.html#associatedtype.Err\" class=\"associatedtype\">Err</a> = <a class=\"enum\" href=\"palette/rgb/enum.FromHexError.html\" title=\"enum palette::rgb::FromHexError\">FromHexError</a></h4></section></summary><div class='docblock'>The associated error which can be returned from parsing.</div></details></div></details>","FromStr","palette::rgb::Srgba","palette::rgb::LinSrgba","palette::rgb::GammaSrgba"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()