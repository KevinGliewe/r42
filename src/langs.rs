
#[path = "r42t.rs"] mod r42t;

pub struct Language {
    pub name: &'static str,
    pub extension: &'static str,
    pub template_writer: r42t::Writer,
    pub expression_writer: r42t::Writer,
}

pub const LENGUAGES: [Language; 4] = [
    Language {name: "Rust", extension: "rs", template_writer: rust_template_writer, expression_writer: rust_expression_writer},
    Language {name: "C#", extension: "cs", template_writer: csharp_template_writer, expression_writer: csharp_expression_writer},
    Language {name: "Java", extension: "java", template_writer: java_template_writer, expression_writer: java_expression_writer},
    Language {name: "JavaScript", extension: "js", template_writer: javascript_template_writer, expression_writer: javascript_expression_writer},
];

// Rust

fn rust_template_writer(code_buffer: &mut String, template_buffer: &String){
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nbuffer.push_str(\"{}\");\n", template_buffer).as_str());
}

fn rust_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nbuffer.push_str(format!(\"{{:?}}\", {}).as_str());\n", expression_buffer).as_str());
}

// C#

fn csharp_template_writer(code_buffer: &mut String, template_buffer: &String){
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nbuffer.Write(\"{}\");\n", template_buffer).as_str());
}

fn csharp_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nbuffer.Write(({}).ToString());\n", expression_buffer).as_str());
}

// Java

fn java_template_writer(code_buffer: &mut String, template_buffer: &String){
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nbuffer.write(\"{}\")\n", template_buffer).as_str());
}

fn java_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nbuffer.write(({}).toString())\n", expression_buffer).as_str());
}

// JavaScript

fn javascript_template_writer(code_buffer: &mut String, template_buffer: &String){
    if template_buffer.len() == 0 { return; }
    code_buffer.push_str(format!("\nwrite(\"{}\")\n", template_buffer).as_str());
}

fn javascript_expression_writer(code_buffer: &mut String, expression_buffer: &String){
    if expression_buffer.len() == 0 { return;}
    code_buffer.push_str(format!("\nwrite({})\n", expression_buffer).as_str());
}