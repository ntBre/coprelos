make-env: env.yaml
	mamba env create -f $^
