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

// #[pp::php_callback]
// fn sampler_function<T: Context>(_args: &str, c: T) {
//     println!("blha");
//     c.pass();
// }

#[pp::php_module("name", "version")]
mod yellow {

    #[module_init]
    fn init() {

    }

    // module_init!(path_to_fn); // TODO
}

pub fn main() {
    // unsafe { mhh(1,2) };
}

