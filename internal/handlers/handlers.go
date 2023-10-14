package handlers

import (
	"net/http"

	"github.com/mariinkys/delphinus_flashcards/internal/config"
	"github.com/mariinkys/delphinus_flashcards/internal/forms"
	"github.com/mariinkys/delphinus_flashcards/internal/helpers"
	"github.com/mariinkys/delphinus_flashcards/internal/models"
	"github.com/mariinkys/delphinus_flashcards/internal/render"
)

// Repo the repository used by the handlers
var Repo *Repository

// Repository is the repository type
type Repository struct {
	App *config.AppConfig
}

// NewRepo creates a new repository
func NewRepo(a *config.AppConfig) *Repository {
	return &Repository{
		App: a,
	}
}

// NewHandlers sets the repository for the handlers
func NewHandlers(r *Repository) {
	Repo = r
}

// Home is the handler for the home page
func (m *Repository) Home(w http.ResponseWriter, r *http.Request) {
	// remoteIP := r.RemoteAddr
	// m.App.Session.Put(r.Context(), "remote_ip", remoteIP)
	render.Template(w, r, "home.page.tmpl", &models.TemplateData{})
}

// Generator is the handler for the generator page
func (m *Repository) Generator(w http.ResponseWriter, r *http.Request) {
	render.Template(w, r, "generator.page.tmpl", &models.TemplateData{
		Form: forms.New(nil),
	})
}

// PostGenerator is the handler for the generator page
func (m *Repository) PostGenerator(w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	if err != nil {
		helpers.ServerError(w, err)
		return
	}

	characterString := r.Form.Get("characters")
	language := r.Form.Get("language")

	form := forms.New(r.PostForm)
	form.Required("characters", "language")

	if !form.Valid() {
		render.Template(w, r, "generator.page.tmpl", &models.TemplateData{
			Form: form,
		})
		return
	}

	//TODO: TMP
	if language == "jp" {
		res := helpers.RunJap(characterString)
		w.Write([]byte(res))
	} else {
		res := helpers.RunCh(characterString)
		w.Write([]byte(res))
	}

}
