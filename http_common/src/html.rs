pub struct HTML {

}

impl HTML {
    pub fn html(body: &str) -> String {
        return HTML::element("html", body, None);
    }

    pub fn body(body: &str) -> String {
        return HTML::element("body", body, None);
    }

    pub fn h1(body: &str) -> String {
        return HTML::element("h1", body, None);
    }

    pub fn hr(body: &str) -> String {
        return HTML::element("hr", body, None);
    }

    pub fn table(body: &str) -> String {
        return HTML::element("table", body, None);
    }

    pub fn tr(body: &str) -> String {
        return HTML::element("tr", body, None);
    }

    pub fn th(body: &str) -> String {
        return HTML::element("th", body, None);
    }

    pub fn td(body: &str) -> String {
        return HTML::element("td", body, None);
    }

    pub fn a(body: &str, href: &str) -> String {
        return HTML::element("a", body, Some(&[("href", href)]));
    }

    pub fn element(tag: &str, body: &str, attributes: Option<&[(&str, &str)]>) -> String {
        let mut tag_attributes = String::from("");

        if attributes != None {
            tag_attributes = String::from(" ");

            for a in attributes.unwrap().iter() {
                tag_attributes += format!("{}=\"{}\" ", a.0, a.1).as_str();
            }
        }

        return format!("<{tag}{tag_attributes}>{body}</{tag}>\n");
    }
}