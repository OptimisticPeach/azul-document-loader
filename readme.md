## azul_document_loader
A document loader for [azul](https://github.com/maps4print/azul) written in rust.

# Syntax
The syntax isn't that complicated:

    node_type:optional_id(optional_text)[
        {This is a comment}
        div:div_style;
        image:image_size_style("catimg");
        label:large_label_red("this is a label");
        {Paragraph styles have to contain the font and font size,
         the font in the text node is meant to be for caching purposes so that text
         isn't rendered on the fly.}
        text:paragraph_style("long text" "font to be cached" 12);
    ]