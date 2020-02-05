SED := sed
CP := cp -f

const-config.toml:
	$(CP) const-config.toml.tmpl const-config.toml

.PHONY: clean
clean:
	$(RM) const-config.toml
