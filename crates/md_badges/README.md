# MD Badges

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=fff" alt="Rust">
  <img src="https://img.shields.io/badge/Zed%20Industries-084CCF?logo=zedindustries&logoColor=fff" alt="Zed Industries">
</p>

CLI tool to access the simpleicon badges easily, (written in rust :crab:).

> \[!IMPORTANT\]
>
> This is my practice project in rust. Do not take seriously. (धन्यवाद)

Already wrote this project in Python, [see](https://gist.github.com/arv-anshul/f4ccfd9258f24ffa9769dfca9b9e091b).

## Usage

Install `md_badges` binary in your system using below command:

```bash
cargo install --git https://github.com/arv-anshul/thrust md_badges
```

Check whether it is installed correctly in `$PATH`:

```bash
md_badges --version
```

Different output format of badges:

- format: `markdown` _(default)_

```bash
$ md_badges python polars
![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=fff)
![Polars](https://img.shields.io/badge/Polars-CD792C?logo=polars&logoColor=fff)
```

- format: `url`

```bash
$ md_badges --format url pandas numpy scikitlearn
https://img.shields.io/badge/pandas-150458?logo=pandas&logoColor=fff
https://img.shields.io/badge/NumPy-013243?logo=numpy&logoColor=fff
https://img.shields.io/badge/scikit--learn-F7931E?logo=scikitlearn&logoColor=fff
```

- format: `html`

```bash
$ md_badges --format html rust zedindustries python
<img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=fff" alt="Rust">
<img src="https://img.shields.io/badge/Zed%20Industries-084CCF?logo=zedindustries&logoColor=fff" alt="Zed Industries">
<img src="https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=fff" alt="Python">
```

## Features

- [x] Use `clap` crate and impleament all the previous fetaures.
- [x] Add `--fetch-icons` flag which fetch icons from simpleicons github and dump fetched data at defined file path.
- [ ] Add `--sorted-output` flag to return sorted output by `slugs`.
- [ ] Add `--online` flag which fetch icons data from github and return output on this without storing data in file.
