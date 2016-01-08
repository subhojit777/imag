#
#
#
# Variables
#
#
#

export MAKE_FLAGS=--no-print-directory

export OUT=$(shell pwd)/bin
export OUT_PDF=$(OUT)/pdf/
export OUT_HTML=$(OUT)/html/

DOCUMENT_CLASS=article
SETTING_FONTSIZE=11pt

## Source directory
SRC_DIR=$(shell pwd)/src

## CSS directory
CSS_DIR=$(SRC_DIR)/css

## All markdown files in the working directory
export SRC=$(shell find $(SRC_DIR) -name "*.md" | sort)

## Templates
TEMPLATES=$(shell pwd)/templates

DOCUMENT_SETTINGS_PDF=                           \
	--listings                                   \
	--variable fontsize=$(SETTING_FONTSIZE)      \
	--variable papersize=a4paper                 \
	--variable classoption=cleardoublepage=empty \
	--variable classoption=index=totoc           \
	--variable classoption=openright             \
	--variable classoption=final                 \
	--variable classoption=listof=nochaptergap   \
	--variable documentclass=$(DOCUMENT_CLASS)   \
	--variable babel-lang=english                \
	--variable geometry=portrait

DOCUMENT_SETTINGS_HTML=			\
	--variable lang=de 			\
	--variable lof=true			\
	--variable lol=true			\
	--variable lot=true			\
	--variable toc=true			\
	--table-of-contents			\
	--webtex

#
#
#
# Binary and argument construction
#
#
#

ECHO_CMD=$(shell which echo)
ECHO_ARG=-e
ECHO=$(ECHO_CMD) $(ECHO_ARG)
export ECHO

MKDIR_CMD=$(shell which mkdir)
MKDIR_ARG=-p
MKDIR=$(MKDIR_CMD) $(MKDIR_ARG)
export MKDIR

RM_CMD=$(shell which rm)
RM_ARG=-fr
RM=$(RM_CMD) $(RM_ARG)
export RM

PANDOC=$(shell which pandoc)

PANDOC_PARAMS=-r markdown+simple_tables+table_captions+yaml_metadata_block+definition_lists+raw_html+markdown_in_html_blocks \
			  --filter pandoc-crossref

PANDOC_CC=$(PANDOC) $(PANDOC_PARAMS)

export PANDOC_CC_PDF=$(PANDOC)        \
			  $(PANDOC_PARAMS)        \
			  --latex-engine=pdflatex \
			  $(DOCUMENT_SETTINGS_PDF)

export PANDOC_CC_HTML=$(PANDOC) $(PANDOC_PARAMS) $(DOCUMENT_SETTINGS_HTML)

TARGET_PDF=$(OUT_PDF)/paper.pdf
TARGET_HTML=$(OUT_HTML)/index.html
#
#
# Tasks
#
#

# Main task
all: $(TARGET_PDF) $(TARGET_HTML)
	@$(ECHO) "\t[ALL   ]"

# create out directory
$(OUT):
	@$(ECHO) "\t[MKDIR ] $@"
	@$(MKDIR) $(OUT)

# create html out directory
$(OUT_HTML): $(OUT)
	@$(ECHO) "\t[MKDIR ] $@"
	@$(MKDIR) $(OUT_HTML)

# create html out directory
$(OUT_PDF): $(OUT)
	@$(ECHO) "\t[MKDIR ] $@"
	@$(MKDIR) $(OUT_PDF)

# cleanup task
clean:
	@$(ECHO) "\t[RM    ] $@"
	@$(RM) $(OUT)

pdf: $(TARGET_PDF)

$(TARGET_PDF): $(OUT_PDF)
	@$(ECHO) "\t[PANDOC] pdf"
	@$(PANDOC_CC_PDF)                         \
		--template $(TEMPLATES)/default.latex \
		$(SRC) -o $@

html: $(TARGET_HTML)

$(TARGET_HTML): $(OUT_HTML)
	@$(ECHO) "\t[PANDOC] html"
	@$(PANDOC_CC_HTML)                        \
		--template $(TEMPLATES)/default.html5 \
		$(SRC) -o $@

.PHONY: $(TARGET_PDF) $(TARGET_HTML)

