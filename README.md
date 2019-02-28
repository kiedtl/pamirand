# pamirand [![Build Status](https://travis-ci.org/lptstr/pamirand.svg?branch=master)](https://travis-ci.org/lptstr/pamirand)
A **Pa**rk-**Mi**ller style **rand**om number generator

## Installation
#### Windows
- Try using [Scoop](https://scoop.sh). <br>
  - First, add the LPTSTR-Scoop bucket (if you haven't already):
    ```
    $ scoop bucket add lptstr https://github.com/lptstr/lptstr-scoop
    ```
  - Now install the application:
    ```
    $ scoop install pamirand
    ```

- Or, if you don't want to use a package manager, just download `painter.exe` from the latest release in the latest [releases section](https://github.com/lptstr/painter/releases).

#### Other platforms
- Build this project from source:
  - First, make sure that you have Cargo and RustC v1.30.0+ installed:
  ```
  $ cargo -vV
  cargo 1.32.0 (8610973aa 2019-01-02)
  release: 1.32.0
  commit-hash: 8610973aaf48615ba7dc9a38a9a2795ba6f36a31
  commit-date: 2019-01-02
  $ rustc --version --verbose
  rustc 1.32.0 (9fda7c223 2019-01-16)
  binary: rustc
  commit-hash: 9fda7c2237db910e41d6a712e9a2139b352e558b
  commit-date: 2019-01-16
  host: x86_64-pc-windows-gnu
  release: 1.32.0
  LLVM version: 8.0
  ```
  - Then clone this repository:
  ```
  $ git clone https://github.com/lptstr/pamirand
  ```
  - Build with Cargo:
  ```
  $ cargo run --verbose --release
  ```
  
## Usage 
```
$ ./pamirand [seed]
```
For example, to get one random number with the seed `678`, try:
```
$ ./pamirand 678
3272773
```
Or, to get 10 random numbers, starting from the seed 2846, try this PowerShell script:
```powershell
$i = 2846
0..9 | % {
  $i = ./pamirand $i
  $i
}
```
Output:
```
137379266
5047150
965325539
1128920463
1722126848
1840588085
1334007351
1571684826
495954630
78247974
```

## Inspiration
- [park-miller](https://github.com/sindresorhus/park-miller) by **@sindresorhus**
