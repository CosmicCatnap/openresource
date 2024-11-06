MAKEFLAGS=-j 2
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

build-backend:
	cd backend && cargo build $(release)

build-frontend:
	cd frontend && npm install && npm run build

build: build-backend build-frontend

dev-frontend:
	cd frontend && npm run dev

dev-backend:
	cd backend && cargo run

dev: dev-backend dev-frontend

install-backend: build-backend
	systemctl stop openresource
	cp backend/target/$(target)/$(prog) ~/server
	systemctl start openresource

install-frontend: build-frontend
	cp -r frontend/out/* ~/public

install: install-backend install-frontend

generate-api: npx @openapitools/openapi-generator-cli generate -i .\openapi.yaml -g rust-axum -o ./api

all: build

help:
	@echo "usage: make $(prog) [debug=1]"
