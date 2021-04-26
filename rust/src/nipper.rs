use nipper::Document;

fn main() {
//   let contents = fs::read_to_string("./src/test.html")
//       .expect("Something went wrong reading the file");

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


//
//   document.select("tr.athing").iter().for_each(|athing| {
//       let title = athing.select(".title a");
//       let href = athing.select(".storylink");
//       println!("{}", title.text());

//       println!();
//   });
}


