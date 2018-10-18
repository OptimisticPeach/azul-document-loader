extern crate azul;

use azul::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::ops::DerefMut;

pub mod dom_create;
pub mod load;
pub mod parse;
pub mod tokenize;

///
/// load_text
/// create_textids
/// consume_ast
/// 

pub fn load_resources(filename: &str) -> (Vec<parse::TextArgument>, parse::ASTPoint) {
    let source = load::load_into_string(filename).unwrap();
    let tokens = tokenize::tokenize(&*source);
    parse::parse(&mut VecDeque::from(tokens))
}

fn create_fonts<T>(app: &mut App<T>, fonts: &Vec<(Box<[u8]>, &str)>) -> HashMap<String, FontId>
where
    T: Layout,
{
    let mut new_fonts = HashMap::<String, FontId>::new();
    for (bytes, name) in fonts {
        let fontid = FontId::ExternalFont(name.to_string());
        app.add_font(fontid.clone(), &mut &**bytes).unwrap();
        new_fonts.insert(name.to_string(), fontid);
    }
    new_fonts
}

pub fn create_resources<T>(
    app: &mut App<T>,
    mut strings: VecDeque<parse::TextArgument>,
    preloaded_fonts: &Vec<(Box<[u8]>, &str)>,
    images: Vec<(String, &mut Box<[u8]>, ImageType)>
) -> VecDeque<TextId>
where
    T: Layout,
{
    let fonts = create_fonts(app, &preloaded_fonts);
    let mut ids = VecDeque::<TextId>::new();
    let default = FontId::BuiltinFont("sans-serif".into());
    while let Some(t) = strings.pop_front() {
        let fontid;
        if let Some(ref x) = t.font{
            if !fonts.contains_key(x) {
                panic!("You forgot to load in font {:?}", x);
            }
            fontid = fonts.get(&t.font.unwrap()).unwrap();
        }
        else {
            fontid = &default;
        }
        let fontsize = t.size.unwrap_or(10) as f32;
        ids.push_back(app.add_text_cached(t.body, &fontid, PixelValue::px(fontsize), None));
    }
    for (name, data, imgtype) in images{
        app.add_image(name, &mut&**data, imgtype).unwrap();
    }
    (
        ids
    )
}

pub fn consume_ast<T>(
    syntax_tree: &parse::ASTPoint,
    texts: &VecDeque<TextId>,
    info_source: &WindowInfo<T>,
) -> Dom<T>
where
    T: Layout,
{
    dom_create::create_dom(syntax_tree, &mut texts.clone(), info_source)
}
