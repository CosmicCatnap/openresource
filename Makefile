prog :=openresource

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build-api:
	cd backend && cargo build $(release)

install-api:
	systemctl stop openresource
	cp backend/target/$(target)/$(prog) ~/server
	systemctl start openresource

build-web:
	cd frontend && npm install

install-web:
	cp -r frontend/out/* ~/public

run-dev:
	~/server && cd frontend && npm run dev

run-web:
	cd frontend && npm run dev

run-api:
	~/server

all: build install

help:
	@echo "usage: make $(prog) [debug=1]"
