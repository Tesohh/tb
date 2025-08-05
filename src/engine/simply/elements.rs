pub struct Heading {
    pub level: u8,
    pub content: Paragraph,
}

pub struct Paragraph {
    pub content: Vec<Span>,
}

pub struct Span {
    pub color: ratatui::style::Color,
    pub bold: bool,
    pub italic: bool,
    pub content: String,
}

pub struct Link {
    pub href: String,
    pub content: Vec<Span>,
}

pub struct Image {}

pub struct List {
    pub kind: ListKind,
    pub items: Vec<ListItem>,
}

pub enum ListKind {
    Unordered,
}

pub struct ListItem {
    content: Paragraph,
}

pub enum TbElementKind {
    Heading(Heading),
    Paragraph(Paragraph),
    Span(Span),
    Link(Link),
    Image(Image),
    List(List),
}
