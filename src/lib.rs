use pyo3::prelude::*;
use pyo3::types::PyAny;
use std::collections::HashSet;

/// Simple greeting function (your original test)
#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Hello, {}!", name))
}

/// Return shallow memory size of a Python object (like sys.getsizeof)
#[pyfunction]
fn sizeof<'py>(py: Python<'py>, obj: Bound<'py, PyAny>) -> PyResult<usize> {
    let sys = py.import("sys")?;
    let size: usize = sys.call_method1("getsizeof", (obj,))?.extract()?;
    Ok(size)
}

fn deep_sizeof_inner(
    _py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    seen: &mut HashSet<usize>,
    sys: &Bound<'_, PyAny>,
    gc: &Bound<'_, PyAny>,
) -> PyResult<usize> {
    // id(obj)
    let obj_id: usize = obj.as_ptr() as usize;

    // evita ciclos / dupla contagem
    if seen.contains(&obj_id) {
        return Ok(0);
    }
    seen.insert(obj_id);

    // tamanho shallow
    let mut size: usize = sys.call_method1("getsizeof", (obj,))?.extract()?;

    // pega referências internas via gc
    let referents = gc.call_method1("get_referents", (obj,))?;

    for ref_obj in referents.try_iter()? {
        let ref_obj = ref_obj?;
        size += deep_sizeof_inner(_py, &ref_obj, seen, sys, gc)?;
    }

    Ok(size)
}

#[pyfunction]
fn deep_sizeof(py: Python<'_>, obj: Bound<'_, PyAny>) -> PyResult<usize> {
    let mut seen = HashSet::new();
    let sys = py.import("sys")?;
    let gc = py.import("gc")?;
    deep_sizeof_inner(py, &obj, &mut seen, &sys, &gc)
}

/// Version function (useful for VS Code later)
#[pyfunction]
fn fathom_version() -> PyResult<&'static str> {
    Ok("0.1.0")
}

#[pymodule]
fn pyfathom(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(sizeof, m)?)?;
    m.add_function(wrap_pyfunction!(deep_sizeof, m)?)?;
    m.add_function(wrap_pyfunction!(fathom_version, m)?)?;

    m.add("__version__", "0.1.0")?;
    Ok(())
}
