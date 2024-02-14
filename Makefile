

.PHONY: run
# run运行服务
run:
	cargo run

.PHONY: build
# build编译
build:
	cargo build

.PHONY: up
# 运行docker服务
up:
	docker-compose -f docker-compose.yml up -d

.PHONY: down
# 停止docker服务
down:
	docker-compose -f docker-compose.yml down



# show help
help:
	@echo ''
	@echo 'Usage:'
	@echo ' make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf "\033[36m%-22s\033[0m %s\n", helpCommand,helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help
