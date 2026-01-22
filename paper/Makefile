# Pressure-Field Coordination Paper - Build System

MAIN = main
LATEX = pdflatex
BIBTEX = bibtex

.PHONY: all clean watch draft final

# Default: build draft (single pass for speed)
all: draft

# Draft build (single pass for speed)
draft:
	$(LATEX) $(MAIN).tex

# Final build (full bibliography)
final:
	$(LATEX) $(MAIN).tex
	$(BIBTEX) $(MAIN)
	$(LATEX) $(MAIN).tex
	$(LATEX) $(MAIN).tex

# Clean auxiliary files
clean:
	rm -f $(MAIN).aux $(MAIN).bbl $(MAIN).blg $(MAIN).log $(MAIN).out
	rm -f $(MAIN).toc $(MAIN).lof $(MAIN).lot $(MAIN).fls $(MAIN).fdb_latexmk
	rm -f $(MAIN).synctex.gz $(MAIN).bcf $(MAIN).run.xml

# Clean everything including PDF
distclean: clean
	rm -f $(MAIN).pdf

# Watch for changes (requires entr)
watch:
	@echo "Watching for changes... (Ctrl+C to stop)"
	@ls *.tex *.bib | entr -c make draft
