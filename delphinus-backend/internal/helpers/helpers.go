package helpers

import (
	"fmt"
	"net/http"
	"runtime/debug"
	"strings"
	"unicode"

	"github.com/mariinkys/delphinus-backend/internal/config"
	"github.com/mariinkys/delphinus-backend/internal/models"
)

var app *config.AppConfig

// NewHelpers sets up app config for helpers
func NewHelpers(a *config.AppConfig) {
	app = a
}

// ClientError handles client-side errors
func ClientError(w http.ResponseWriter, status int) {
	app.InfoLog.Println("Client Error with status", status)
	http.Error(w, http.StatusText(status), status)
}

// ServerError handles server-side errors
func ServerError(w http.ResponseWriter, err error) {
	trace := fmt.Sprintf("%s\n%s", err.Error(), debug.Stack())
	app.ErrorLog.Println(trace)
	http.Error(w, http.StatusText(http.StatusInternalServerError), http.StatusInternalServerError)
}

// TrimLeftString Trims Left a string given a string and a character
func TrimLeftString(s string, char string) string {
	if idx := strings.Index(s, char); idx != -1 {
		return s[idx:]
	}
	return s
}

// TrimRightString Trims Right a string given a string and a character
func TrimRightString(s string, char string) string {
	if idx := strings.Index(s, char); idx != -1 {
		return s[:idx]
	}
	return s
}

// GenerateFlashcardsOutput Generates the output needed to import on Quizlet...
func GenerateFlashcardsOutput(flashcards []models.Flashcard) string {
	var result string

	for i, s := range flashcards {
		result += fmt.Sprint(s.Front + "/#*#/" + s.Back + "\n")
		if i != (len(flashcards) - 1) {
			result += fmt.Sprint("\\#" + "\n")
		}
	}

	return result
}

// RemoveWhiteSpaces Removes whitespace from all words on a string array
func RemoveWhiteSpaces(input []string) []string {
	var output []string

	for _, s := range input {
		output = append(output, strings.Map(func(r rune) rune {
			if unicode.IsSpace(r) {
				return -1
			}
			return r
		}, s))
	}
	return output
}
