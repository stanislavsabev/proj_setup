# Python project

Template for python CI/CD project setup.

Using `pytest`, `flake8`, `mypy`, `tox` and `GitHub Actions` workflows.

---

[![Tests icon here!](https://github.com/<profile>/<project>/workflows/tests/badge.svg)](https://github.com/<profile>/<project>/actions/workflows/tests.yaml)

---

## Setup

### Create project directory.

Download this repo and copy the `python_proj/` directory.

```bash
git clone https://github.com/stanislavsabev/proj_setup.git
```

```bash
cp -R proj_setup.git/python_proj myproject

cd myproject
```

Setup virtual environment and activate it.

```bash
python -m venv .venv
```

- Linux / macOS
```bash
source ./.venv/bin/activate
```
- Windows
```powershell
.\.venv\Scripts\activate.bat
```

Install requirements.

```bash
pip install -r requirements.txt
pip install -r requirements-dev.txt
```

### Install locally

Change the package name - optional, but recommended.

- Rename in `src/` directory.
```bash
mv src/python_proj src/myproject
```

- Change all mentions of `python_proj` in 

```text
  pyproject.toml
  setup.cfg
  setup.py
  tests/test_start.py 
  example.py
```

Install as editable package

```bash
pip install -e .
```

## Local Usage

Run `pytest`, `flake8` and `mypy` from the command line...

```bash
$ pytest
...
tests\test_start.py .
[100%]

----------- coverage: platform win32, python 3.9.2-final-0 -----------
Name                             Stmts   Miss  Cover
----------------------------------------------------
src\python_proj\__init__.py       0      0   100%
src\python_proj\start.py          2      0   100%
----------------------------------------------------
TOTAL                                2      0   100%


========================================================================= 1 passed in 0.11s ===
```
```bash
$ mypy src
Success: no issues found in 6 source files
```
```bash
$ flake8 src tests
0
```
.. or all at once, with `tox`

```bash
$ tox -e py37,mypy,flake8 # Change 'py37' based on your python version.
...
______________________________________________________________________________ summary ___
  py37: commands succeeded
  mypy: commands succeeded
  flake8: commands succeeded
  congratulations :)
```

## GitHub Actions

Create [GitHub](https://github.com) repo and push your changes. This will trigger the CI/CD pipeline.

For more information see the GitHub Actions  [documentation](https://docs.github.com/en/actions/using-workflows).
