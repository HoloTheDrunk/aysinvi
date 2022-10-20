use std::collections::HashMap;

macro_rules! init_map {
    ($($($keyword:literal)|* => $type:literal ; $code:literal),* $(,)?) => {
        HashMap::<String, String>::from([
            $(
                $(
                    (
                        $keyword.to_string(),
                        concat!(
                            stringify!($type),
                            ';',
                            stringify!($code),
                        ).to_owned()
                    )
                ),*
            ),*
        ])
    };
}

#[derive(PartialEq, Eq)]
enum ElemType {
    Delimiter,
    Word,
}

pub fn highlight_aysinvi(source: &str) -> String {
    let mapping = init_map!(
         "ngop"
        | "'u" | "meu" | "pxeu" | "ayu"
        | "alu" | "txew" => 0;33,

        "lì'ukìng" => 1;33,

         "san" | "sìk" | "ke"
        | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        | "melo" | "pxelo"
        | "teng" => 0;31,

        "fa" | "si" | "livu" => 0;32,

         "txo" | "tsakrr" | "txokefyaw"
        | "leyn" | "vaykrr" | "ftang" => 0;35,

        "sì" | "ulte" => 0;36,
    );

    let mut in_string = false;

    split_words(source)
        .iter()
        .map(move |(elem_type, word)| {
            if *elem_type == ElemType::Word {
                if *word == "san" {
                    in_string = true;
                }

                let res = if !in_string {
                    mapping.get(*word).map_or_else(
                        || format!("\x1b[1;34m{word}\x1b[0m"),
                        |format| format!("\x1b[{format}m{word}\x1b[0m"),
                    )
                } else {
                    format!("\x1b[0;31m{word}\x1b[0m")
                };

                if *word == "sìk" {
                    in_string = false;
                }

                res
            } else {
                (*word).to_owned()
            }
        })
        .collect::<String>()
}

fn split_words(text: &str) -> Vec<(ElemType, &str)> {
    let mut result = Vec::new();
    let mut last = 0;

    for (index, matched) in
        text.match_indices(|c: char| c.is_whitespace() || [',', '.'].contains(&c))
    {
        if last != index {
            result.push((ElemType::Word, &text[last..index]));
        }

        result.push((ElemType::Delimiter, matched));

        last = index + matched.len();
    }

    if last < text.len() {
        result.push((ElemType::Word, &text[last..]));
    }

    result
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_highlighting() {
//
//     }
// }
