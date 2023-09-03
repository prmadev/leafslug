watch-run: 
	cargo watch -q -c -w src/ -x  run | bunyan -l info 

watch-test:
	cargo watch -q -c -w tests/ -x "nextest run quick_dev --no-capture -F local" 
