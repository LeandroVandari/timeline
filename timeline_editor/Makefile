wasm:
	RUSTFLAGS="-Ctarget-cpu=generic" \
		cargo build \
  		--no-default-features --profile wasm-release --target wasm32-unknown-unknown \
		--features wasm
	mkdir -p wasm_output
	wasm-bindgen --out-name timeline --target web ../target/wasm32-unknown-unknown/wasm-release/timeline_editor.wasm --out-dir wasm_output
	wasm-opt -Oz --all-features --output wasm_output/optimized.wasm wasm_output/timeline_bg.wasm
	mv wasm_output/optimized.wasm wasm_output/timeline_bg.wasm
	if test -d assets; then cp -r assets/ wasm_output/assets/; fi
	echo '''<!DOCTYPE html> \
		<html lang="en"> \
		<head> \
    		<meta charset="UTF-8" /> \
    		<script type="module">import init from "./timeline.js"; init();</script> \
    		<style> \
        		body { \
            		margin: 0; \
            		overflow: hidden; \
        		} \
        		#timeline { \
            		width: 100dvw; \
            		height: 100dvh; \
        		} \
    		</style> \
		</head> \
		<body> \
    		<div id="timeline"><canvas id="timeline-canvas"></canvas></div> \
		</body> \
		</html>''' > wasm_output/index.html

wasm_debug:
	RUSTFLAGS="-Ctarget-cpu=generic" \
		cargo build \
  		--no-default-features --target wasm32-unknown-unknown \
		--features wasm,debug
	mkdir -p wasm_output
	wasm-bindgen --out-name timeline --target web ../target/wasm32-unknown-unknown/debug/timeline_editor.wasm --out-dir wasm_output
	if test -d assets; then cp -r assets/ wasm_output/assets/; fi
	echo '''<!DOCTYPE html> \
		<html lang="en"> \
		<head> \
    		<meta charset="UTF-8" /> \
    		<script type="module">import init from "./timeline.js"; init();</script> \
    		<style> \
        		body { \
            		margin: 0; \
            		overflow: hidden; \
        		} \
        		#timeline { \
            		width: 100dvw; \
            		height: 100dvh; \
        		} \
    		</style> \
		</head> \
		<body> \
    		<div id="timeline"><canvas id="timeline-canvas"></canvas></div> \
		</body> \
		</html>''' > wasm_output/index.html



profile:
	cargo build --bin big_timeline -r --no-default-features --features profiling
