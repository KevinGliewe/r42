enum TemplateContext {
    Template,
    Code,
    Expression,
}

pub fn transform(input: &str) -> String {

    let mut code_buffer = String::with_capacity((input.len() as f32 * 1.2f32) as usize);
    let mut expression_buffer = String::with_capacity(128);
    let mut template_buffer= String::with_capacity(512);

    let mut context = TemplateContext::Template;

    let input:Vec<char> = input.chars().collect();

    let mut idx:usize = 0;
    while idx < input.len() {
        let c3 = (
            input[idx],
            input.get(idx + 1).unwrap_or(&'\0'),
            input.get(idx + 2).unwrap_or(&'\0')
        );

        match context {
            TemplateContext::Template => {

                match c3 {
                    ('<', '#', '=') => {
                        idx += 2;
                        context = TemplateContext::Expression;
                        write_template(&mut code_buffer, &mut template_buffer);
                    }
                    ('<', '#', _) => {
                        idx += 1;
                        context = TemplateContext::Code;
                        write_template(&mut code_buffer, &mut template_buffer);
                    }
                    (_, _, _) => {
                        append_escaped(&mut template_buffer, c3.0);
                    }
                    
                }
            }
            TemplateContext::Code => {
                match c3 {
                    ('#', '>', _) => {
                        idx += 1;
                        context = TemplateContext::Template;
                    }
                    (_, _, _) => {
                        code_buffer.push(c3.0);
                    }
                }
            }   
            TemplateContext::Expression => {
                match c3 {
                    ('#', '>', _) => {
                        idx += 1;
                        context = TemplateContext::Template;
                        write_expression(&mut code_buffer, &mut expression_buffer);
                    }
                    (_, _, _) => {
                        expression_buffer.push(c3.0);
                    }
                }
            }
        }

        idx += 1;
    }

    return code_buffer;
}

fn append_escaped(buffer: &mut String, c: char){
    if c == '"' {
        buffer.push('\\');
        buffer.push('"');
    } else if c == '\\' {
        buffer.push('\\');
        buffer.push('\\');
    } else if c == '\0' {
        buffer.push('\\');
        buffer.push('0');
    } else if c == '\n' {
        buffer.push('\\');
        buffer.push('n');
    } else if c == '\r' {
        buffer.push('\\');
        buffer.push('r');
    } else if c == '\t' {
        buffer.push('\\');
        buffer.push('t');
    } else {
        buffer.push(c);
    }
}

fn write_template(code_buffer: &mut String, template_buffer: &mut String){
    if template_buffer.len() == 0 {
        return;
    }

    let code = format!("buffer.push_str(\"{}\");", template_buffer);
    template_buffer.clear();
    code_buffer.push('\n');
    code_buffer.push_str(code.as_str());
    code_buffer.push('\n');
}

fn write_expression(code_buffer: &mut String, expression_buffer: &mut String){
    if expression_buffer.len() == 0 {
        return;
    }

    let code = format!("buffer.push_str(format!(\"{{:?}}\", {}).as_str());", expression_buffer);
    expression_buffer.clear();
    code_buffer.push('\n');
    code_buffer.push_str(code.as_str());
    code_buffer.push('\n');
}