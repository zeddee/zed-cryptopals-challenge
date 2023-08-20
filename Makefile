
default: build

build:
	cargo build
test:
	cargo test
docs:
	rm -rf docs
	cargo doc --target-dir=dist-docs
	echo "<meta http-equiv=\"refresh\" content=\"0; url=zed_cryptopals_challenge\">" > dist-docs/doc/index.html
	mv dist-docs/doc docs
	rm -rf dist-docs
