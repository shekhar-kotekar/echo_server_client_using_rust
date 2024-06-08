.PHONY: help run_server run_client run

run_server:
	make -C echo_server run

run_client:
	make -C echo_client run

run: run_server run_client
	@echo "Done"