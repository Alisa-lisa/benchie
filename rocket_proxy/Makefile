.PHONY: watch
watch:
	cargo watch -x run

start:
	docker build -t proxy . && docker run -p 8089:8089 --network=foo proxy
