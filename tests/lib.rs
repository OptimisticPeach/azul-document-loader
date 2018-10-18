extern crate azul;

use azul::prelude::*;
use azul_document_loader::*;
use std::collections::VecDeque;

#[derive(Debug)]
struct MyDataModel{
    ast: Option<parse::ASTPoint>,
    texts: Option<VecDeque<TextId>>
}

impl Layout for MyDataModel {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        if let Some(ref texts) = self.texts{
            if let Some(ref ast) = self.ast{
                return consume_ast(&ast, &texts, &info)
            }
        }
        panic!("Can't find ast and text inside of MyDataModel");
    }
}

#[test]
fn test_it() {
    macro_rules! CSS_PATH {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/main.css")
        };
    }
    let ubuntu_font = include_bytes!("Ubuntu-Regular.ttf");

    let css = Css::new_from_str(include_str!(CSS_PATH!())).unwrap();

    let mut app = App::new(MyDataModel{ast: None, texts: None}, AppConfig::default());
    let (texts, ast) = load_resources(".\\tests\\test.azd");
    let textids = create_resources(&mut app, VecDeque::from(texts), &vec![(Box::new(*ubuntu_font), "Ubuntu-Regular")], Vec::new());
    (*app.app_state.data.lock().unwrap()).ast = Some(ast);
    (*app.app_state.data.lock().unwrap()).texts = Some(textids);
    app.run(Window::new(WindowCreateOptions::default(), css).unwrap())
        .unwrap();
}
