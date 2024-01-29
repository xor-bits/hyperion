(function() {var type_impls = {
"hyperion_ring":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RingBuf%3CT,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#461-469\">source</a><a href=\"#impl-RingBuf%3CT,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, C&gt; <a class=\"struct\" href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\">RingBuf</a>&lt;T, C&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.from\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#462-468\">source</a><h4 class=\"code-header\">pub const fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.from\" class=\"fn\">from</a>(items: C, capacity: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>) -&gt; Self</h4></section></div></details>",0,"hyperion_ring::StaticRingBuf"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RingBuf%3CT,+Static%3CT,+N%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#471-475\">source</a><a href=\"#impl-RingBuf%3CT,+Static%3CT,+N%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"struct\" href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\">RingBuf</a>&lt;T, <a class=\"struct\" href=\"hyperion_ring/struct.Static.html\" title=\"struct hyperion_ring::Static\">Static</a>&lt;T, N&gt;&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#472-474\">source</a><h4 class=\"code-header\">pub const fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.new\" class=\"fn\">new</a>() -&gt; Self</h4></section></div></details>",0,"hyperion_ring::StaticRingBuf"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RingBuf%3CT,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#488-559\">source</a><a href=\"#impl-RingBuf%3CT,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, C&gt; <a class=\"struct\" href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\">RingBuf</a>&lt;T, C&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"hyperion_ring/trait.Storage.html\" title=\"trait hyperion_ring::Storage\">Storage</a>&lt;T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.push\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#494-501\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.push\" class=\"fn\">push</a>(&amp;self, val: T) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, T&gt;</h4></section></summary><div class=\"docblock\"><h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>this is a <strong>write</strong> operation, see <a href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\"><code>Self</code></a></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_arr\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#505-512\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.push_arr\" class=\"fn\">push_arr</a>&lt;const LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt;(\n    &amp;self,\n    val: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; LEN]</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; LEN]</a>&gt;</h4></section></summary><div class=\"docblock\"><h5 id=\"safety-1\"><a class=\"doc-anchor\" href=\"#safety-1\">§</a>Safety</h5>\n<p>this is a <strong>write</strong> operation, see <a href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\"><code>Self</code></a></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pop\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#516-523\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.pop\" class=\"fn\">pop</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;</h4></section></summary><div class=\"docblock\"><h5 id=\"safety-2\"><a class=\"doc-anchor\" href=\"#safety-2\">§</a>Safety</h5>\n<p>this is a <strong>read</strong> operation, see <a href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\"><code>Self</code></a></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pop_arr\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#527-537\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.pop_arr\" class=\"fn\">pop_arr</a>&lt;const LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt;(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; LEN]</a>&gt;</h4></section></summary><div class=\"docblock\"><h5 id=\"safety-3\"><a class=\"doc-anchor\" href=\"#safety-3\">§</a>Safety</h5>\n<p>this is a <strong>read</strong> operation, see <a href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\"><code>Self</code></a></p>\n</div></details></div></details>",0,"hyperion_ring::StaticRingBuf"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RingBuf%3CT,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#561-587\">source</a><a href=\"#impl-RingBuf%3CT,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, C&gt; <a class=\"struct\" href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\">RingBuf</a>&lt;T, C&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    C: <a class=\"trait\" href=\"hyperion_ring/trait.Storage.html\" title=\"trait hyperion_ring::Storage\">Storage</a>&lt;T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#568-574\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.push_slice\" class=\"fn\">push_slice</a>(&amp;self, buf: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.slice.html\">[T]</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>this is a <strong>write</strong> operation, see <a href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\"><code>Self</code></a></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pop_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#578-586\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"hyperion_ring/struct.RingBuf.html#tymethod.pop_slice\" class=\"fn\">pop_slice</a>(&amp;self, buf: &amp;mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.slice.html\">[T]</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><h5 id=\"safety-1\"><a class=\"doc-anchor\" href=\"#safety-1\">§</a>Safety</h5>\n<p>this is a <strong>read</strong> operation, see <a href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\"><code>Self</code></a></p>\n</div></details></div></details>",0,"hyperion_ring::StaticRingBuf"],["<section id=\"impl-Sync-for-RingBuf%3CT,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hyperion_ring/lib.rs.html#459\">source</a><a href=\"#impl-Sync-for-RingBuf%3CT,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>, C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"hyperion_ring/struct.RingBuf.html\" title=\"struct hyperion_ring::RingBuf\">RingBuf</a>&lt;T, C&gt;</h3></section>","Sync","hyperion_ring::StaticRingBuf"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()