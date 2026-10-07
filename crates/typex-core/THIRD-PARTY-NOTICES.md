# Third-party notices

TyPeX's bounded mathematical character tables and single-line fraction rule
are informed by UniTeX, commit `6e6294bdfb6101eb6e3dc440e1b9cb5a1a1efdc0`:
`src/impl/unicode/{fixed,unary,binary,proper,unicode-table}.ts`.
The parser, lossless recovery, scalar indexing and state machine here are a
separate Rust implementation of `typex.windows.demo/2-route-a`; they do not
inherit UniTeX's fallback, normalization, UTF-16 length or full command set.
Include this notice with distributions using these tables/rules.

The following is reproduced from UniTeX's LICENSE:

MIT License

Copyright (c) 2023 by kokic <kokicplus@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
