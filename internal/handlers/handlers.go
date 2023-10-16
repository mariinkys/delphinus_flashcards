package handlers

import (
	"errors"
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
		m.App.Session.Put(r.Context(), "error", "Please fill all the inputs!")
		render.Template(w, r, "generator.page.tmpl", &models.TemplateData{
			Form: form,
		})
		return
	}

	//TODO: TMP
	if language == "jp" {
		res, nFound := helpers.RunJap(characterString)
		m.App.Session.Put(r.Context(), "res", res)
		m.App.Session.Put(r.Context(), "not_found", nFound)
		http.Redirect(w, r, "/result", http.StatusSeeOther)
	} else {
		res, nFound := helpers.RunCh(characterString)
		m.App.Session.Put(r.Context(), "res", res)
		m.App.Session.Put(r.Context(), "not_found", nFound)
		http.Redirect(w, r, "/result", http.StatusSeeOther)
	}

}

// Result is the handler for the result page
func (m *Repository) Result(w http.ResponseWriter, r *http.Request) {
	//Make the data to pass it to the template
	data := make(map[string]interface{})

	res, ok := m.App.Session.Get(r.Context(), "res").(string)
	if !ok {
		m.App.Session.Put(r.Context(), "error", "Server Error!")
		http.Redirect(w, r, "/generator", http.StatusSeeOther)
		helpers.ServerError(w, errors.New("can't get result from session"))
		return
	}

	notFoundRes, ok := m.App.Session.Get(r.Context(), "not_found").([]string)
	if !ok {
		m.App.Session.Put(r.Context(), "error", "Server Error!")
		http.Redirect(w, r, "/generator", http.StatusSeeOther)
		helpers.ServerError(w, errors.New("can't get result from session"))
		return
	}

	data["res"] = res
	data["not_found"] = notFoundRes

	render.Template(w, r, "result.page.tmpl", &models.TemplateData{
		Data: data,
	})
}

// Faq is the handler for the Faq page
func (m *Repository) Faq(w http.ResponseWriter, r *http.Request) {
	render.Template(w, r, "faq.page.tmpl", &models.TemplateData{})
}
