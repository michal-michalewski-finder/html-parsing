use nipper::Document;

fn main() {

    for i in 0..1000 {
        println!("{}", i);
        let contents = include_str!("test.html");
        let document = Document::from(contents);

        document.select("link").iter().for_each(|link| {
            let rel = String::from(link.attr("rel").unwrap());

            if rel == "stylesheet" {
                let href = String::from(link.attr("href").unwrap());
                println!("{}", rel);
                println!("{}", href);
            }
        });
    }
}



