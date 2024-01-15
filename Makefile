export-grist x:
	clear && cargo build && cargo run --bin export-grist

extract-mongo-fqdn e:
	clear && cargo build && cargo run --bin extract-mongo-fqdn

upload-and-execute-mongo-script ux:
	clear && bash shell/upload-and-execute-mongo-script.sh