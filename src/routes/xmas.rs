use std::fmt;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
pub enum Giftee {
    Aria,
    Finn,
    Gabe,
    Hannah,
    Joe,
    Lucinda,
    Nilam,
    Rachel,
    Ste,
}

impl fmt::Display for Giftee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Aria    => "Aria",
            Self::Finn    => "Finn",
            Self::Gabe    => "Gabe",
            Self::Hannah  => "Hannah",
            Self::Joe     => "Joe",
            Self::Lucinda => "Lucinda",
            Self::Nilam   => "Nilam",
            Self::Rachel  => "Rachel",
            Self::Ste     => "Ste",
        })
    }
}

#[derive(Serialize)]
struct XmasContext {
    parent: &'static str,
    giftee: String,
}

#[get("/xmas/<id>")]
pub fn get_xmas(id: String) -> Template {
    let giftee = match id.as_ref() {
        "OsasHC" => Some(Giftee::Aria),
        "frkTL6" => Some(Giftee::Finn),
        "wNLcyz" => Some(Giftee::Gabe),
        "1SK9jB" => Some(Giftee::Hannah),
        "QOylgX" => Some(Giftee::Joe),
        "szYHar" => Some(Giftee::Lucinda),
        "E2bocH" => Some(Giftee::Nilam),
        "0dSNgX" => Some(Giftee::Rachel),
        "YdzsZs" => Some(Giftee::Ste),
        _ => None,
    };

    let mut context = XmasContext {
        parent: "layout",
        giftee: String::new(),
    };

    if let Some(g) = giftee {
        context.giftee = format!("{}", g);
    }

    Template::render("xmas/gifts", &context)
}
