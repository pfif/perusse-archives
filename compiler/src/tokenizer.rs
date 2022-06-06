use serde::Deserialize;
use std::collections::HashMap;
use toml;

#[derive(Debug, PartialEq, Default)]
struct Capsule {
    header: Header,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
struct Header {
    titre: String,
    id: String,
    personnages: Vec<String>,
    // TODO Tech debt - Allow for other type of publication
    publications: Vec<PublicationYoutube>,
    calambours: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
struct PublicationYoutube {
    id: String,
    timecode_debut: String,
    timecode_fin: String,
}

// TODO Better error handling (stop using unwrap)
fn tokenize_file(file: String) -> Capsule {
    let header_end: usize = file.find("---").unwrap();
    let header_raw: String = file[..header_end].to_string();
    let header: Header = toml::from_str(&header_raw).unwrap();
    let capsule: Capsule = Capsule { header };
    return capsule;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_tokenized() {
        let file: String = String::from(
            r#" titre = "Émission de variété: 'Calambours stupides'"
id = "FP1FRA"
personnages = ["Annonceur", "Animateur"]

[["publications"]]
type = "youtube"
id = "XpG2nemCuGg"
timecode_debut = "04:02"
timecode_fin = "06:44"

[calambours]
cam = ["qu'amer", "aman"]
per = ["persiste"]
chanaze = ["Charles Aznavour"]
"évi" = ["une vie d'amant"]
Tri = ["très belles jambes"]
---"#,
        );
        let expected_output: Capsule = Capsule {
            header: Header {
                titre: "Émission de variété: 'Calambours stupides'".to_string(),
                id: "FP1FRA".to_string(),
                personnages: Vec::from(["Annonceur".to_string(), "Animateur".to_string()]),
                publications: Vec::from([PublicationYoutube {
                    id: "XpG2nemCuGg".to_string(),
                    timecode_debut: "04:02".to_string(),
                    timecode_fin: "06:44".to_string(),
                }]),
                calambours: HashMap::from([
                    (
                        "cam".to_string(),
                        Vec::from(["qu'amer".to_string(), "aman".to_string()]),
                    ),
                    ("per".to_string(), Vec::from(["persiste".to_string()])),
                    (
                        "chanaze".to_string(),
                        Vec::from(["Charles Aznavour".to_string()]),
                    ),
                    (
                        "évi".to_string(),
                        Vec::from(["une vie d'amant".to_string()]),
                    ),
                    (
                        "Tri".to_string(),
                        Vec::from(["très belles jambes".to_string()]),
                    ),
                ]),
            },
        };

        assert_eq!(expected_output, tokenize_file(file))
    }
}
