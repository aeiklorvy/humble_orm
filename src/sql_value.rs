/// Represents any value that can be translated into an SQL string
pub trait SqlValue {
    /// Defines how a value should be translated into an SQL string.
    fn to_sql(&self) -> String;
}

impl SqlValue for String {
    fn to_sql(&self) -> String {
        // use debug trait to escape all quotes
        format!("{self:?}")
    }
}

impl SqlValue for &str {
    fn to_sql(&self) -> String {
        // use debug trait to escape all quotes
        format!("{self:?}")
    }
}

impl SqlValue for i32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlValue for u32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlValue for i64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlValue for u64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlValue for f64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlValue for bool {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlValue for time::Date {
    fn to_sql(&self) -> String {
        let y = self.year();
        let m = self.month() as u8;
        let d = self.day();
        format!("\"{y:04}-{m:02}-{d:02}\"")
    }
}

impl SqlValue for time::Time {
    fn to_sql(&self) -> String {
        let h = self.hour();
        let m = self.minute();
        let s = self.second();
        format!("\"{h:02}:{m:02}:{s:02}\"")
    }
}

impl SqlValue for time::PrimitiveDateTime {
    fn to_sql(&self) -> String {
        let y = self.year();
        let m = self.month() as u8;
        let d = self.day();
        let h = self.hour();
        let mm = self.minute();
        let s = self.second();
        format!("\"{y:04}-{m:02}-{d:02}T{h:02}:{mm:02}:{s:02}\"")
    }
}
