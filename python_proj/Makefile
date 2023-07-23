.PHONY:  package
SHELL := /bin/bash
PROJ_NAME=<name>

init:
	python -m venv .venv
	source .venv/bin/activate \
	&& python -m pip install --upgrade pip \
	&& pip install -r requirements-dev.txt

run:
	source .venv/bin/activate \
	&& python -m $(PROJ_NAME)

clean:
	rm -rf .venv

package:
	rm -rf .$(PROJ_NAME)
	mkdir .$(PROJ_NAME)
	cp .env .$(PROJ_NAME)/
	cp .pre-commit-config.yaml .$(PROJ_NAME)/
	cp ASSIGNMENT.md .$(PROJ_NAME)/
	cp docker-compose.yml .$(PROJ_NAME)/
	cp LICENSE.md .$(PROJ_NAME)/
	cp Makefile .$(PROJ_NAME)/
	cp README.md .$(PROJ_NAME)/
	cp poetry.lock .$(PROJ_NAME)/
	cp pyproject.toml .$(PROJ_NAME)/
	cp setup.cfg .$(PROJ_NAME)/
	find ./$(PACKAGE_NAME) -name '*.py' -exec cp --parents "{}" .$(PROJ_NAME)/ \;
	cd .$(PROJ_NAME) && zip -r ../assignment.zip .


pre_commit:
	source .venv/bin/activate \
	&& pre-commit run --all
