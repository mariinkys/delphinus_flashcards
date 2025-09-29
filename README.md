<h1 align="center">Delphinus</h1>

<p align="center">
 <a href="https://delphinus.mariinkys.dev" target="_blank"><img src="https://github.com/mariinkys/delphinus/blob/main/assets/delphinus_banner.png?raw=true"></a>
</p>

<p align="center">
 This project contains a small web application written in Rust with the <a href="https://leptos.dev/" target="_blank">Leptos</a> framework, it can generate flashcards compatible with Quizlet, Vaia and Anki for both Chinese and Japanese.
</p>


## How to Use?

[See the FAQ](https://delphinus.mariinkys.dev/faq)

## Development Notes

Remember the env variables needed for the server binary:
```
export LEPTOS_OUTPUT_NAME="leptos_start"
export LEPTOS_SITE_ROOT="site" #necessary
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```

## Thanks

You can also help do this and more projects, [Buy me a coffee](https://www.buymeacoffee.com/mariinkys)

## Acknowledgements

This site uses the <a class="link-primary" href="https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project">JMdict/EDICT</a>
 and <a class="link-primary" href="https://www.edrdg.org/wiki/index.php/KANJIDIC_Project">KANJIDIC </a> dictionary files.
These files are the property of the <a class="link-primary" href="https://www.edrdg.org/">Electronic Dictionary Research and Development Group</a>
, and are used in conformance with the Group's<a class="link-primary" href="https://www.edrdg.org/edrdg/licence.html"> licence</a>.

This site uses the <a class="link-primary" href="https://www.mdbg.net/chinese/dictionary?page=cedict">CC-CEDICT </a> dictionary files and are used in conformance with its license.
