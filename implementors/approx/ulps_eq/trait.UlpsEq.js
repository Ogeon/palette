(function() {var implementors = {
"palette":[["impl&lt;Wp, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Hsluv.html\" title=\"struct palette::Hsluv\">Hsluv</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Hsluv.html\" title=\"struct palette::Hsluv\">Hsluv</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.LuvHue.html\" title=\"struct palette::LuvHue\">LuvHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;C, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C&gt;&gt; for <a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C&gt;<span class=\"where fmt-newline\">where\n    C: UlpsEq&lt;Epsilon = T::Epsilon&gt; + <a class=\"trait\" href=\"palette/blend/trait.Premultiply.html\" title=\"trait palette::blend::Premultiply\">Premultiply</a>&lt;Scalar = T&gt;,\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;S, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Hsl.html\" title=\"struct palette::Hsl\">Hsl</a>&lt;S, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Hsl.html\" title=\"struct palette::Hsl\">Hsl</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.RgbHue.html\" title=\"struct palette::RgbHue\">RgbHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;S, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/luma/struct.Luma.html\" title=\"struct palette::luma::Luma\">Luma</a>&lt;S, T&gt;&gt; for <a class=\"struct\" href=\"palette/luma/struct.Luma.html\" title=\"struct palette::luma::Luma\">Luma</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Okhsv.html\" title=\"struct palette::Okhsv\">Okhsv</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Okhsv.html\" title=\"struct palette::Okhsv\">Okhsv</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.OklabHue.html\" title=\"struct palette::OklabHue\">OklabHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;Wp, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Lchuv.html\" title=\"struct palette::Lchuv\">Lchuv</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Lchuv.html\" title=\"struct palette::Lchuv\">Lchuv</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.LuvHue.html\" title=\"struct palette::LuvHue\">LuvHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;Wp, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Yxy.html\" title=\"struct palette::Yxy\">Yxy</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Yxy.html\" title=\"struct palette::Yxy\">Yxy</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;Wp, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Lch.html\" title=\"struct palette::Lch\">Lch</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Lch.html\" title=\"struct palette::Lch\">Lch</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.LabHue.html\" title=\"struct palette::LabHue\">LabHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.LabHue.html\" title=\"struct palette::LabHue\">LabHue</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.LabHue.html\" title=\"struct palette::LabHue\">LabHue</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/angle/trait.RealAngle.html\" title=\"trait palette::angle::RealAngle\">RealAngle</a> + <a class=\"trait\" href=\"palette/angle/trait.SignedAngle.html\" title=\"trait palette::angle::SignedAngle\">SignedAngle</a> + <a class=\"trait\" href=\"palette/num/trait.Zero.html\" title=\"trait palette::num::Zero\">Zero</a> + <a class=\"trait\" href=\"palette/angle/trait.AngleEq.html\" title=\"trait palette::angle::AngleEq\">AngleEq</a>&lt;Mask = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.bool.html\">bool</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"palette/angle/trait.HalfRotation.html\" title=\"trait palette::angle::HalfRotation\">HalfRotation</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;Output = T::Epsilon&gt;,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.RgbHue.html\" title=\"struct palette::RgbHue\">RgbHue</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.RgbHue.html\" title=\"struct palette::RgbHue\">RgbHue</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/angle/trait.RealAngle.html\" title=\"trait palette::angle::RealAngle\">RealAngle</a> + <a class=\"trait\" href=\"palette/angle/trait.SignedAngle.html\" title=\"trait palette::angle::SignedAngle\">SignedAngle</a> + <a class=\"trait\" href=\"palette/num/trait.Zero.html\" title=\"trait palette::num::Zero\">Zero</a> + <a class=\"trait\" href=\"palette/angle/trait.AngleEq.html\" title=\"trait palette::angle::AngleEq\">AngleEq</a>&lt;Mask = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.bool.html\">bool</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"palette/angle/trait.HalfRotation.html\" title=\"trait palette::angle::HalfRotation\">HalfRotation</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;Output = T::Epsilon&gt;,</span>"],["impl&lt;S, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, T&gt;&gt; for <a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;Wp, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Lab.html\" title=\"struct palette::Lab\">Lab</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Lab.html\" title=\"struct palette::Lab\">Lab</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Okhsl.html\" title=\"struct palette::Okhsl\">Okhsl</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Okhsl.html\" title=\"struct palette::Okhsl\">Okhsl</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.OklabHue.html\" title=\"struct palette::OklabHue\">OklabHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;C, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Alpha.html\" title=\"struct palette::Alpha\">Alpha</a>&lt;C, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Alpha.html\" title=\"struct palette::Alpha\">Alpha</a>&lt;C, T&gt;<span class=\"where fmt-newline\">where\n    C: UlpsEq&lt;Epsilon = T::Epsilon&gt;,\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;S, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Hsv.html\" title=\"struct palette::Hsv\">Hsv</a>&lt;S, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Hsv.html\" title=\"struct palette::Hsv\">Hsv</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.RgbHue.html\" title=\"struct palette::RgbHue\">RgbHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Oklch.html\" title=\"struct palette::Oklch\">Oklch</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Oklch.html\" title=\"struct palette::Oklch\">Oklch</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.OklabHue.html\" title=\"struct palette::OklabHue\">OklabHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Oklab.html\" title=\"struct palette::Oklab\">Oklab</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Oklab.html\" title=\"struct palette::Oklab\">Oklab</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;Wp, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Xyz.html\" title=\"struct palette::Xyz\">Xyz</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Xyz.html\" title=\"struct palette::Xyz\">Xyz</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Okhwb.html\" title=\"struct palette::Okhwb\">Okhwb</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Okhwb.html\" title=\"struct palette::Okhwb\">Okhwb</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.OklabHue.html\" title=\"struct palette::OklabHue\">OklabHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,</span>"],["impl&lt;S, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Hwb.html\" title=\"struct palette::Hwb\">Hwb</a>&lt;S, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Hwb.html\" title=\"struct palette::Hwb\">Hwb</a>&lt;S, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/stimulus/trait.Stimulus.html\" title=\"trait palette::stimulus::Stimulus\">Stimulus</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + UlpsEq + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    <a class=\"struct\" href=\"palette/struct.RgbHue.html\" title=\"struct palette::RgbHue\">RgbHue</a>&lt;T&gt;: UlpsEq + AbsDiffEq&lt;Epsilon = T::Epsilon&gt;,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.OklabHue.html\" title=\"struct palette::OklabHue\">OklabHue</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.OklabHue.html\" title=\"struct palette::OklabHue\">OklabHue</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/angle/trait.RealAngle.html\" title=\"trait palette::angle::RealAngle\">RealAngle</a> + <a class=\"trait\" href=\"palette/angle/trait.SignedAngle.html\" title=\"trait palette::angle::SignedAngle\">SignedAngle</a> + <a class=\"trait\" href=\"palette/num/trait.Zero.html\" title=\"trait palette::num::Zero\">Zero</a> + <a class=\"trait\" href=\"palette/angle/trait.AngleEq.html\" title=\"trait palette::angle::AngleEq\">AngleEq</a>&lt;Mask = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.bool.html\">bool</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"palette/angle/trait.HalfRotation.html\" title=\"trait palette::angle::HalfRotation\">HalfRotation</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;Output = T::Epsilon&gt;,</span>"],["impl&lt;T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.LuvHue.html\" title=\"struct palette::LuvHue\">LuvHue</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.LuvHue.html\" title=\"struct palette::LuvHue\">LuvHue</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"palette/angle/trait.RealAngle.html\" title=\"trait palette::angle::RealAngle\">RealAngle</a> + <a class=\"trait\" href=\"palette/angle/trait.SignedAngle.html\" title=\"trait palette::angle::SignedAngle\">SignedAngle</a> + <a class=\"trait\" href=\"palette/num/trait.Zero.html\" title=\"trait palette::num::Zero\">Zero</a> + <a class=\"trait\" href=\"palette/angle/trait.AngleEq.html\" title=\"trait palette::angle::AngleEq\">AngleEq</a>&lt;Mask = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.bool.html\">bool</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"palette/angle/trait.HalfRotation.html\" title=\"trait palette::angle::HalfRotation\">HalfRotation</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;Output = T::Epsilon&gt;,</span>"],["impl&lt;Wp, T&gt; UlpsEq&lt;<a class=\"struct\" href=\"palette/struct.Luv.html\" title=\"struct palette::Luv\">Luv</a>&lt;Wp, T&gt;&gt; for <a class=\"struct\" href=\"palette/struct.Luv.html\" title=\"struct palette::Luv\">Luv</a>&lt;Wp, T&gt;<span class=\"where fmt-newline\">where\n    T: UlpsEq,\n    T::Epsilon: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()