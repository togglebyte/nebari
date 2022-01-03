(function() {var implementors = {};
implementors["nebari"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/io/fs/struct.StdFileManager.html\" title=\"struct nebari::io::fs::StdFileManager\">StdFileManager</a>","synthetic":false,"types":["nebari::io::fs::StdFileManager"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/io/memory/struct.MemoryFileManager.html\" title=\"struct nebari::io::memory::MemoryFileManager\">MemoryFileManager</a>","synthetic":false,"types":["nebari::io::memory::MemoryFileManager"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/io/struct.PathIds.html\" title=\"struct nebari::io::PathIds\">PathIds</a>","synthetic":false,"types":["nebari::io::PathIds"]},{"text":"impl&lt;File:&nbsp;<a class=\"trait\" href=\"nebari/io/trait.ManagedFile.html\" title=\"trait nebari::io::ManagedFile\">ManagedFile</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/struct.ThreadPool.html\" title=\"struct nebari::ThreadPool\">ThreadPool</a>&lt;File&gt;","synthetic":false,"types":["nebari::roots::ThreadPool"]},{"text":"impl&lt;Index, ReducedIndex&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/tree/struct.BTreeEntry.html\" title=\"struct nebari::tree::BTreeEntry\">BTreeEntry</a>&lt;Index, ReducedIndex&gt;","synthetic":false,"types":["nebari::tree::btree_entry::BTreeEntry"]},{"text":"impl&lt;Root:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"nebari/tree/trait.Root.html\" title=\"trait nebari::tree::Root\">Root</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/tree/struct.State.html\" title=\"struct nebari::tree::State\">State</a>&lt;Root&gt;","synthetic":false,"types":["nebari::tree::state::State"]},{"text":"impl&lt;Root:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"nebari/tree/trait.Root.html\" title=\"trait nebari::tree::Root\">Root</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/tree/struct.ActiveState.html\" title=\"struct nebari::tree::ActiveState\">ActiveState</a>&lt;Root&gt;","synthetic":false,"types":["nebari::tree::state::ActiveState"]},{"text":"impl&lt;Index&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/tree/struct.UnversionedTreeRoot.html\" title=\"struct nebari::tree::UnversionedTreeRoot\">UnversionedTreeRoot</a>&lt;Index&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Index: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"nebari/tree/trait.Reducer.html\" title=\"trait nebari::tree::Reducer\">Reducer</a>&lt;Index&gt; + <a class=\"trait\" href=\"nebari/tree/trait.EmbeddedIndex.html\" title=\"trait nebari::tree::EmbeddedIndex\">EmbeddedIndex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + 'static,&nbsp;</span>","synthetic":false,"types":["nebari::tree::unversioned::UnversionedTreeRoot"]},{"text":"impl&lt;EmbeddedIndex&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/tree/struct.VersionedTreeRoot.html\" title=\"struct nebari::tree::VersionedTreeRoot\">VersionedTreeRoot</a>&lt;EmbeddedIndex&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;EmbeddedIndex: <a class=\"trait\" href=\"nebari/tree/trait.EmbeddedIndex.html\" title=\"trait nebari::tree::EmbeddedIndex\">EmbeddedIndex</a> + <a class=\"trait\" href=\"nebari/tree/trait.Reducer.html\" title=\"trait nebari::tree::Reducer\">Reducer</a>&lt;EmbeddedIndex&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + 'static,&nbsp;</span>","synthetic":false,"types":["nebari::tree::versioned::VersionedTreeRoot"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/struct.Buffer.html\" title=\"struct nebari::Buffer\">Buffer</a>&lt;'a&gt;","synthetic":false,"types":["nebari::buffer::Buffer"]},{"text":"impl&lt;M:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"nebari/io/trait.FileManager.html\" title=\"trait nebari::io::FileManager\">FileManager</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"nebari/struct.Context.html\" title=\"struct nebari::Context\">Context</a>&lt;M&gt;","synthetic":false,"types":["nebari::context::Context"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()