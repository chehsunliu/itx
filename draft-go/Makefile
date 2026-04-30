BIN_DIR := bin

.PHONY: all build build-backend build-worker clean

all: build

build: build-backend build-worker

build-backend:
	go build -o $(BIN_DIR)/draft-backend ./draft-backend/cmd/draft-backend

build-worker:
	go build -o $(BIN_DIR)/draft-worker ./draft-worker/cmd/draft-worker

clean:
	rm -rf $(BIN_DIR)
