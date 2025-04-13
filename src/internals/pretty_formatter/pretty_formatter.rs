use crate::internals::pretty_formatter::PrettyDisplay;
use std::fmt::Arguments;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Deref;
use std::ops::DerefMut;

const INDENTATION_SIZE: usize = 4;

pub struct PrettyFormatter<'a, 'b> {
    indentation: usize,
    formatter: &'a mut Formatter<'b>,
}

impl<'a, 'b> PrettyFormatter<'a, 'b> {
    pub fn new(formatter: &'a mut Formatter<'b>) -> Self {
        Self {
            indentation: INDENTATION_SIZE,
            formatter,
        }
    }

    pub fn write_fmt(&mut self, arguments: Arguments<'_>) -> FmtResult {
        self.formatter.write_fmt(arguments)
    }

    pub fn write_fmt_array<V>(&mut self, items: &[V]) -> FmtResult
    where
        V: PrettyDisplay,
    {
        write!(self.formatter, "[")?;
        for (i, item) in items.iter().enumerate() {
            if item.is_indenting() {
                self.increment_indentation();

                if i == 0 {
                    self.write_indentation()?;
                } else {
                    for _ in 0..INDENTATION_SIZE {
                        write!(self.formatter, " ")?;
                    }
                }
            }

            item.pretty_fmt(self)?;

            if i < items.len() - 1 {
                write!(self.formatter, ",")?;

                if items.get(i + 1).is_some_and(|v| !v.is_indenting()) {
                    write!(self.formatter, " ")?;
                }
            }

            if item.is_indenting() {
                self.decrement_indentation();
                self.write_indentation()?;
            }
        }
        write!(self.formatter, "]")?;

        Ok(())
    }

    pub fn write_fmt_object<'i, I, V>(&mut self, items: I) -> FmtResult
    where
        I: IntoIterator<Item = (&'i String, &'i V)>,
        V: PrettyDisplay + 'i,
    {
        write!(self.formatter, "{{")?;
        self.increment_indentation();

        let mut is_empty = true;
        for (i, (key, value)) in items.into_iter().enumerate() {
            is_empty = false;

            if i > 0 {
                write!(self.formatter, ",")?;
            }

            self.write_indentation()?;

            write!(self.formatter, r#""{key}": "#)?;
            value.pretty_fmt(self)?;
        }

        self.decrement_indentation();
        if !is_empty {
            self.write_indentation()?;
        }

        write!(self.formatter, "}}")?;

        Ok(())
    }

    pub fn increment_indentation(&mut self) {
        self.indentation += INDENTATION_SIZE;
    }

    pub fn decrement_indentation(&mut self) {
        self.indentation -= INDENTATION_SIZE;
    }

    pub fn write_indentation(&mut self) -> FmtResult {
        writeln!(self.formatter)?;

        for _ in 0..self.indentation {
            write!(self.formatter, " ")?;
        }

        Ok(())
    }
}

impl<'a, 'b> From<&'a mut Formatter<'b>> for PrettyFormatter<'a, 'b> {
    fn from(formatter: &'a mut Formatter<'b>) -> Self {
        Self::new(formatter)
    }
}

impl<'b> Deref for PrettyFormatter<'_, 'b> {
    type Target = Formatter<'b>;

    fn deref(&self) -> &Self::Target {
        self.formatter
    }
}

impl DerefMut for PrettyFormatter<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.formatter
    }
}
