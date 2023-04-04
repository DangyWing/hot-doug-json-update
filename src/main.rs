use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Attributes {
    trait_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    description: String,
    image: String,
    creators: String,
    attributes: Vec<Attributes>,
}

fn main() {
    for i in 0..6969 {
        let input_path = format!("./../../../HotDougs/json/{i}.json");
        let output_path = format!("./../../../HotDougs/newjson");

        let data = std::fs::read_to_string(input_path).unwrap();
        // let serde_data: serde_json::Value = serde_json::from_str(data.as_str()).unwrap();
        let serde_data: Metadata = serde_json::from_str(data.as_str()).unwrap();

        println!("serde_data before:{:?}", serde_data.image.as_str());

        // replace image with new string
        let new_image =
            format!("https://bafybeidt47j5vxce2rnxiojieot2hcg5tzygmpxwyh5gdkucy5qw6cpar4.ipfs.dweb.link/{i}.png")
                .to_string();

        let new_data = Metadata {
            image: new_image,
            ..serde_data
        };

        println!("serde_data after:{:?}", new_data.image.as_str());
        if !std::path::Path::new(output_path.as_str()).exists() {
            std::fs::create_dir_all(output_path.clone()).unwrap();
        }

        std::fs::write(
            format!("./../../../HotDougs/newjson/{i}.json"),
            serde_json::to_string_pretty(&new_data).unwrap(),
        )
        .unwrap()
    }
}
