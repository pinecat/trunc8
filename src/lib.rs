pub fn truncate(text: &str, len: usize) -> Vec<String> {
    // Variable to store all the lines from the text input
    let mut lines: Vec<String> = vec![];

    // Split the text on spaces
    let spl = text.split(" ");

    // Create a variable to store words, and loop through each word
    let mut line: String = String::from("");
    for word in spl {
        // Remove newlines if they are present
        let word = format!(" {}", word.trim());

        // Check if concat is too long
        if (line.len() + word.len()) > len + 1 {
            lines.push(line.clone());
            line.clear();
        }

        // Add the word to the line
        line.push_str(word.as_str());
    }

    // Do one final push, in case we have a leftover line
    lines.push(line);

    // Trim lines
    let lines = lines.iter().map(|l| l.trim().to_string()).collect();

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_truncate_a_simple_string() {
        let result = truncate("Hello World", 5);
        assert_eq!(result, vec!["Hello", "World"]);
    }

    #[test]
    fn it_can_truncate_a_longer_string() {
        let len = 80;
        let result = truncate("Hello world! This is a very long piece of text, and I will be using it to test this truncate function. It's actually quite exciting, as there are a few different ways I could truncate the string. The method I'm using is pretty fast and reliable, though, and seems to be a perfect fit for what I want to do. This particular truncate functions works on words, not on individual characters, meaning that the text it produces is readable in paragraph format. Rust's built-in truncate works solely on characters, so you may break up a word between lines.", len);
        assert_eq!(
            result,
            vec![
                "Hello world! This is a very long piece of text, and I will be using it to test",
                "this truncate function. It's actually quite exciting, as there are a few",
                "different ways I could truncate the string. The method I'm using is pretty fast",
                "and reliable, though, and seems to be a perfect fit for what I want to do. This",
                "particular truncate functions works on words, not on individual characters,",
                "meaning that the text it produces is readable in paragraph format. Rust's",
                "built-in truncate works solely on characters, so you may break up a word between",
                "lines."
            ]
        );

        result.iter().for_each(|line| {
            let truncated = line.len() <= len;
            assert_eq!(truncated, true);
        });
    }

    #[test]
    fn it_can_truncate_lorem_ipsum() {
        let len = 120;
        let result = truncate("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ultricies mi eget mauris pharetra et ultrices. Hendrerit dolor magna eget est lorem. Ullamcorper eget nulla facilisi etiam dignissim diam quis enim. Cursus vitae congue mauris rhoncus aenean vel elit scelerisque. Diam vulputate ut pharetra sit amet aliquam id diam maecenas. Molestie ac feugiat sed lectus. Ac turpis egestas integer eget. Blandit libero volutpat sed cras ornare arcu dui vivamus. Volutpat diam ut venenatis tellus. Id porta nibh venenatis cras sed felis eget velit. Fermentum posuere urna nec tincidunt. Massa placerat duis ultricies lacus sed turpis tincidunt id. In fermentum et sollicitudin ac. Tellus pellentesque eu tincidunt tortor aliquam. Adipiscing elit pellentesque habitant morbi tristique senectus et. Lacus sed turpis tincidunt id aliquet risus. Morbi tristique senectus et netus et malesuada fames ac turpis. Ac turpis egestas integer eget aliquet nibh praesent. Orci porta non pulvinar neque laoreet suspendisse. Pharetra pharetra massa massa ultricies mi quis hendrerit dolor magna. Nibh praesent tristique magna sit. Egestas erat imperdiet sed euismod nisi porta lorem mollis aliquam. Sed odio morbi quis commodo odio aenean sed adipiscing. Velit sed ullamcorper morbi tincidunt ornare massa eget. Ultrices eros in cursus turpis massa tincidunt dui. Rhoncus mattis rhoncus urna neque viverra. Nisl purus in mollis nunc sed id semper risus in. Ullamcorper sit amet risus nullam eget felis eget. Egestas sed sed risus pretium quam vulputate dignissim suspendisse. Eu mi bibendum neque egestas congue quisque egestas diam. Enim ut sem viverra aliquet eget sit amet tellus cras. Diam maecenas ultricies mi eget mauris pharetra. Neque volutpat ac tincidunt vitae. A arcu cursus vitae congue mauris rhoncus aenean vel elit. Duis ut diam quam nulla. Tempor orci eu lobortis elementum nibh. Diam phasellus vestibulum lorem sed risus. Senectus et netus et malesuada fames. Euismod quis viverra nibh cras pulvinar mattis. Sapien et ligula ullamcorper malesuada proin libero nunc consequat. Lobortis scelerisque fermentum dui faucibus in ornare quam viverra. Integer vitae justo eget magna fermentum iaculis. Enim diam vulputate ut pharetra sit. Volutpat commodo sed egestas egestas fringilla. Eget nunc lobortis mattis aliquam faucibus purus in massa. Integer enim neque volutpat ac tincidunt vitae semper. Mattis vulputate enim nulla aliquet porttitor lacus luctus accumsan. Amet aliquam id diam maecenas ultricies. Adipiscing elit ut aliquam purus sit amet luctus venenatis lectus. Habitant morbi tristique senectus et netus et malesuada fames. Vitae nunc sed velit dignissim sodales ut. In metus vulputate eu scelerisque felis imperdiet proin fermentum leo. Fermentum posuere urna nec tincidunt praesent semper. Volutpat est velit egestas dui id ornare. Tellus cras adipiscing enim eu turpis egestas pretium aenean. Porta nibh venenatis cras sed felis. Massa sapien faucibus et molestie ac feugiat sed.", len);
        assert_eq!(
            result, 
            vec![
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna",
                "aliqua. Ultricies mi eget mauris pharetra et ultrices. Hendrerit dolor magna eget est lorem. Ullamcorper eget nulla",
                "facilisi etiam dignissim diam quis enim. Cursus vitae congue mauris rhoncus aenean vel elit scelerisque. Diam vulputate",
                "ut pharetra sit amet aliquam id diam maecenas. Molestie ac feugiat sed lectus. Ac turpis egestas integer eget. Blandit",
                "libero volutpat sed cras ornare arcu dui vivamus. Volutpat diam ut venenatis tellus. Id porta nibh venenatis cras sed",
                "felis eget velit. Fermentum posuere urna nec tincidunt. Massa placerat duis ultricies lacus sed turpis tincidunt id. In",
                "fermentum et sollicitudin ac. Tellus pellentesque eu tincidunt tortor aliquam. Adipiscing elit pellentesque habitant",
                "morbi tristique senectus et. Lacus sed turpis tincidunt id aliquet risus. Morbi tristique senectus et netus et malesuada",
                "fames ac turpis. Ac turpis egestas integer eget aliquet nibh praesent. Orci porta non pulvinar neque laoreet",
                "suspendisse. Pharetra pharetra massa massa ultricies mi quis hendrerit dolor magna. Nibh praesent tristique magna sit.",
                "Egestas erat imperdiet sed euismod nisi porta lorem mollis aliquam. Sed odio morbi quis commodo odio aenean sed",
                "adipiscing. Velit sed ullamcorper morbi tincidunt ornare massa eget. Ultrices eros in cursus turpis massa tincidunt dui.",
                "Rhoncus mattis rhoncus urna neque viverra. Nisl purus in mollis nunc sed id semper risus in. Ullamcorper sit amet risus",
                "nullam eget felis eget. Egestas sed sed risus pretium quam vulputate dignissim suspendisse. Eu mi bibendum neque egestas",
                "congue quisque egestas diam. Enim ut sem viverra aliquet eget sit amet tellus cras. Diam maecenas ultricies mi eget",
                "mauris pharetra. Neque volutpat ac tincidunt vitae. A arcu cursus vitae congue mauris rhoncus aenean vel elit. Duis ut",
                "diam quam nulla. Tempor orci eu lobortis elementum nibh. Diam phasellus vestibulum lorem sed risus. Senectus et netus et",
                "malesuada fames. Euismod quis viverra nibh cras pulvinar mattis. Sapien et ligula ullamcorper malesuada proin libero",
                "nunc consequat. Lobortis scelerisque fermentum dui faucibus in ornare quam viverra. Integer vitae justo eget magna",
                "fermentum iaculis. Enim diam vulputate ut pharetra sit. Volutpat commodo sed egestas egestas fringilla. Eget nunc",
                "lobortis mattis aliquam faucibus purus in massa. Integer enim neque volutpat ac tincidunt vitae semper. Mattis vulputate",
                "enim nulla aliquet porttitor lacus luctus accumsan. Amet aliquam id diam maecenas ultricies. Adipiscing elit ut aliquam",
                "purus sit amet luctus venenatis lectus. Habitant morbi tristique senectus et netus et malesuada fames. Vitae nunc sed",
                "velit dignissim sodales ut. In metus vulputate eu scelerisque felis imperdiet proin fermentum leo. Fermentum posuere",
                "urna nec tincidunt praesent semper. Volutpat est velit egestas dui id ornare. Tellus cras adipiscing enim eu turpis",
                "egestas pretium aenean. Porta nibh venenatis cras sed felis. Massa sapien faucibus et molestie ac feugiat sed."
            ]
        );

        result.iter().for_each(|line| {
            let truncated = line.len() <= len;
            assert_eq!(truncated, true);
        });
    }

    #[test]
    fn it_works_with_urls() {
        let len = 20;
        let result = truncate("Hello this is an example with a URL, to test how the library will handle that: https://github.com/pinecat/trunc8. For reference, please see the aforementioned repository for bug reports and pull requests.", len);
        assert_eq!(result, vec!["Hello this is an", "example with a URL,", "to test how the", "library will handle", "that:", "https://github.com/pinecat/trunc8.", "For reference,", "please see the", "aforementioned", "repository for bug", "reports and pull", "requests."]);
    }
}
