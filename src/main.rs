use std::io::Write;
use std::str::FromStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use cpython::{Python, PyDict, PyResult, PyModule};

use serde::{Serialize, Deserialize};
use futures::executor::block_on;


#[derive(Serialize, Deserialize, Debug)]
struct Message {
   contents: String,
   real: Option<String>,
}


#[derive(Debug)]
struct Absolute;

const MY_MODULE: &'static str = include_str!("./pyface/utils/baseline.py");


async fn process<T>(y:i64, x:i64) -> Option<(T, T)> 
    where T: FromStr + Display
{
    if let mut z = T::from_str("3") {
        match z {
            Ok(m) => println!("Let their be -> {}", m),
            _ => eprintln!("Error in if let "),
        }
    }
    let product = y * x;
    let mut msg = Box::<Message>::new(Message {contents: "test".to_string(), real: Some("yes".to_string())});
    let mut real = msg.real.take();
    println!("Is msg real {:?}", real);

    let mut for_me = Message {contents: "test".to_string(), real: Some("yes".to_string())};
    real = for_me.real.take();
    println!("Is for_me real {:?}", real);

    Box::<Absolute>::new(Absolute);
    println!("In Async {}", product);
    async {
        let pi: f64 = std::f64::consts::PI;
        println!("This is pi {:?}", pi);
        println!("Am i getting the hang of this.")
    }.await;
    return None
}


fn call_python() -> PyResult<Message> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let m = module_from_str(py, "baseline", MY_MODULE)?;
    let out: String = m.call(py, "read_data", (2,), None)?.extract(py)?;
    println!("{:?}", out);
    Ok(Message { contents: "good".to_string(), real: Some("yes".to_string())})
}


fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    let m = PyModule::new(py, name)?;
    m.add(py, "__builtins__", py.import("builtins")?)?;
    let m_locals = m.get(py, "__dict__")?.extract(py)?;
    py.run(source, Some(&m_locals), None)?;
    Ok(m)
}


fn main() {
    // An
    Absolute;
    // Unit

    // tautology
    let large: Vec<i32> = (0..1000).collect::<Vec<i32>>();

    let mut numbers: Vec<u8> = Vec::new();
    let writer: &mut dyn Write = &mut numbers;

    println!("Hello, world!");
    let mut d = u8::from_str("2");
    match d.as_mut(){
        Ok(v) => numbers.push(*v),
        //Ok(v) => writer.write(*v),
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

#[cfg(test)]
mod tests{
    use super::*;

    async fn async_fun() {
        assert_eq!(process::<i32>(1, 2).await, None);
        assert_eq!(process::<i32>(5, 6).await, None);
    }

    #[test]
    fn test_async_functions(){
        let entry_point = async move {
            println!("Entering async test");
            async_fun().await;
        };
        block_on(entry_point);
    }

    #[test]
    fn test_scope(){
        println!("Entering lifetime test");
        'search:
            for i in 1..3 {
                println!("Testing life time break {:?}", i);
                break 'search;
            }
        fn serve() -> !{
            loop {
                let s = "test".to_string();
            }
        }
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
        let py_sys = py.import("sys")?;
        let py_os = py.import("os")?;
        locals.set_item(py, "sys", py_sys)?;
        locals.set_item(py, "os", py_os)?;
        py.eval("print(sys.path)", None, Some(&locals))?;
        py.eval(r##"sys.path.append(os.path.join(os.path.realpath(os.path.curdir), "src"))"##, None, Some(&locals))?;
        py.eval("print(sys.path)", None, Some(&locals))?;
        // __file__ is not defined tho ...
        //sys.path.append(os.path.join(os.path.dirname(__file__), "lib"))
        let unit = py.import("unittest")?;
        let m = PyModule::import(py, "pyface")?;

        locals.set_item(py, "unittest", unit)?;
        locals.set_item(py, "pyface", m)?;


        match py.eval("print(dir(pyface))", None, Some(&locals)){
            Ok(v) => (),
            Err(e) => writeln!(std::io::stderr(), "Python error {:?}", e).unwrap(),
        }
        match py.eval("unittest.main(module='pyface.utils.test', verbosity=2, exit=False)", None, Some(&locals)){
            Ok(v) => {
                ()
            }
            Err(e) => writeln!(std::io::stderr(), "Python error {:?}", e).unwrap(),
        }

        Ok(())
    }
}
