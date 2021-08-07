use serde::de;
use warp::{reject, Filter, Rejection};

pub fn query<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where T: de::DeserializeOwned + Send + 'static, {
    warp::query::raw()
    .or_else(|_| async {
        Ok::<_, Rejection>((String::new(),))
    })
    .and_then(|query: String| async move {
        let decoded
        =urlencoding::decode(query.as_str())
        .map_err(|_| { reject() })?;
        
        serde_json::from_str::<T>(decoded.into_owned().as_str())
        .map_err(|_err| { reject() })
    })
}