
pub fn render(buffer: &mut String) {

buffer.push_str("\r\n\r\n<thing>");

buffer.push_str(format!("{:?}", (5+5)).as_str());

buffer.push_str("</thing>\r\n\r\n");
}