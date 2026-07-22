use yew::prelude::*;

// #[derive(Debug)]
// enum Genre {
//
// }
macro_rules! define_genre {
    ($($variant:ident),* $(,)?) => {  // Added $(,)? to handle trailing commas
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Genre {
            $($variant),*
        }

        impl Genre {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Genre::$variant => stringify!($variant)),*
                }
            }
        }
    };
}


       
   
macro_rules! img_path {
    ($name:expr) => {
        // format!("static/movies/{}.avif", $name)
        // concat gives static str
        concat!("static/movies/", $name, ".avif")
    };
}

// Now this works perfectly, even with the comma after Drama
define_genre! {
    Tragedy,
    Crime,
    Romance,
    TragicRomance,
    FeelGoodRomance,
    Drama,
    Psychological,
    PsychologicalDrama,
    PsychologicalTriller,
    Action,
    CarAction,
    LeagalDrama,
    Comedy,
    DarkComedy,
    Thriller,
    Musical,
    Mystery,
    FeelGood,
    Adventure,
    Fantasy,
    Epic,
    SiFi,
}

#[function_component(Movies)]
pub fn movies_page() -> Html {
    let movies: Vec<Html> = vec![
        Movie {
            img: "https://hips.hearstapps.com/digitalspyuk.cdnds.net/16/28/1468273033-nup-173272-1422.JPG",
            title: "Mr. Robot",
            date: "2015 - 2019",
            genre: &[Genre::Crime, Genre::Drama],
            stars: "Rami Malek, Christian Slater, Carly Chaikin",
        }.into(),
        Movie {
            img: img_path!("remember_me"),
            title: "Remember Me",
            date: "2010",
            genre: &[Genre::Romance, Genre::Drama, Genre::Tragedy],
            stars: "Robert Pattinson, Emilie de Ravin, Caitlyn Rund",
        }.into(),
        Movie {
            img: img_path!("bb"),
            title: "Breaking Bad",
            date: "2008 - 2013",
            genre: &[Genre::Crime, Genre::Drama, Genre::Tragedy, Genre::Psychological],
            stars: "Bryan Cranston, Aaron Paul, Anna Gunn",
        }.into(),
        Movie {
            img: img_path!("drive"),
            title: "Drive",
            date: "2011",
            genre: &[Genre::Drama, Genre::Action, Genre::CarAction],
            stars: "Ryan Gosling, Carey Mulligan, Bryan Cranston",
        }.into(),
        Movie {
            img: img_path!("bcs"),
            title: "Better Call Saul",
            date: "2015 - 2022",
            genre: &[Genre::DarkComedy, Genre::LeagalDrama, Genre::Crime, Genre::Tragedy],
            stars: "Bob Odenkirk, Rhea Seehorn, Jonathan Banks",
        }.into(),
        Movie {
            img: img_path!("before_sunrise"),
            title: "Before Sunrise",
            date: "1995",
            genre: &[Genre::TragicRomance, Genre::Drama, Genre::Romance],
            stars: "Ethan Hawke, Julie Delpy",
        }.into(),
        Movie {
            img: img_path!("before_sunset"),
            title: "Before Sunset",
            date: "2004",
            genre: &[Genre::FeelGoodRomance, Genre::Drama, Genre::Romance],
            stars: "Ethan Hawke, Julie Delpy",
        }.into(),
        Movie {
            img: img_path!("before_midnight"),
            title: "Before Midnight",
            date: "2013",
            genre: &[Genre::FeelGoodRomance, Genre::Drama, Genre::Romance],
            stars: "Ethan Hawke, Julie Delpy",
        }.into(),
        Movie {
            img: img_path!("nightcrawler"),
            title: "Nightcrawler",
            date: "2014",
            genre: &[Genre::PsychologicalDrama, Genre::PsychologicalTriller, Genre::Crime],
            stars: "Jake Gyllenhaal, Rene Russo, Bill Paxton",
        }.into(),
        Movie {
            img: img_path!("el_camino"),
            title: "El Camino",
            date: "2019",
            genre: &[Genre::PsychologicalDrama, Genre::PsychologicalTriller, Genre::Crime],
            stars: "Aaron Paul, Jonathan Banks, Matt Jones",
        }.into(),
        Movie {
            img: img_path!("la_la_land"),
            title: "La La Land",
            date: "2016",
            genre: &[Genre::Drama, Genre::Romance, Genre::Musical, Genre::Comedy],
            stars: "Ryan Gosling, Emma Stone, Rosemarie DeWitt",
        }.into(),
        Movie {
            img: img_path!("fight_club"),
            title: "Fight Club",
            date: "1999",
            genre: &[Genre::DarkComedy, Genre::PsychologicalTriller, Genre::Crime, Genre::Thriller],
            stars: "Brad Pitt, Edward Norton",
        }.into(),
        Movie {
            img: img_path!("stay"),
            title: "Stay",
            date: "2005",
            genre: &[Genre::PsychologicalTriller, Genre::PsychologicalTriller, Genre::Mystery, Genre::Thriller],
            stars: "Ewan McGregor, Naomi Watts, Ryan Gosling",
        }.into(),
        Movie {
            img: img_path!("daredevil"),
            title: "Daredevil",
            date: "2015 - 2018",
            genre: &[Genre::LeagalDrama, Genre::PsychologicalTriller, Genre::Action, Genre::Crime],
            stars: "Charlie Cox, Vincent D'Onofrio, Deborah Ann Woll",
        }.into(),

        Movie {
            img: img_path!("the_shawshank_redemption"),
            title: "The Shawshank Redemption",
            date: "1994",
            genre: &[Genre::FeelGood, Genre::Drama, Genre::Crime],
            stars: "Tim Robbins, Morgan Freeman, Bob Gunton",
        }.into(),
        Movie {
            img: img_path!("five_feet_apart"),
            title: "Five Feet Apart",
            date: "2019",
            genre: &[Genre::TragicRomance, Genre::Drama, Genre::Romance],
            stars: "Haley Lu Richardson, Cole Sprouse, Moises Arias",
        }.into(),
        Movie {
            img: img_path!("walter_mitty"),
            title: "The Secret Life of Walter Mitty",
            date: "2013",
            genre: &[Genre::Adventure, Genre::Drama, Genre::Romance, Genre::Fantasy],
            stars: "Ben Stiller, Kristen Wiig, Jon Daly",
        }.into(),
        Movie {
            img: img_path!("top_gun"),
            title: "Top Gun: Maverick",
            date: "2022",
            genre: &[Genre::Action, Genre::Drama, Genre::Epic],
            stars: "Tom Cruise, Jennifer Connelly, Miles Teller",
        }.into(),
        Movie {
            img: img_path!("dune"),
            title: "Dune: Part One",
            date: "2021".into(),
            genre: &[Genre::Adventure, Genre::SiFi, Genre::Action, Genre::Epic],
            stars: "Timothée Chalamet, Rebecca Ferguson, Zendaya",
        }.into(),
        Movie {
            img: img_path!("dunkirk"),
            title: "Dunkirk",
            date: "2017",
            genre: &[Genre::Action, Genre::Drama, Genre::Epic, Genre::Thriller],
            stars: "Fionn Whitehead, Barry Keoghan, Mark Rylance",
        }.into(),
        ];

    // Responsive masonry columns
    html! {
        <>
          <crate::components::header::Header />
            <div class="p-4 mx-auto max-w-3xl flex flex-col justify-center">
              <div style="columns: 300px;" class="!text-mocha-text dark:!text-latte-text">
                { for movies.into_iter().map(|movie| html! {
                  movie
                }) }
              </div>
            <crate::components::footer::Footer />
          </div>
        </>
    }
}

