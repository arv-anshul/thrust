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

### Format: `url`

```bash
$ cargo run -q -- pandas,numpy,scikit url
https://img.shields.io/badge/GeoPandas-139C5A?logo=geopandas&logoColor=fff
https://img.shields.io/badge/pandas-150458?logo=pandas&logoColor=fff

https://img.shields.io/badge/NumPy-013243?logo=numpy&logoColor=fff

https://img.shields.io/badge/scikit--learn-F7931E?logo=scikitlearn&logoColor=fff
```

### Format: `md` (default)

```bash
$ cargo run -q -- python,polars  # (optional) add `md` as second argument
![MicroPython](https://img.shields.io/badge/MicroPython-2B2728?logo=micropython&logoColor=fff)
![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=fff)
![PythonAnywhere](https://img.shields.io/badge/PythonAnywhere-1D9FD7?logo=pythonanywhere&logoColor=fff)

![Polars](https://img.shields.io/badge/Polars-CD792C?logo=polars&logoColor=fff)
```

### Format: `html`

```bash
$ cargo run -q -- rust,zed,python html
<img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=fff" alt="Rust">
<img src="https://img.shields.io/badge/RustDesk-024EFF?logo=rustdesk&logoColor=fff" alt="RustDesk">
<img src="https://img.shields.io/badge/Trusted%20Shops-FFDC0F?logo=trustedshops&logoColor=fff" alt="Trusted Shops">
<img src="https://img.shields.io/badge/Trustpilot-00B67A?logo=trustpilot&logoColor=fff" alt="Trustpilot">
<img src="https://img.shields.io/badge/VirusTotal-394EFF?logo=virustotal&logoColor=fff" alt="VirusTotal">

<img src="https://img.shields.io/badge/Zed%20Industries-084CCF?logo=zedindustries&logoColor=fff" alt="Zed Industries">
```
