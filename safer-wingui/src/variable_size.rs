// MIT/Apache2 License

pub trait VariableSize {
    type Canary;

    fn size(&self) -> usize;
}