#[derive(Debug)]
struct Movie {
    img: &'static str,
    title: &'static str,
    date: &'static str,
    genre: &'static [Genre],
    stars: &'static str,
}

impl From<Movie> for Html {
    fn from(val: Movie) -> Self {
        let genre_list = val
            .genre
            .iter()
            .map(|g| g.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        html! {
          <div class="polaroid_wrapper">
            <img
                src={val.img}
                alt="No Description"
                loading="lazy"
          />
            <div class="polaroid_wrapper_ctx">
              <h1>{ val.title }</h1>
              <h4>{ val.date }</h4>
            </div>
              <div class="p-1"></div>
              <p><b>{ "Genre:  " }</b>{ genre_list }</p>
              <div class="p-1"></div>
            <p><b>{ "Stars:  " }</b>{val.stars }</p>
          </div>
        }
    }
}


#[derive(Debug)]
struct MovieEntry {
    img: &'static str,
    title: &'static str,
    date: &'static str,
    genre: &'static [Genre],
    stars: &'static str,
}

impl From<MovieEntry> for Html {
    fn from(val: Movie) -> Self {
        let genre_list = val
            .genre
            .iter()
            .map(|g| g.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        html! {
          <div class="polaroid_wrapper">
            <img
                src={val.img}
                alt="No Description"
                loading="lazy"
          />
            <div class="polaroid_wrapper_ctx">
              <h1>{ val.title }</h1>
              <h4>{ val.date }</h4>
            </div>
              <div class="p-1"></div>
              <p><b>{ "Genre:  " }</b>{ genre_list }</p>
              <div class="p-1"></div>
            <p><b>{ "Stars:  " }</b>{val.stars }</p>
          </div>
        }
    }
}



const Dune: MovieEntry = MovieEntry {
    img: img_path!("dune"),
    title: "Dune: Part One",
    date: "2021",
    genre: &[Genre::SiFi, Genre::Epic],
    stars: "Timothée Chalamet, Rebecca Ferguson, Zendaya",
};

