use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GeoLoc {
    pub lat: String,
    pub lng: String,
}
impl GeoLoc {

}
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: GeoLoc,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Company {
    pub name: String,
    //#[wasm_bindgen(js_name = catchPhrase)]
    pub catchPhrase: String,
    pub bs: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize, Debug)]
struct UserData {
    users: Vec<User>
}
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn run(repo: String) -> Result<UserData, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = format!("https://jsonplaceholder.typicode.com/users");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    let users: Vec<User> = serde_wasm_bindgen::from_value(json)?;

    // Send the JSON response back to JS.
    Ok(UserData {users: users})
}

#[wasm_bindgen]
pub fn rtn_js(data: UserData) -> Result<JsValue, JsValue> {
    Ok(serde_wasm_bindgen::to_value(&data)?)
}