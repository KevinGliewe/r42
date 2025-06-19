
#[path = "r42t.rs"] pub mod r42t;

/// Representation of a supported target language.
#[derive(Copy, Clone)]
pub struct Language {
    /// Human readable name shown in help output
    pub name: &'static str,
    /// File extension used for generated files
    pub extension: &'static str,
    /// Writer for plain template fragments
    pub template_writer: r42t::Writer,
    /// Writer for expression fragments
    pub expression_writer: r42t::Writer,
}

/// List of languages that r42 can generate code for.
/// The order determines the default language when none is specified.
pub const LANGUAGES: [Language; 5] = [
    Language {name: "Rust", extension: "rs", template_writer: rust_template_writer, expression_writer: rust_expression_writer},
    Language {name: "C#", extension: "cs", template_writer: csharp_template_writer, expression_writer: csharp_expression_writer},
    Language {name: "Java", extension: "java", template_writer: java_template_writer, expression_writer: java_expression_writer},
    Language {name: "JavaScript", extension: "js", template_writer: javascript_template_writer, expression_writer: javascript_expression_writer},
    Language {name: "C++", extension: "cpp", template_writer: cpp_template_writer, expression_writer: cpp_expression_writer},
];

// Rust

/// Writes plain text for Rust templates.
fn rust_template_writer(code_buffer: &mut String, template_buffer: &String){
    // Skip empty fragments to avoid extra code
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nbuffer.push_str(\"{}\");\n", template_buffer).as_str());
}

/// Writes a Rust expression result into the output buffer.
fn rust_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    // Nothing to do for empty expressions
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nbuffer.push_str(format!(\"{{:?}}\", {}).as_str());\n", expression_buffer).as_str());
}

// C#

/// Writes plain text for C# templates.
fn csharp_template_writer(code_buffer: &mut String, template_buffer: &String){
    // Skip empty fragments to reduce noise
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nbuffer.Write(\"{}\");\n", template_buffer).as_str());
}

/// Writes the result of a C# expression into the output buffer.
fn csharp_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    // Ignore empty expression segments
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nbuffer.Write(({}).ToString());\n", expression_buffer).as_str());
}

// Java

/// Writes plain text for Java templates.
fn java_template_writer(code_buffer: &mut String, template_buffer: &String){
    // Skip empty segments
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nbuffer.write(\"{}\");\n", template_buffer).as_str());
}

/// Writes the result of a Java expression into the output buffer.
fn java_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    // Ignore empty expression segments
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nbuffer.write(({}).toString());\n", expression_buffer).as_str());
}

// JavaScript

/// Writes plain text for JavaScript templates.
fn javascript_template_writer(code_buffer: &mut String, template_buffer: &String){
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nwrite(\"{}\");\n", template_buffer).as_str());
}

/// Writes the result of a JavaScript expression into the output buffer.
fn javascript_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    // Ignore empty expression segments
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nwrite({});\n", expression_buffer).as_str());
}

// C++

/// Writes plain text for C++ templates.
fn cpp_template_writer(code_buffer: &mut String, template_buffer: &String){
    // Skip empty text fragments
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nwrite(\"{}\");\n", template_buffer).as_str());
}

/// Writes the result of a C++ expression into the output buffer.
fn cpp_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    // Ignore empty expression segments
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nwrite({});\n", expression_buffer).as_str());
}
