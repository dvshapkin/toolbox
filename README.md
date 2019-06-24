# toolbox::{vfs, alg}

[![Crates.io](https://img.shields.io/crates/l/toolbox.svg)](https://github.com/torkve/shotwellvfs/blob/master/LICENSE)

This project is no longer supported (**deprecated**).\
It divided into two independent parts:
 - if you need *virtual file system*, please use [**vfsys**](https://crates.io/crates/vfsys) crate.
 - if you need *algorithms & data structures*, please use [**alg_ds**](https://crates.io/crates/alg_ds) crate.

### vfs
Virtual file system for convenient work with relative paths.

<small>Tested on linux, windows.</small>

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
