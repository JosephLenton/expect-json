use crate::internals::pretty_formatter::PrettyDisplay;
use std::fmt::Arguments;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct PrettyFormatter<'a, 'b> {
    indentation: usize,
    formatter: &'a mut Formatter<'b>,
}

impl<'a, 'b> PrettyFormatter<'a, 'b> {
    pub fn new(formatter: &'a mut Formatter<'b>) -> Self {
        Self {
            indentation: 0,
            formatter,
        }
    }

    pub fn write_pretty_fmt<D>(&mut self, value: &D) -> FmtResult
    where
        D: PrettyDisplay,
    {
        value.pretty_fmt(self)
    }

    pub fn write_display<D>(&mut self, value: &D) -> FmtResult
    where
        D: Display,
    {
        write!(self.formatter, "{value}")
    }

    pub fn write_fmt(&mut self, arguments: Arguments<'_>) -> FmtResult {
        self.formatter.write_fmt(arguments)
    }
}

impl<'a, 'b> From<&'a mut Formatter<'b>> for PrettyFormatter<'a, 'b> {
    fn from(formatter: &'a mut Formatter<'b>) -> Self {
        Self::new(formatter)
    }
}

impl<'a, 'b> Deref for PrettyFormatter<'a, 'b> {
    type Target = Formatter<'b>;

    fn deref(&self) -> &Self::Target {
        self.formatter
    }
}

impl<'a, 'b> DerefMut for PrettyFormatter<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.formatter
    }
}
