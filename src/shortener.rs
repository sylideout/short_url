use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use rand::prelude::*;


#[pyclass]
pub struct UrlShortener {
    url_map: Mutex<HashMap<String, String>>,
}

#[pymethods]
impl UrlShortener {
    #[new]
    fn new() -> Self {
        UrlShortener {
            url_map: Mutex::new(HashMap::new())
        }
    }

    #[getter]
    fn url_map(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.url_map.lock().unwrap().clone())
    }

    fn shorten(&self, s: String) -> String {
        let code = generate_six_char_guid64();
        self.url_map
            .lock()
            .unwrap()
            .insert(code.clone(), s);
        code
    }

    fn retrieve(&self, code: String) -> String {
        let lock = self.url_map.lock().unwrap();
        lock.get(&code).unwrap().clone()
    }
}

fn generate_six_char_guid64() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const GUID_LENGTH: usize = 6;
    let mut rng = rand::rng();
    let guid: String = (0..GUID_LENGTH)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    guid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_six_char_guid64() {
        let random_six_digit_guid: String = generate_six_char_guid64();
        assert_eq!(random_six_digit_guid.len(), 6);
    }

}
