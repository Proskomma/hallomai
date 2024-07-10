package tree_sitter_usfm3_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-usfm3"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_usfm3.Language())
	if language == nil {
		t.Errorf("Error loading Usfm3 grammar")
	}
}
