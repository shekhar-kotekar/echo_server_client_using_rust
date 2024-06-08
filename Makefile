.PHONY: help build_server run_server run_client run

build_server:
	make -C echo_server build

run_server:
	make -C echo_server run

run_client:
	make -C echo_client run

run: run_server run_client
	@echo "Done"