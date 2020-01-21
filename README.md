# toolbox::{alg, ds, vfs}

[![Crates.io](https://img.shields.io/crates/l/toolbox.svg)](https://github.com/dvshapkin/toolbox/LICENSE)

<small>Tested on linux, windows.</small>

Further support for this library is not planned. 
At the moment, it is a "wrapper" of its two dependencies: 
- [alg_ds](https://crates.io/crates/alg_ds)
- [vfsys](https://crates.io/crates/vfsys)

### alg
*Search algorithms:*
<table>
<tr><th>name</th><th>complexity</th></tr>
<tr><td>max</td> <td>O(n)</td></tr>
<tr><td>min</td> <td>O(n)</td></tr>
<tr><td>binary</td> <td>O(log<sub><small>2</small></sub>n)</td></tr>
</table>

*Sort algorithms:*
<table>
<tr><th>name</th><th>complexity</th></tr>
<tr><td>selection</td> <td>O(n<sup><small>2</small></sup>)</td></tr>
<tr><td>quick</td> <td>O(n log<sub><small>2</small></sub>n)</td></tr>
</table>

### ds
**Matrix** data structure (two-dimensional array)

### vfs
Virtual file system for convenient work with relative paths.