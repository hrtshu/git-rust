use std::io::Write;

pub trait ObjectBase {
    fn obj_type(&self) -> &str;

    fn write_to<W>(&self, writer: &mut W) -> std::io::Result<()> where W: Write {
        self.write_header_to(writer)?;
        self.write_body_to(writer)?;
        Ok(())
    }

    fn write_header_to<W>(&self, writer: &mut W) -> std::io::Result<()> where W: Write {
        write!(writer, "{} {}\0", self.obj_type(), self.body_size())
    }

    fn body_size(&self) -> usize;

    fn write_body_to<W>(&self, writer: &mut W) -> std::io::Result<()> where W: Write;
}
