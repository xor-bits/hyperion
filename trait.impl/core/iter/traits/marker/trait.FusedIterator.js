(function() {var implementors = {
"either":[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<span class=\"where fmt-newline\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>&lt;Item = L::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</span>"]],
"itertools":[["impl&lt;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.WithPosition.html\" title=\"struct itertools::structs::WithPosition\">WithPosition</a>&lt;I&gt;"],["impl&lt;I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.PadUsing.html\" title=\"struct itertools::structs::PadUsing\">PadUsing</a>&lt;I, F&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>) -&gt; I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>,</span>"],["impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ZipLongest.html\" title=\"struct itertools::structs::ZipLongest\">ZipLongest</a>&lt;T, U&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</span>"],["impl&lt;I, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.TupleWindows.html\" title=\"struct itertools::structs::TupleWindows\">TupleWindows</a>&lt;I, T&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>&lt;Item = T::Item&gt;,\n    T: <a class=\"trait\" href=\"itertools/traits/trait.HomogeneousTuple.html\" title=\"trait itertools::traits::HomogeneousTuple\">HomogeneousTuple</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    T::Item: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;I, J&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.Interleave.html\" title=\"struct itertools::structs::Interleave\">Interleave</a>&lt;I, J&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    J: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</span>"],["impl&lt;I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.Update.html\" title=\"struct itertools::structs::Update\">Update</a>&lt;I, F&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;mut I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>),</span>"],["impl&lt;I, J&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.InterleaveShortest.html\" title=\"struct itertools::structs::InterleaveShortest\">InterleaveShortest</a>&lt;I, J&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    J: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>&lt;Item = I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</span>"],["impl&lt;I, J, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.MergeBy.html\" title=\"struct itertools::structs::MergeBy\">MergeBy</a>&lt;I, J, F&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    J: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>&lt;Item = I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,\n    F: MergePredicate&lt;I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</span>"],["impl&lt;A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.RepeatN.html\" title=\"struct itertools::structs::RepeatN\">RepeatN</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.Positions.html\" title=\"struct itertools::structs::Positions\">Positions</a>&lt;I, F&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,</span>"],["impl&lt;I, ElemF&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.IntersperseWith.html\" title=\"struct itertools::structs::IntersperseWith\">IntersperseWith</a>&lt;I, ElemF&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    ElemF: IntersperseElement&lt;I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</span>"],["impl&lt;I, J&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.Product.html\" title=\"struct itertools::structs::Product\">Product</a>&lt;I, J&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    J: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl&lt;I, F, T, U, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.FilterMapOk.html\" title=\"struct itertools::structs::FilterMapOk\">FilterMapOk</a>&lt;I, F&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>&lt;Item = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, E&gt;&gt;,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(T) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;U&gt;,</span>"],["impl&lt;I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.TakeWhileInclusive.html\" title=\"struct itertools::structs::TakeWhileInclusive\">TakeWhileInclusive</a>&lt;'_, I, F&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,</span>"],["impl&lt;I, F, T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.FilterOk.html\" title=\"struct itertools::structs::FilterOk\">FilterOk</a>&lt;I, F&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>&lt;Item = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, E&gt;&gt;,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;T</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,</span>"],["impl&lt;I, T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.FlattenOk.html\" title=\"struct itertools::structs::FlattenOk\">FlattenOk</a>&lt;I, T, E&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>&lt;Item = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, E&gt;&gt;,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>,</span>"],["impl&lt;I, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.TupleCombinations.html\" title=\"struct itertools::structs::TupleCombinations\">TupleCombinations</a>&lt;I, T&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a>,\n    T: HasCombination&lt;I&gt;,</span>"]],
"rand":[["impl&lt;D, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"rand/distributions/struct.DistIter.html\" title=\"struct rand::distributions::DistIter\">DistIter</a>&lt;D, R, T&gt;<span class=\"where fmt-newline\">where\n    D: <a class=\"trait\" href=\"rand/distributions/trait.Distribution.html\" title=\"trait rand::distributions::Distribution\">Distribution</a>&lt;T&gt;,\n    R: <a class=\"trait\" href=\"rand/trait.Rng.html\" title=\"trait rand::Rng\">Rng</a>,</span>"]],
"smallvec":[["impl&lt;A: <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"smallvec/struct.IntoIter.html\" title=\"struct smallvec::IntoIter\">IntoIter</a>&lt;A&gt;"],["impl&lt;'a, T: <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"smallvec/struct.Drain.html\" title=\"struct smallvec::Drain\">Drain</a>&lt;'a, T&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()