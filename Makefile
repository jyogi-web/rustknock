.PHONY: init
init:
	docker compose up -d --build
	yarn

.PHONY: yew_start
yew_start:
	trunk serve

.PHONY: react_start
react_start:
	PORT=8080 yarn workspace frontend-react start

