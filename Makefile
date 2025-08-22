.PHONY: render
render: build
	 ./target/release/raytracer > image.ppm

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	rm -rf ./target
