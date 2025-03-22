(function() {
    var implementors = Object.fromEntries([["palette",[["impl&lt;C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"palette/blend/trait.Premultiply.html\" title=\"trait palette::blend::Premultiply\">Premultiply</a>,\n    C::<a class=\"associatedtype\" href=\"palette/blend/trait.Premultiply.html#associatedtype.Scalar\" title=\"type palette::blend::Premultiply::Scalar\">Scalar</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"palette/num/trait.Real.html\" title=\"trait palette::num::Real\">Real</a>,</div>"],["impl&lt;C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.f32.html\">f32</a>&gt; for <a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.f32.html\">f32</a>&gt; + <a class=\"trait\" href=\"palette/blend/trait.Premultiply.html\" title=\"trait palette::blend::Premultiply\">Premultiply</a>&lt;Scalar = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.f32.html\">f32</a>&gt;,</div>"],["impl&lt;C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.f64.html\">f64</a>&gt; for <a class=\"struct\" href=\"palette/blend/struct.PreAlpha.html\" title=\"struct palette::blend::PreAlpha\">PreAlpha</a>&lt;C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.f64.html\">f64</a>&gt; + <a class=\"trait\" href=\"palette/blend/trait.Premultiply.html\" title=\"trait palette::blend::Premultiply\">Premultiply</a>&lt;Scalar = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.f64.html\">f64</a>&gt;,</div>"],["impl&lt;C, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/struct.Alpha.html\" title=\"struct palette::Alpha\">Alpha</a>&lt;C, T&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;M, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/lms/struct.Lms.html\" title=\"struct palette::lms::Lms\">Lms</a>&lt;M, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;M, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/lms/struct.Lms.html\" title=\"struct palette::lms::Lms\">Lms</a>&lt;M, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/luma/struct.Luma.html\" title=\"struct palette::luma::Luma\">Luma</a>&lt;S, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/luma/struct.Luma.html\" title=\"struct palette::luma::Luma\">Luma</a>&lt;S, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;S, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/rgb/struct.Rgb.html\" title=\"struct palette::rgb::Rgb\">Rgb</a>&lt;S, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/cam16/struct.Cam16UcsJab.html\" title=\"struct palette::cam16::Cam16UcsJab\">Cam16UcsJab</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/struct.Oklab.html\" title=\"struct palette::Oklab\">Oklab</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/cam16/struct.Cam16UcsJab.html\" title=\"struct palette::cam16::Cam16UcsJab\">Cam16UcsJab</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Oklab.html\" title=\"struct palette::Oklab\">Oklab</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;T, C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Alpha.html\" title=\"struct palette::Alpha\">Alpha</a>&lt;C, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    C: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt;,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/struct.Lab.html\" title=\"struct palette::Lab\">Lab</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/struct.Luv.html\" title=\"struct palette::Luv\">Luv</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/struct.Xyz.html\" title=\"struct palette::Xyz\">Xyz</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> for <a class=\"struct\" href=\"palette/struct.Yxy.html\" title=\"struct palette::Yxy\">Yxy</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Lab.html\" title=\"struct palette::Lab\">Lab</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Luv.html\" title=\"struct palette::Luv\">Luv</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Xyz.html\" title=\"struct palette::Xyz\">Xyz</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;Wp, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a>&lt;T&gt; for <a class=\"struct\" href=\"palette/struct.Yxy.html\" title=\"struct palette::Yxy\">Yxy</a>&lt;Wp, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/arith/trait.DivAssign.html\" title=\"trait core::ops::arith::DivAssign\">DivAssign</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[14479]}