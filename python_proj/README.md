# python_starter

Standard template for a python project with `tox` and GitHub Actions workflows.

---

[![tests](https://github.com/stanislavsabev/python_starter/workflows/tests/badge.svg)](https://github.com/stanislavsabev/python_starter/actions/workflows/tests.yaml)

---

## Setup

Create project directory.

```text
mkdir myproject
cd myproject
```

Pull starter repo.

```text
git pull https://github.com/stanislavsabev/python_starter.git
```

Setup virtual environment and activate it.

```text
python -m venv venv

source ./venv/bin/activate # Linux / Mac
.\venv\Scripts\activate.bat # Windows
```

Install requirements.

```text
pip install -r requirements.txt
pip install -r requirements-dev.txt
```

**NOTE:** At this point you can rename the package.

1. Rename the `src/python_starter` directory.

2. Change all mentions of `python_starter` in pyproject.toml, setup.cfg, setup.py, tests/test_start.py and (optional) in example.py.

Install the package in edit mode

```text
pip install -e .
```

## Local Usage

Run `pytest`, `flake8` and `mypy` from a command line...

```text
$ pytest
...
tests\test_start.py .
[100%]

----------- coverage: platform win32, python 3.9.2-final-0 -----------
Name                             Stmts   Miss  Cover
----------------------------------------------------
src\python_starter\__init__.py       0      0   100%
src\python_starter\start.py          2      0   100%
----------------------------------------------------
TOTAL                                2      0   100%


========================================================================= 1 passed in 0.11s ===
```

```text
$ mypy src
Success: no issues found in 6 source files

$ flake8 src tests
0
```

.. or  using `tox`

```text
$ tox -e py37,mypy,flake8 # Change 'py37' based on your python version.
...
______________________________________________________________________________ summary ___
  py37: commands succeeded
  mypy: commands succeeded
  flake8: commands succeeded
  congratulations :)
```

## Remote usage with `GitHub Actions`

Create [GitHub](https://github.com) repo and push your changes.

For more information see the GitHub Actions  [documentation](https://docs.github.com/en/actions/using-workflows).
