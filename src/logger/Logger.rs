pub struct ConsLog {
    line_number: usize,
}

pub trait Logger {
    fn fatal(&self, input: &str);
    fn error(&self, input: &str);
    fn warn(&self, input: &str);
    fn info(&self, input: &str);
    fn debug(&self, input: &str);
    fn trace(&self, input: &str);
}

impl ConsLog {
    pub fn new() -> Self {
        Self { line_number: 0 }
    }
    pub fn increase_line_number(&mut self) {
      self.line_number +=1;
    }
}

impl Logger for ConsLog {
    fn fatal(&self, input: &str) {
        //self.increase_line_number();
        let output = format!("FATAL[{}]: {}", self.line_number, input);
        println!("{}", output);
    }
    fn warn(&self, input: &str) {
        //self.line_number += 1;
        let output = format!("WARN [{}]: {}", self.line_number, input);
        println!("{}", output);
    }
    fn info(&self, input: &str) {
        //self.line_number += 1;
        let output = format!("INFO [{}]: {}", self.line_number, input);
        println!("{}", output);
    }
    fn error(&self, input: &str) {
        //self.line_number += 1;
        let output = format!("ERROR[{}]: {}", self.line_number, input);
        println!("{}", output);
    }
    fn debug(&self, input: &str) {
        //self.line_number += 1;
        let output = format!("DEBUG[{}]: {}", self.line_number, input);
        println!("{}", output);
    }
    fn trace(&self, input: &str) {
        //self.line_number += 1;
        let output = format!("LOG  [{}]: {}", self.line_number, input);
        println!("{}", output);
    }
}

pub struct ScopedLogger {
  verbose: bool,
  prefixes: Vec<String>
}

impl ScopedLogger {
  pub fn new(prefixes: Vec<String>) -> Self{
    Self{
      verbose: false,
      prefixes
    }
  }
  pub fn set_verbose(&mut self, verbose: bool) {
    self.verbose = verbose;
  }
  pub fn extend_scope(&mut self, prefix: String) -> &mut Self {
    self.prefixes.push(prefix);

    return self;
  }
  fn get_prefixes_string(&self) -> String{
    return self.prefixes.iter().map(|i| format!("[{}]", i)).collect::<Vec<String>>().join("")
    
  }
}

impl Logger for ScopedLogger {
  
  fn warn(&self, input: &str) {
    println!("WARN - {}: {}",self.get_prefixes_string(), input)
  }
  fn error(&self, input: &str) {
    println!("ERROR - {}: {}",self.get_prefixes_string(), input);
  }
  fn debug(&self, input: &str) {
    println!("DEBUG - {}: {}",self.get_prefixes_string(), input);
  }
  fn trace(&self, input: &str) {
    println!("TRACE - {}: {}",self.get_prefixes_string(), input);
  } 
  fn fatal(&self, input: &str) {
    println!("FATAL - {}: {}",self.get_prefixes_string(), input);
  } 
  fn info(&self, input: &str) {
    println!("INFO - {}: {}",self.get_prefixes_string(), input);
  } 

}
