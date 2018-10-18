extern crate azul;

use azul::prelude::*;
use crate::parse::*;
use std::collections::VecDeque;

macro_rules! relate {
    ($head:expr, $($child:expr),+) => {
        {
        $head
        $(
            .with_child($child)
        )+
        }
    };
    ($node:expr, $id:ident) => {
        expr.with_id(stringify!($id))
    }}

macro_rules! div {
    () => {
        Dom::new(NodeType::Div)
    };
    ($id:expr) => {
        Dom::new(NodeType::Div).with_id($id)
    };
}
macro_rules! label {
    ($text:expr) => {
        Dom::new(NodeType::Label(format!("{}", $text)))
    };
    ($text:expr, $id:expr) => {
        Dom::new(NodeType::Label(format!("{}", $text))).with_id($id)
    };
}
macro_rules! image {
    ($find:expr, $imgid:expr) => {
        Dom::new(NodeType::Image($find.resources.get_image($imgid).unwrap()))
    };

    ($find:expr, $imgid:expr, $id:expr) => {
        Dom::new(NodeType::Image($find.resources.get_image($imgid).unwrap())).with_id($id)
    };
}
macro_rules! text {
    ($text:expr) => {{
        Dom::new(NodeType::Text($text))
    }};

    ($text:expr, $id:expr) => {{
        Dom::new(NodeType::Text($text)).with_id($id)
    }};
}

fn create_single<T>(
    point: &NType,
    id: &Option<String>,
    texts: &mut VecDeque<TextId>,
    info: &WindowInfo<T>
) -> Dom<T>
where
    T: Layout,
{
    match point {
        NType::Div => {
            if let Some(ref x) = id {
                div!(x.clone())
            } else {
                div!()
            }
        }
        NType::Label(ref text) => {
            if let Some(ref x) = id {
                label!(text.clone(), x.clone())
            } else {
                label!(text.clone())
            }
        }
        NType::Image(ref imgid) => {
            if let Some(ref x) = id {
                image!(info, imgid.clone(), x.clone())
            } else {
                Dom::new(NodeType::Image(
                    info.resources.get_image(imgid.clone()).unwrap(),
                ))
            }
        }
        NType::Text => {
            let t_id = texts.pop_front().unwrap();
            if let Some(ref x) = id {
                text!(t_id, x.clone())
            } else {
                text!(t_id)
            }
        }
    }
}

pub fn create_dom<T>(
    head: &ASTPoint,
    texts: &mut VecDeque<TextId>,
    info: &WindowInfo<T>
) -> Dom<T>
where
    T: Layout,
{
    match head {
        //node.1 is an Id
        ASTPoint::Element(ref node) => create_single(&node.0, &node.1, texts, info),
        ASTPoint::Joint(ref head, ref body) => {
            let mut main_node = create_single(&head.0, &head.1, texts, info);
            for i in body {
                main_node = relate![main_node, create_dom(i, texts, info)];
            }
            main_node
        }
    }
}
