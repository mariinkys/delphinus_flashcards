package models

import "github.com/mariinkys/delphinus_flashcards/internal/forms"

type TemplateData struct {
	StringMap map[string]string
	IntMap    map[string]int
	FloatMap  map[string]float32
	Data      map[string]interface{}
	Flash     string
	Success   string
	Warning   string
	Error     string
	Form      *forms.Form
}
