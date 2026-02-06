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
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Genre::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

macro_rules! img_path {
    ($name:literal) => {
        format!("static/movies/{}.avif", $name)
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
            img: "https://hips.hearstapps.com/digitalspyuk.cdnds.net/16/28/1468273033-nup-173272-1422.JPG".into(),
            title: "Mr. Robot".into(),
            date: "2015 - 2019".into(),
            genre: vec![Genre::Crime, Genre::Drama],
            stars: "Rami Malek, Christian Slater, Carly Chaikin".into(),
        }.into(),
        Movie {
            img: img_path!("remember_me"),
            title: "Remember Me".into(),
            date: "2010".into(),
            genre: vec![Genre::Romance, Genre::Drama, Genre::Tragedy],
            stars: "Robert Pattinson, Emilie de Ravin, Caitlyn Rund".into(),
        }.into(),
        Movie {
            img: img_path!("bb"),
            title: "Breaking Bad".into(),
            date: "2008 - 2013".into(),
            genre: vec![Genre::Crime, Genre::Drama, Genre::Tragedy, Genre::Psychological],
            stars: "Bryan Cranston, Aaron Paul, Anna Gunn".into(),
        }.into(),
        Movie {
            img: img_path!("drive"),
            title: "Drive".into(),
            date: "2011".into(),
            genre: vec![Genre::Drama, Genre::Action, Genre::CarAction],
            stars: "Ryan Gosling, Carey Mulligan, Bryan Cranston".into(),
        }.into(),
        Movie {
            img: img_path!("bcs"),
            title: "Better Call Saul".into(),
            date: "2015 - 2022".into(),
            genre: vec![Genre::DarkComedy, Genre::LeagalDrama, Genre::Crime, Genre::Tragedy],
            stars: "Bob Odenkirk, Rhea Seehorn, Jonathan Banks".into(),
        }.into(),
        Movie {
            img: img_path!("before_sunrise"),
            title: "Before Sunrise".into(),
            date: "1995".into(),
            genre: vec![Genre::TragicRomance, Genre::Drama, Genre::Romance],
            stars: "Ethan Hawke, Julie Delpy".into(),
        }.into(),
        Movie {
            img: img_path!("before_sunset"),
            title: "Before Sunset".into(),
            date: "2004".into(),
            genre: vec![Genre::FeelGoodRomance, Genre::Drama, Genre::Romance],
            stars: "Ethan Hawke, Julie Delpy".into(),
        }.into(),
        Movie {
            img: img_path!("before_midnight"),
            title: "Before Midnight".into(),
            date: "2013".into(),
            genre: vec![Genre::FeelGoodRomance, Genre::Drama, Genre::Romance],
            stars: "Ethan Hawke, Julie Delpy".into(),
        }.into(),
        Movie {
            img: img_path!("nightcrawler"),
            title: "Nightcrawler".into(),
            date: "2014".into(),
            genre: vec![Genre::PsychologicalDrama, Genre::PsychologicalTriller, Genre::Crime],
            stars: "Jake Gyllenhaal, Rene Russo, Bill Paxton".into(),
        }.into(),
        Movie {
            img: img_path!("el_camino"),
            title: "El Camino".into(),
            date: "2019".into(),
            genre: vec![Genre::PsychologicalDrama, Genre::PsychologicalTriller, Genre::Crime],
            stars: "Aaron Paul, Jonathan Banks, Matt Jones".into(),
        }.into(),
        Movie {
            img: img_path!("la_la_land"),
            title: "La La Land".into(),
            date: "2016".into(),
            genre: vec![Genre::Drama, Genre::Romance, Genre::Musical, Genre::Comedy],
            stars: "Ryan Gosling, Emma Stone, Rosemarie DeWitt".into(),
        }.into(),
        Movie {
            img: img_path!("fight_club"),
            title: "Fight Club".into(),
            date: "1999".into(),
            genre: vec![Genre::DarkComedy, Genre::PsychologicalTriller, Genre::Crime, Genre::Thriller],
            stars: "Brad Pitt, Edward Norton".into(),
        }.into(),
        Movie {
            img: img_path!("stay"),
            title: "Stay".into(),
            date: "2005".into(),
            genre: vec![Genre::PsychologicalTriller, Genre::PsychologicalTriller, Genre::Mystery, Genre::Thriller],
            stars: "Ewan McGregor, Naomi Watts, Ryan Gosling".into(),
        }.into(),
        Movie {
            img: img_path!("daredevil"),
            title: "Daredevil".into(),
            date: "2015 - 2018".into(),
            genre: vec![Genre::LeagalDrama, Genre::PsychologicalTriller, Genre::Action, Genre::Crime],
            stars: "Charlie Cox, Vincent D'Onofrio, Deborah Ann Woll".into(),
        }.into(),

        Movie {
            img: img_path!("the_shawshank_redemption"),
            title: "The Shawshank Redemption".into(),
            date: "1994".into(),
            genre: vec![Genre::FeelGood, Genre::Drama, Genre::Crime],
            stars: "Tim Robbins, Morgan Freeman, Bob Gunton".into(),
        }.into(),
        Movie {
            img: img_path!("five_feet_apart"),
            title: "Five Feet Apart".into(),
            date: "2019".into(),
            genre: vec![Genre::TragicRomance, Genre::Drama, Genre::Romance],
            stars: "Haley Lu Richardson, Cole Sprouse, Moises Arias".into(),
        }.into(),
        Movie {
            img: img_path!("walter_mitty"),
            title: "The Secret Life of Walter Mitty".into(),
            date: "2013".into(),
            genre: vec![Genre::Adventure, Genre::Drama, Genre::Romance, Genre::Fantasy],
            stars: "Ben Stiller, Kristen Wiig, Jon Daly".into(),
        }.into(),
        Movie {
            img: img_path!("top_gun"),
            title: "Top Gun: Maverick".into(),
            date: "2022".into(),
            genre: vec![Genre::Action, Genre::Drama, Genre::Epic],
            stars: "Tom Cruise, Jennifer Connelly, Miles Teller".into(),
        }.into(),
        Movie {
            img: img_path!("dune"),
            title: "Dune: Part One".into(),
            date: "2021".into(),
            genre: vec![Genre::Adventure, Genre::SiFi, Genre::Action, Genre::Epic],
            stars: "Timothée Chalamet, Rebecca Ferguson, Zendaya".into(),
        }.into(),
        Movie {
            img: img_path!("dunkirk"),
            title: "Dunkirk".into(),
            date: "2017".into(),
            genre: vec![Genre::Action, Genre::Drama, Genre::Epic, Genre::Thriller],
            stars: "Fionn Whitehead, Barry Keoghan, Mark Rylance".into(),
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
                </div>
            <crate::components::footer::Footer />
        </>
    }
}

#[derive(Debug)]
struct Movie {
    img: String,
    title: String,
    date: String,
    genre: Vec<Genre>,
    stars: String,
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
