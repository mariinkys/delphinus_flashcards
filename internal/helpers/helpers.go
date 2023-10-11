package helpers

import (
	"fmt"
	"net/http"
	"runtime/debug"

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
