dev-simulator-run:
	cd device-simulator && yarn start

bridge-build:
	cd rmq-bridge && cargo build

bridge-run:
	cd rmq-bridge && cargo run

consumer-build:
	cd rmq-consumer && cargo build

consumer-run:
	cd rmq-consumer && cargo run