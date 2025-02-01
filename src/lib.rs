use pyo3::prelude::*;
mod shortener;

#[pymodule]
fn short_url(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<shortener::UrlShortener>()?;
    Ok(())
}
