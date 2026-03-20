use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    pub ok: bool,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            ok: true,
            message: message.into(),
            data,
        }
    }

    pub fn failure(message: impl Into<String>, data: T) -> Self {
        Self {
            ok: false,
            message: message.into(),
            data,
        }
    }
}

