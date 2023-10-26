pub struct Value(f64);

impl Value {
    pub fn new(val: f64) -> Self {
        Self(val)
    }

    pub fn from_value(&self) -> Self {
        Self::new(self.0)
    }
}

impl std::ops::Deref for Value {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}

pub struct ValueVec(Vec<Value>);

impl std::ops::Deref for ValueVec {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ValueVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ValueVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn write(&mut self, value: Value) {
        self.0.push(value);
    }
}

impl Default for ValueVec {
    fn default() -> Self {
        Self::new()
    }
}
