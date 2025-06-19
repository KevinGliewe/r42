
/// Parsing state used when transforming templates.
enum TemplateContext {
    /// Plain template text outside of any code block
    Template,
    /// Raw code to be copied directly into the output
    Code,
    /// A Rust expression that will be evaluated and written
    Expression,
}

/// Callback used for writing captured template or expression fragments.
pub type Writer = fn(code_buffer: &mut String, template_buffer: &String);

/// Convert an r42 template into code using the provided writer callbacks.
///
/// * `input` - The raw template source.
/// * `template_writer` - Called whenever plain template text is encountered.
/// * `expression_writer` - Called for expression blocks beginning with `<#=`.
pub fn transform(input: &str, template_writer: &Writer, expression_writer: &Writer) -> String {

    let mut code_buffer = String::with_capacity((input.len() as f32 * 1.2f32) as usize);
    let mut expression_buffer = String::with_capacity(128);
    let mut template_buffer= String::with_capacity(512);

    let mut context = TemplateContext::Template;

    // Convert the input string to a vector so we can peek ahead
    let input:Vec<char> = input.chars().collect();

    let mut idx:usize = 0;
    while idx < input.len() {
        // Look at up to three characters so we can recognise markers
        let c3 = (
            input[idx],
            input.get(idx + 1).unwrap_or(&'\0'),
            input.get(idx + 2).unwrap_or(&'\0')
        );

        match context {
            // Currently reading plain template text
            TemplateContext::Template => {

                match c3 {
                    // Found start of an expression block: `<#=`
                    ('<', '#', '=') => {
                        idx += 2;
                        context = TemplateContext::Expression;
                        template_writer(&mut code_buffer, &template_buffer);
                        template_buffer.clear();
                    }
                    // Found start of a raw code block: `<#`
                    ('<', '#', _) => {
                        idx += 1;
                        context = TemplateContext::Code;
                        template_writer(&mut code_buffer, &template_buffer);
                        template_buffer.clear();
                    }
                    // Normal text: escape characters as needed
                    (_, _, _) => {
                        append_escaped(&mut template_buffer, c3.0);
                    }
                    
                }
            }
            // Inside a raw code block
            TemplateContext::Code => {
                match c3 {
                    // End of code block
                    ('#', '>', _) => {
                        idx += 1;
                        context = TemplateContext::Template;
                    }
                    // Keep code characters verbatim
                    (_, _, _) => {
                        code_buffer.push(c3.0);
                    }
                }
            }   
            // Inside an expression block to be evaluated
            TemplateContext::Expression => {
                match c3 {
                    // End of expression block
                    ('#', '>', _) => {
                        idx += 1;
                        context = TemplateContext::Template;
                        expression_writer(&mut code_buffer, &expression_buffer);
                        expression_buffer.clear();
                    }
                    // Accumulate expression characters
                    (_, _, _) => {
                        expression_buffer.push(c3.0);
                    }
                }
            }
        }

        idx += 1;
    }

    // Flush any remaining buffered text depending on the final state
    match context {
        TemplateContext::Template => {
            if !template_buffer.is_empty() {
                template_writer(&mut code_buffer, &template_buffer);
            }
        }
        TemplateContext::Expression => {
            if !expression_buffer.is_empty() {
                expression_writer(&mut code_buffer, &expression_buffer);
            }
        }
        TemplateContext::Code => {
            // Remaining code fragments have already been pushed directly.
        }
    }

    code_buffer
}

/// Append a character to the buffer while escaping characters that would
/// otherwise break the generated string literal.
fn append_escaped(buffer: &mut String, c: char){
    // Escape quotes so they don't terminate the string literal
    if c == '"' {
        buffer.push('\\');
        buffer.push('"');
    } else if c == '\\' {
        // Escape backslashes themselves
        buffer.push('\\');
        buffer.push('\\');
    } else if c == '\0' {
        // Represent NUL characters as the two byte sequence \0
        buffer.push('\\');
        buffer.push('0');
    } else if c == '\n' {
        // Use escape sequences for control characters
        buffer.push('\\');
        buffer.push('n');
    } else if c == '\r' {
        buffer.push('\\');
        buffer.push('r');
    } else if c == '\t' {
        buffer.push('\\');
        buffer.push('t');
    } else {
        // Normal characters are appended unchanged
        buffer.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_template(code: &mut String, buf: &String) {
        code.push_str(buf);
    }

    fn capture_expression(code: &mut String, buf: &String) {
        code.push_str(buf);
    }

    #[test]
    fn flushes_trailing_template() {
        let result = transform("hello", &(capture_template as Writer), &(capture_expression as Writer));
        assert_eq!(result, "hello");
    }

    #[test]
    fn flushes_trailing_expression() {
        let result = transform("<#=42", &(capture_template as Writer), &(capture_expression as Writer));
        assert_eq!(result, "42");
    }

    #[test]
    fn parses_code_and_expression_blocks() {
        let result = transform("<#let x = 5;#>value:<#=x#>", &(capture_template as Writer), &(capture_expression as Writer));
        assert_eq!(result, "let x = 5;value:x");
    }

    #[test]
    fn escapes_special_characters() {
        let input = "\"\n\r\t\\\0";
        let result = transform(input, &(capture_template as Writer), &(capture_expression as Writer));
        assert_eq!(result, "\\\"\\n\\r\\t\\\\\\0");
    }

    #[test]
    fn handles_unicode() {
        let input = "café";
        let result = transform(input, &(capture_template as Writer), &(capture_expression as Writer));
        assert_eq!(result, "café");
    }
}
