# Programms
SED := sed

# TOML Template files
TOML_TEMPLATES := const-config.toml system-config.toml

.PHONY: all
all: $(TOML_TEMPLATES)

%.toml: %.toml.tmpl
	$(CP) $< $@

.PHONY: clean
clean:
	$(RM) *.toml
