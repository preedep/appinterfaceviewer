#[derive(Debug, Clone)]
pub enum CommunicationMethod {
    RESTAPI {
        method: String,
        endpoint: String,
        route_names: Vec<String>, // Store multiple route names
    },
    MQ {
        queue_name: String,
        route_names: Vec<String>, // Store multiple route names
    },
    Kafka {
        topic: String,
        route_names: Vec<String>, // Store multiple route names
    },
    GRPC {
        service_name: String,
        route_names: Vec<String>, // Store multiple route names
    },
    FileTransfer {
        file_path: String,
        route_names: Vec<String>,
    },
    Soap {
        wsdl_url: String,
        route_names: Vec<String>,
    },
}

// Helper function to get route names from CommunicationMethod
impl CommunicationMethod {
    pub fn route_names(&self) -> &Vec<String> {
        match self {
            CommunicationMethod::RESTAPI { route_names, .. }
            | CommunicationMethod::MQ { route_names, .. }
            | CommunicationMethod::Kafka { route_names, .. }
            | CommunicationMethod::GRPC { route_names, .. }
            | CommunicationMethod::FileTransfer { route_names, .. }
            | CommunicationMethod::Soap {route_names,..} => route_names,
        }
    }

    // Helper function to get a string representation of the communication method type
    pub fn communication_type(&self) -> String {
        match self {
            CommunicationMethod::RESTAPI { .. } => "REST API".to_string(),
            CommunicationMethod::MQ { .. } => "MQ".to_string(),
            CommunicationMethod::Kafka { .. } => "Kafka".to_string(),
            CommunicationMethod::GRPC { .. } => "gRPC".to_string(),
            CommunicationMethod::FileTransfer { .. } => "File Transfer".to_string(),
            CommunicationMethod::Soap { .. } => "SOAP".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppInformation {
    name: String,
}
impl AppInformation {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
