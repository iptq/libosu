#[macro_use]
extern crate cpython;

py_module_initializer!(libosu, initlibosupy, PyInit_libosu, |py, m| {
    try!(m.add(py, "__doc__", "Python bindings for libosu."));

    Ok(())
});
