# SWC Memory Leak Repro

Run:

```
cargo +nightly miri run
```

<details>
<summary>Output:</summary>

```
error: memory leaked: alloc6124 (Rust heap, size: 48, align: 8), allocated here:
   --> /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs:100:9
    |
100 |         __rust_alloc(layout.size(), layout.align())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: BACKTRACE:
    = note: inside `std::alloc::alloc` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs:243:9: 243:39
    = note: inside `alloc::alloc::exchange_malloc` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs:332:11: 332:34
    = note: inside `std::boxed::Box::<triomphe::arc::ArcInner<hstr::dynamic::Entry>>::new` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:260:9: 260:20
    = note: inside `triomphe::arc::Arc::<hstr::dynamic::Entry>::new` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/triomphe-0.1.13/src/arc.rs:81:33: 84:11
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:169:21: 174:23
    = note: inside `hashbrown::map::RawEntryMut::<'_, triomphe::arc::Arc<hstr::dynamic::Entry>, (), std::hash::BuildHasherDefault<hstr::dynamic::EntryHasher>>::or_insert_with::<{closure@<&mut hstr::dynamic::AtomStore as hstr::dynamic::Storage>::insert_entry::{closure#1}}>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs:3430:30: 3430:39
    = note: inside `<&mut hstr::dynamic::AtomStore as hstr::dynamic::Storage>::insert_entry` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:163:26: 177:15
    = note: inside `hstr::dynamic::new_atom::<&mut hstr::dynamic::AtomStore>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:142:17: 142:49
    = note: inside `hstr::dynamic::AtomStore::atom::<'_, std::borrow::Cow<'_, str>>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:119:9: 119:36
    = note: inside `swc_atoms::AtomStore::atom::<'_, std::borrow::Cow<'_, str>>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_atoms-0.6.7/src/lib.rs:230:14: 230:28
    = note: inside `swc_atoms::AtomStoreCell::atom::<'_, &str>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_atoms-0.6.7/src/lib.rs:247:18: 247:41
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:769:42: 769:57
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:897:17: 897:58
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::with_buf::<{closure@swc_ecma_parser::lexer::Lexer<'_>::read_word_as_str_with<{closure@swc_ecma_parser::lexer::Lexer<'_>::read_word_with::{closure#0}}, swc_ecma_parser::token::Word>::{closure#0}}, (swc_ecma_parser::token::Word, bool)>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:175:9: 175:27
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::read_word_as_str_with::<{closure@swc_ecma_parser::lexer::Lexer<'_>::read_word_with::{closure#0}}, swc_ecma_parser::token::Word>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:799:9: 910:11
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::read_word_with` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:762:34: 770:11
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/table.rs:89:5: 96:7
    = note: inside `<{closure@swc_ecma_parser::lexer::table::L_C::{closure#0}} as std::ops::FnOnce<(&mut swc_ecma_parser::lexer::Lexer<'_>,)>>::call_once - shim` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5: 250:71
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::read_token` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:188:30: 188:43
    = note: inside `swc_ecma_parser::lexer::state::<impl swc_ecma_parser::lexer::Lexer<'_>>::next_token` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/state.rs:337:9: 337:26
    = note: inside `swc_ecma_parser::lexer::state::<impl std::iter::Iterator for swc_ecma_parser::lexer::Lexer<'_>>::next` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/state.rs:347:19: 347:46
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/input.rs:377:52: 377:68
    = note: inside `std::option::Option::<swc_ecma_parser::token::TokenAndSpan>::or_else::<{closure@swc_ecma_parser::input::Buffer<swc_ecma_parser::lexer::Lexer<'_>>::cur::{closure#0}}>` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:1543:21: 1543:24
    = note: inside `swc_ecma_parser::input::Buffer::<swc_ecma_parser::lexer::Lexer<'_>>::cur` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/input.rs:377:24: 377:69
    = note: inside `swc_ecma_parser::input::Buffer::<swc_ecma_parser::lexer::Lexer<'_>>::cur_pos` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/input.rs:419:17: 419:27
    = note: inside `swc_ecma_parser::Parser::<swc_ecma_parser::lexer::Lexer<'_>>::parse_program` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/macros.rs:300:9: 300:27
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lib.rs:515:47: 515:64
    = note: inside `swc_ecma_parser::with_file_parser::<swc_ecma_ast::Program, {closure@swc_ecma_parser::parse_file_as_program::{closure#0}}>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lib.rs:479:15: 479:25
    = note: inside `swc_ecma_parser::parse_file_as_program` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lib.rs:503:13: 503:85
note: inside `main`
   --> src/main.rs:17:19
    |
17  |       let program = swc_ecma_parser::parse_file_as_program(
    |  ___________________^
18  | |         &src,
19  | |         swc_ecma_parser::Syntax::Es(Default::default()),
20  | |         swc_ecma_ast::EsVersion::EsNext,
21  | |         None,
22  | |         &mut errs,
23  | |     )
    | |_____^

error: memory leaked: alloc5944 (Rust heap, size: 7, align: 1), allocated here:
   --> /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs:100:9
    |
100 |         __rust_alloc(layout.size(), layout.align())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: BACKTRACE:
    = note: inside `std::alloc::alloc` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs:100:9: 100:52
    = note: inside `std::alloc::Global::alloc_impl` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs:183:73: 183:86
    = note: inside `<std::alloc::Global as std::alloc::Allocator>::allocate` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs:243:9: 243:39
    = note: inside `alloc::raw_vec::RawVec::<u8>::try_allocate_in` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:230:45: 230:67
    = note: inside `alloc::raw_vec::RawVec::<u8>::with_capacity_in` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:158:15: 158:79
    = note: inside `std::vec::Vec::<u8>::with_capacity_in` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:699:20: 699:61
    = note: inside `<u8 as std::slice::hack::ConvertVec>::to_vec::<std::alloc::Global>` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:162:25: 162:62
    = note: inside `std::slice::hack::to_vec::<u8, std::alloc::Global>` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:111:9: 111:28
    = note: inside `std::slice::<impl [u8]>::to_vec_in::<std::alloc::Global>` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:462:9: 462:34
    = note: inside `std::slice::<impl [u8]>::to_vec` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:437:9: 437:31
    = note: inside `std::slice::<impl std::borrow::ToOwned for [u8]>::to_owned` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:844:9: 844:22
    = note: inside `std::str::<impl std::borrow::ToOwned for str>::to_owned` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/str.rs:212:46: 212:72
    = note: inside `std::borrow::Cow::<'_, str>::into_owned` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/borrow.rs:325:35: 325:54
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:170:33: 170:50
    = note: inside `hashbrown::map::RawEntryMut::<'_, triomphe::arc::Arc<hstr::dynamic::Entry>, (), std::hash::BuildHasherDefault<hstr::dynamic::EntryHasher>>::or_insert_with::<{closure@<&mut hstr::dynamic::AtomStore as hstr::dynamic::Storage>::insert_entry::{closure#1}}>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs:3430:30: 3430:39
    = note: inside `<&mut hstr::dynamic::AtomStore as hstr::dynamic::Storage>::insert_entry` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:163:26: 177:15
    = note: inside `hstr::dynamic::new_atom::<&mut hstr::dynamic::AtomStore>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:142:17: 142:49
    = note: inside `hstr::dynamic::AtomStore::atom::<'_, std::borrow::Cow<'_, str>>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hstr-0.2.10/src/dynamic.rs:119:9: 119:36
    = note: inside `swc_atoms::AtomStore::atom::<'_, std::borrow::Cow<'_, str>>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_atoms-0.6.7/src/lib.rs:230:14: 230:28
    = note: inside `swc_atoms::AtomStoreCell::atom::<'_, &str>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_atoms-0.6.7/src/lib.rs:247:18: 247:41
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:769:42: 769:57
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:897:17: 897:58
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::with_buf::<{closure@swc_ecma_parser::lexer::Lexer<'_>::read_word_as_str_with<{closure@swc_ecma_parser::lexer::Lexer<'_>::read_word_with::{closure#0}}, swc_ecma_parser::token::Word>::{closure#0}}, (swc_ecma_parser::token::Word, bool)>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:175:9: 175:27
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::read_word_as_str_with::<{closure@swc_ecma_parser::lexer::Lexer<'_>::read_word_with::{closure#0}}, swc_ecma_parser::token::Word>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:799:9: 910:11
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::read_word_with` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:762:34: 770:11
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/table.rs:89:5: 96:7
    = note: inside `<{closure@swc_ecma_parser::lexer::table::L_C::{closure#0}} as std::ops::FnOnce<(&mut swc_ecma_parser::lexer::Lexer<'_>,)>>::call_once - shim` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5: 250:71
    = note: inside `swc_ecma_parser::lexer::Lexer::<'_>::read_token` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/mod.rs:188:30: 188:43
    = note: inside `swc_ecma_parser::lexer::state::<impl swc_ecma_parser::lexer::Lexer<'_>>::next_token` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/state.rs:337:9: 337:26
    = note: inside `swc_ecma_parser::lexer::state::<impl std::iter::Iterator for swc_ecma_parser::lexer::Lexer<'_>>::next` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lexer/state.rs:347:19: 347:46
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/input.rs:377:52: 377:68
    = note: inside `std::option::Option::<swc_ecma_parser::token::TokenAndSpan>::or_else::<{closure@swc_ecma_parser::input::Buffer<swc_ecma_parser::lexer::Lexer<'_>>::cur::{closure#0}}>` at /home/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:1543:21: 1543:24
    = note: inside `swc_ecma_parser::input::Buffer::<swc_ecma_parser::lexer::Lexer<'_>>::cur` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/input.rs:377:24: 377:69
    = note: inside `swc_ecma_parser::input::Buffer::<swc_ecma_parser::lexer::Lexer<'_>>::cur_pos` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/input.rs:419:17: 419:27
    = note: inside `swc_ecma_parser::Parser::<swc_ecma_parser::lexer::Lexer<'_>>::parse_program` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/parser/macros.rs:300:9: 300:27
    = note: inside closure at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lib.rs:515:47: 515:64
    = note: inside `swc_ecma_parser::with_file_parser::<swc_ecma_ast::Program, {closure@swc_ecma_parser::parse_file_as_program::{closure#0}}>` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lib.rs:479:15: 479:25
    = note: inside `swc_ecma_parser::parse_file_as_program` at /home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_ecma_parser-0.146.8/src/lib.rs:503:13: 503:85
note: inside `main`
   --> src/main.rs:17:19
    |
17  |       let program = swc_ecma_parser::parse_file_as_program(
    |  ___________________^
18  | |         &src,
19  | |         swc_ecma_parser::Syntax::Es(Default::default()),
20  | |         swc_ecma_ast::EsVersion::EsNext,
21  | |         None,
22  | |         &mut errs,
23  | |     )
    | |_____^
```

</details>
