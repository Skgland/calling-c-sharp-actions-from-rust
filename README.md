Calling C# Actions from Rust
============================

A small quick emonstration Project for how one might go about calling C# Actions from Rust code.

Created in response to <https://github.com/Cysharp/csbindgen/issues/43>.

Basic Principle
---------------

A non-static function is just a static function with an additional argument.
