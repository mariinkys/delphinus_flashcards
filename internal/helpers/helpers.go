package helpers

import (
	"fmt"
	"net/http"
	"runtime/debug"
	"strings"

	"github.com/mariinkys/delphinus_flashcards/internal/config"
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
