test:
	cargo test

cover:
	cargo tarpaulin --skip-clean --out Html --color=never
	brave tarpaulin-report.html

make-env: env.yaml
	mamba env create -f $^
