testflags := --

ifdef TFLAGS
	testflags += $(TFLAGS)
endif

test:
	cargo test --workspace $(testflags)

clippy:
	cargo clippy --workspace

cover:
	cargo tarpaulin --skip-clean --out Html --color=never --workspace
	brave tarpaulin-report.html

make-env: env.yaml
	mamba env create -f $^
