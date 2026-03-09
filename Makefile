dev:
	watchexec --watch app/src -e scss "sed -i '' -e 's/^//' app/style/main.scss" &
	export $(shell cat .env | xargs) && cargo leptos watch