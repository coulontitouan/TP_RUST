# TP_RUST

NOAM DOUCET - @Doucet-Noam1  
TITOUAN COULON - @coulontitouan
<hr>
Les images sont stockées dans le dossier `images/` et les résultats dans le dossier `images/output`.

## Partie 1:

### Question 1:
```
Créer un nouveau projet Cargo, avec une dépendance sur la bibliothèque image, version 0.24.
```

```bash
cargo new tp_rust
cd tp_rust
echo image = \"0.24\" >> Cargo.toml
```

### Question 2:
```
Pour ouvrir une image depuis un fichier, on utilise ... 
On obtient un DynamicImage, à quoi correspond ce type? 
Comment obtenir une image en mode rbg8 ...
Une image arbitraire peut avoir des pixels de nature différente:
• avec un nombre variables de canaux (couleurs ou non, transparence ou non)
• avec un type de donnée différent pour les canaux (entiers sur un octet, flottants ou autres)
Passer l’image d’entrée en mode rgb8, c’est-à-dire avec 3 canaux R, G, B, représentés chacun par un u8.
```

Pour ouvrir une image depuis un fichier, on utilise la fonction open() du module image::io::Reader.
```rust
use image::GenericImageView;

let img = image::open("example.jpg").expect("Failed to open image");
```
`A Dynamic Image : This represents a matrix of pixels which are convertible from and to an RGBA representation.`

DynamicImage représente une image chargée en mémoire.

Pour obtenir une image en mode rgb8, on utilise la méthode to_rgb8() de DynamicImage.
```rust
let rgb_image = img.to_rgb8();
```

### Question 3:
```
Sauver l’image obtenue au format png. Que se passe-t-il si l’image de départ avait un canal alpha?
Expliquer dans le README de votre rendu ce qui se passe ici.
```

Le canal alpha est ignoré lors de la sauvegarde de l'image en PNG. Il sera simplement transformé en couleur noire. (logo.png -> rgb_logo.png)

### Question 4:
```
Afficher dans le terminal la couleur du pixel (32, 52) de l’image de votre choix.
```
![example.jpg](images/example.jpg)

La couleur du pixel (32, 52) est <span style="color:#346513">couleur #346513</span>.
```rust
