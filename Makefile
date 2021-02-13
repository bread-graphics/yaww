# MIT/Apache2 License

CARGO = cargo
RUSTFMT = rustfmt
VKEY = $(PWD)/vkey/target/debug/yaww-vkey
VKEY_SRC = $(PWD)/vkey/src
VKEY_DEPS := $(wildcard $(VKEY_SRC)/*)
VKEY_FILE = $(PWD)/src/vkey.rs
VKEY_DESC = $(PWD)/vkey/vkey.json

auto: $(VKEY_FILE)

$(VKEY_FILE): $(VKEY) $(VKEY_DESC)
	$(VKEY) $(VKEY_DESC) $(VKEY_FILE)
	$(RUSTFMT) $(VKEY_FILE)

$(VKEY): $(VKEY_DEPS)
	cd $(PWD)/vkey; $(CARGO) build
