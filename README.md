This repository tests the [msvc-demangler](https://crates.io/crates/msvc-demangler) crate using tests imported from WINE, specifically from
[the `test_demangle()` function in `dlls/msvcrt/tests/cpp.c`](https://github.com/wine-mirror/wine/blob/813ab925abd45d48c811898028fddc1047b0c250/dlls/msvcrt/tests/cpp.c#L1142).

The tests' license is incompatible with the license of the msvc-demangler repo,
which is why these tests live in a separate repository.

Some tests are commented out because msvc-demangler doesn't pass them yet. They
have been annotated with the reason why they're failing.

Run tests using

```bash
$ cargo test
```
