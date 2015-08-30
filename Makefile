test:
	docker run --rm -t \
		-v $(PWD):/source \
		-w /source \
		arnau/rust \
		cargo test

shell:
	docker run --rm -it \
		--volumes-from vault \
		-v $(PWD):/source \
		-w /source \
		arnau/rust
