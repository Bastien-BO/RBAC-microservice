


#[get("/")]
fn get_permission() -> Json<&'static str> {
    Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[post("/")]
fn post_permission() -> Json<&'static str> {
    Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}