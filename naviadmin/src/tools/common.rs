pub fn get_image_content_type(image_name: &String) -> String {
    let suffix: Vec<&str> = image_name.split(".").collect();
    if suffix.len() > 1 {
        format!("image/{}", suffix[1])
    } else {
        format!("image/jpeg")
    }
}