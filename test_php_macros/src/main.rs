use pproc as pp;

struct Empty { }

struct ZTS { 
    c: String
}

// struct Iko {}

// trait Zval {}

// impl Zval for Iko {

// }

trait Context {
    fn pass(&self);
} 

impl Context for Empty {
    fn pass(&self) {
        //noop
    }
}

impl Context for ZTS {
    fn pass(&self) {
        // println!("pass {}", self.c)
    }
}

// fn sample_function<T: Context>(_args: &str, c: T) {
//     c.pass();
// }

#[pp::php_callback]
fn sampler_function<T: Context>(_args: &str, c: T) {
    println!("blha");
    c.pass();
}

pub fn main() {
    unsafe { mhh(1,2) };
}

unsafe extern "C" fn k(a: i32, b:i32) -> i32 {
    0
}



const X: &'static [unsafe extern "C" fn(i32, i32) -> i32] = &[mhh, k, k];

