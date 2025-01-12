mod custom_argh {
    use crate::color::parse_colors;
    use crate::error_matrix::{ErrorMatrix, ErrorMatrixType};
    use crate::palette::Palette;
    use argh::FromArgs;

    #[derive(FromArgs)]
    /// Traitement d'image en Rust
    pub struct Args {
        /// build readme
        /// faire toutes les images du rendu
        #[argh(switch, short = 'b')]
        pub build: bool,

        /// chemin du fichier image (Exemple : "images/iut.jpg")
        #[argh(option, short = 'i')]
        pub input: Option<String>,

        /// chemin du fichier de sortie
        #[argh(option, short = 'o')]
        pub output: Option<String>,

        /// appliquer un filtre (optionnel)
        /// filtres disponibles :
        /// 'half': moitié des pixels en blanc,
        /// 'bw': noir et blanc,
        /// 'pal': change la palette de couleur pour une autre (utiliser l'option colors),
        /// 'dither': dithering aléatoire,
        /// 'bayer': dithering ordonné (utiliser l'option order),
        /// 'error': diffusion d'erreur (utiliser l'option error)
        #[argh(option)]
        pub filter: Option<String>,

        /// liste de couleurs pour la palette : black, white, red, green, blue, yellow, magenta, cyan
        /// exemple : "black,white"
        #[argh(option)]
        pub colors: Option<String>,

        /// ordre du dithering ordonné
        /// exemple : 4
        #[argh(option)]
        pub order: Option<u8>,

        /// algorithme de diffusion d'erreur
        /// algorithme disponible : 'basic', 'floyd-steinberg', 'jarvis-judice-ninke', 'atkinson'
        #[argh(option)]
        pub error: Option<String>,
    }

    impl Args {
        pub fn get_input(&self) -> &str {
            if self.input.is_none() {
                panic!("Le chemin du fichier image est manquant.");
            }
            self.input.as_deref().unwrap()
        }

        pub fn get_output(&self) -> &str {
            if self.output.is_none() {
                panic!("Le chemin du fichier de sortie est manquant.");
            }
            self.output.as_deref().unwrap()
        }

        pub fn get_filter(&self) -> bool {
            self.filter.is_some()
        }

        pub fn get_filter_type(&self) -> crate::custom_image::Filter {
            use crate::custom_image::Filter;
            match self.filter.as_deref() {
                Some("half") => Filter::Half,
                Some("bw") => Filter::BlackAndWhite,
                Some("pal") => Filter::ColorPalette(self.get_colors()),
                Some("dither") => Filter::RandomDithering,
                Some("bayer") => Filter::OrderedDithering(self.get_order(true).unwrap()),
                Some("error") => Filter::ErrorDiffusion(self.get_colors(), self.get_error()),
                _ => {
                    panic!("Le filtre \"{}\" est incorrect.", self.filter.as_deref().unwrap());
                }
            }
        }

        pub fn get_error(&self) -> ErrorMatrix {
            if self.error.is_none() {
                panic!("Le type d'erreur est manquant. ('basic', 'floyd-steinberg', 'jarvis-judice-ninke', 'atkinson')");
            }
            match self.error.as_deref() {
                Some("basic") => ErrorMatrix::get_type(ErrorMatrixType::Basic),
                Some("floyd-steinberg") => ErrorMatrix::get_type(ErrorMatrixType::FloydSteinberg),
                Some("jarvis-judice-ninke") => ErrorMatrix::get_type(ErrorMatrixType::JarvisJudiceNinke),
                Some("atkinson") => ErrorMatrix::get_type(ErrorMatrixType::Atkinson),
                _ => {
                    panic!("L'algorithme de diffusion d'erreur \"{}\" est incorrect.", self.error.as_deref().unwrap());
                }
            }
        }

        pub fn get_order(&self, mandatory: bool) -> Option<u8> {
            if mandatory && self.order.is_none() {
                panic!("L'ordre du dithering ordonné est manquant.");
            }
            self.order
        }

        pub fn get_colors(&self) -> Palette {
            if self.colors.is_none() {
                panic!("La liste de couleurs est manquante.");
            }
            match parse_colors(&self.colors.clone().unwrap()) {
                Ok(colors) => Palette::new(colors),
                Err(err) => {
                    panic!("La couleur \"{}\" est incorrecte.", err);
                }
            }
        }
    }
}
