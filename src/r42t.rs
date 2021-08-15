
enum TemplateContext {
    Template,
    Code,
    Expression,
}

pub type Writer = fn(code_buffer: &mut String, template_buffer: &String);

pub fn transform(input: &str, template_writer: &Writer, expression_writer: &Writer) -> String {

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
                        template_writer(&mut code_buffer, &template_buffer);
                        template_buffer.clear();
                    }
                    ('<', '#', _) => {
                        idx += 1;
                        context = TemplateContext::Code;
                        template_writer(&mut code_buffer, &template_buffer);
                        template_buffer.clear();
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
                        expression_writer(&mut code_buffer, &expression_buffer);
                        expression_buffer.clear();
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