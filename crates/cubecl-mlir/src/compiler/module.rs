#[derive(Debug, Clone, Copy)]
pub struct MlirRepresentation;

impl std::fmt::Display for MlirRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MlirRepresentation")
    }
}
