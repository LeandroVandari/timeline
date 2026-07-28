wasm:
	RUSTFLAGS="-C target-cpu=generic" cargo build --bin big_timeline --no-default-features --profile wasm-release --target wasm32-unknown-unknown
	mkdir -p wasm_output
	wasm-bindgen --out-name timeline --target web ../target/wasm32-unknown-unknown/wasm-release/big_timeline.wasm --out-dir wasm_output
	wasm-opt -Oz --all-features --output wasm_output/optimized.wasm wasm_output/timeline_bg.wasm
	mv wasm_output/optimized.wasm wasm_output/timeline_bg.wasm
	if test -d assets; then cp -r assets/ wasm_output/assets/; fi
	echo '''<!DOCTYPE html> \
	<html lang="en"> \
	<head> \
		<meta charset="UTF-8" /> \
		<script type="module"> \
			import init from "./timeline.js"; \
			init(); \
		</script> \
	</head> \
	<body> \
		<canvas id="timeline-canvas"></canvas> \
	</body> \
	</html>''' > wasm_output/index.html
profile:
	cargo build --bin big_timeline -r --no-default-features --features profiling
