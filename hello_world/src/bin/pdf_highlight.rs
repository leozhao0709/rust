use lopdf::Document;

fn main() {
    let doc = Document::load("simpleReport.pdf").expect("Error loading PDF");

    let pages = doc.get_pages();
    for (page_num, page_id) in pages {
        let annotations = doc.get_page_annotations(page_id);
        for annotation in &annotations {
            let value = annotation.get(b"Subtype");
            println!("{annotation:?}");
            // println!("{value:?}");
            print!("------------");
        }
    }
}

// fn process_annotation(annot_id: lopdf::ObjectId, doc: &Document) {
//     if let Ok(annot_dict) = doc.get_dictionary(annot_id) {
//         // Check if the annotation is a highlight
//         if let Some(Object::Name(subtype)) = annot_dict.get(b"Subtype") {
//             if subtype.as_str() == "Highlight" {
//                 // Extract information from the highlight annotation
//                 // ...
//             }
//         }
//     }
// }
