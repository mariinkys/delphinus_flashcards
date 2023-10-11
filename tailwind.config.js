/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.{layout.tmpl,page.tmpl,js}"],
  daisyui: {
    themes: ["light", "dark"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}


