use std::io::Write;
use std::str::FromStr;
use std::path::{Path, PathBuf};

use cpython::{Python, PyDict, PyResult, PyModule};

use serde::{Serialize, Deserialize};
use futures::executor::block_on;


#[derive(Serialize, Deserialize, Debug)]
struct Message {
   contents: String,
}


#[derive(Debug)]
struct Absolute;

const MyModule: &'static str = include_str!("./pyface/utils/baseline.py");


async fn process<T:FromStr>(y:i64, x:i64) -> Option<(T, T)> {
    let mut z = T::from_str("test");
    let product = y * x;
    let msg = Box::<Message>::new(Message {contents: "test".to_string()});
    Box::<Absolute>::new(Absolute);
    println!("In Async {}", product);
    let does_work = async {
        println!("Am i getting the hang of this.")
    }.await;
    return None
}


fn call_python() -> PyResult<Message> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let m = module_from_str(py, "baseline", MyModule)?;
    let out: String = m.call(py, "read_data", (2,), None)?.extract(py)?;
    println!("{:?}", out);
    Ok(Message { contents: "good".to_string()})
}


fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    let m = PyModule::new(py, name)?;
    m.add(py, "__builtins__", py.import("builtins")?)?;
    let m_locals = m.get(py, "__dict__")?.extract(py)?;
    py.run(source, Some(&m_locals), None)?;
    Ok(m)
}


fn main() {
    Absolute;
    let mut numbers: Vec<u64> = Vec::new();
    println!("Hello, world!");
    let mut d = u64::from_str("2");
    match d.as_mut(){
        Ok(v) => numbers.push(*v),
        Err(e) => writeln!(std::io::stderr(), "test {:?}", e).unwrap(),
    }
    numbers.push(d.expect("something went wrong"));
    println!("{:?}", numbers);
    let entry_point = async move {
        process::<i32>(6, 2).await;
        process::<i32>(4, 2).await;
    };
    block_on(entry_point);

    println!("{}", "test");
    let m:[i16;4];

    match call_python(){
        Ok(v) => (),
        Err(e) => writeln!(std::io::stderr(), "Python error {:?}", e).unwrap(),
    }
    std::process::exit(0);
}


async fn test_me() {
    assert_eq!(process::<i32>(1, 2).await, None);
    assert_eq!(process::<i32>(5, 6).await, None);
}

#[test]
fn test_async_functions(){
    let entry_point = async move {
        test_me().await;
    };
    block_on(entry_point);
}

#[test]
fn test_python_call(){
    match call_python(){
        Ok(v) => (),
        Err(e) => (),
    }
}

#[test]
fn test_python_unittest() -> PyResult<()>{
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    let unit = py.import("unittest")?;
    const py_tests: &'static str = include_str!("./pyface/__init__.py");
    let m = module_from_str(py, "pyface", py_tests)?;
    locals.set_item(py, "unittest", unit)?;
    locals.set_item(py, "pyface", m)?;
    match py.eval("print(dir(pyface))", None, Some(&locals)){
        Ok(v) => (),
        Err(e) => writeln!(std::io::stderr(), "Python error {:?}", e).unwrap(),
    }
    match py.eval("unittest.main(module='pyface.utils.test', verbosity=2, exit=False)", None, Some(&locals)){
        Ok(v) => (),
        Err(e) => writeln!(std::io::stderr(), "Python error {:?}", e).unwrap(),
    }

    Ok(())
}
