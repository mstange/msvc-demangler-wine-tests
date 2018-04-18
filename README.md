This repository tests the [msvc-demangler](https://crates.io/crates/msvc-demangler) crate using tests imported from WINE, specifically from
[the `test_demangle()` function in `dlls/msvcrt/tests/cpp.c`](https://github.com/wine-mirror/wine/blob/813ab925abd45d48c811898028fddc1047b0c250/dlls/msvcrt/tests/cpp.c#L1142).

The tests' license is incompatible with the license of the msvc-demangler repo,
which is why these tests live in a separate repository.

Some of the tests are still failing. They've also been adjusted to not expect things
like `__thiscall`, `__ptr64` etc. which msvc-demangler doesn't support yet.
