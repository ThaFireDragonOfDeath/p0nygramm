SED := sed

.PHONY: all
all: const-config.toml system-config.toml

const-config.toml:
	$(CP) const-config.toml.tmpl const-config.toml

system-config.toml:
	$(CP) system-config.toml.tmpl system-config.toml

.PHONY: clean
clean:
	$(RM) const-config.toml
