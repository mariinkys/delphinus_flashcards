package main

import (
	"log"
	"net/http"
	"os"
	"strconv"
	"time"

	"github.com/alexedwards/scs/v2"
	_ "github.com/joho/godotenv/autoload"
	"github.com/mariinkys/delphinus-backend/internal/config"
	"github.com/mariinkys/delphinus-backend/internal/handlers"
	"github.com/mariinkys/delphinus-backend/internal/helpers"
)

var app config.AppConfig
var session *scs.SessionManager
var infoLog *log.Logger
var errorLog *log.Logger

func main() {
	portNumber, ok := os.LookupEnv("ApplicationPort")
	if !ok {
		log.Fatal("Cannot load production env variable")
	}

	// Change this to true when in production
	production := os.Getenv("InProduction")
	boolProdValue, err := strconv.ParseBool(production)
	if err != nil {
		log.Fatal("Cannot load production env variable")
	}
	app.InProduction = boolProdValue

	// Create Loggers
	infoLog = log.New(os.Stdout, "INFO - ", log.Ldate|log.Ltime)
	app.InfoLog = infoLog

	errorLog = log.New(os.Stdout, "ERROR - ", log.Ldate|log.Ltime|log.Lshortfile)
	app.ErrorLog = errorLog

	// Set up the session
	session = scs.New()
	session.Lifetime = 24 * time.Hour
	session.Cookie.Persist = true
	session.Cookie.SameSite = http.SameSiteLaxMode
	session.Cookie.Secure = app.InProduction

	app.Session = session

	// Register models on the session
	//gob.Register(models.User{})

	repo := handlers.NewRepo(&app)
	handlers.NewHandlers(repo)
	helpers.NewHelpers(&app)

	log.Printf("Starting application on http://localhost:%s", portNumber)

	server := &http.Server{
		Addr:    ":" + portNumber,
		Handler: routes(&app),
	}

	if err = server.ListenAndServe(); err != nil {
		log.Fatal(err)
	}
}
