# VMware vCenter CVE-2021-21972 checker
Scanner for VMware vCenter Vulnerability

Disclaimer: This is for Educational Purposes only!

[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

## References for CVE:

  [1 - tenable article ](https://www.tenable.com/blog/cve-2021-21972-vmware-vcenter-server-remote-code-execution-vulnerability)

  [2 - rapid7 blog ](https://blog.rapid7.com/2021/02/24/vmware-vcenter-server-cve-2021-21972-remote-code-execution-vulnerability-what-you-need-to-know/)

## Workarounds

  https://kb.vmware.com/s/article/82374
  
  https://kb.vmware.com/s/article/76372

## Usage:

    $ ./vcenter-rce-2021-21972 [OPTIONS]

## FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

## OPTIONS:

    -f, --file <filename>             File containing a list of IPs
    -H, --host <Host IP or domain>    Host IP or Domain to be checked for CVE

## Installation

First you must install Rust package on your GNU/Linux distribution, MacOS ($$$$) or Windows (blergh!)

  https://www.rust-lang.org/tools/install

## Compiling 

It is really easy to compile. Just run :

    $ cargo build --release

Then you'll have the release ready in your <repo_downloaded_dir>/target/release/

That's all folks! Thank you very much.