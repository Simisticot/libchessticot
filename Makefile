profile:
	cargo flamegraph --bench benchmarks \
    -o perft-flamegraph.svg \
    -- --bench --profile-time 3 "perft"
