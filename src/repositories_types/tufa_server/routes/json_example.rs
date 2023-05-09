pub async fn json_example() -> impl actix_web::Responder {
    println!("json example");
    actix_web::web::Json(crate::json_example::JsonExample {
        first: "first_value_json_example".to_string(),
        second: "second_value_json_example".to_string(),
    })
}
