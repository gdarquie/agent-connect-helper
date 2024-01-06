export-grist x:
	clear && cargo build && cargo run --bin export-grist

extract-mongo-fqdn e:
	clear && cargo build && cargo run --bin extract-mongo-fqdn
