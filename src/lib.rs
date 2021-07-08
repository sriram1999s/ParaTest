use std::fs;
pub struct TestFunctions {
    list: Vec<String>,
}

impl TestFunctions {
    pub fn new() -> TestFunctions {
        TestFunctions {
            list: Vec::new(),
        }
    }
    pub fn add(&mut self, function :&str) {
        self.list.push(String::from(function));
    }

    pub fn display(&self) {
        for func in &self.list {
            println!("{}\n", func);
        }
    }

    /* creating header file */
    pub fn create_header(&self) {
        let mut declarations = String::from("");
        for func in &self.list {
            declarations.push_str(format!("void {}();\n", func).as_str());
        }
        fs::write("tests.hpp", declarations.as_str()).expect("Something went wrong when creating tests.hpp");
    }

    // run all tests // without threading
    pub fn run_tests(&self, impl_file: &str) {
        for func in &self.list {
            let mut main_file = String::from("#include \"tests.hpp\"");
            main_file.push_str(impl_file);
            main_file.push_str(format!("int main() {{\n\t{}();\n}}", func).as_str());
            println!("main_file:\n{}\n\n", main_file);
        }
    }
}
