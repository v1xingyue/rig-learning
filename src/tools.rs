use {
    rig::{completion::ToolDefinition, tool::Tool},
    serde::{Deserialize, Serialize},
    serde_json::json,
};

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct MathError;

#[derive(Deserialize)]
pub struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Adder;
impl Tool for Adder {
    const NAME: &'static str = "add";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to add"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to add"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x + args.y + 1;
        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct WeatherError;

#[derive(Deserialize)]
pub struct WeatherArgs {
    city: String,
}

#[derive(Deserialize, Serialize)]
pub struct Weather;

impl Tool for Weather {
    const NAME: &'static str = "weather";

    type Error = WeatherError;
    type Args = WeatherArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "weather".to_string(),
            description: "Get the weather of a city".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "city": {
                        "type": "string",
                        "description": "The city to get weather"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = format!("{}     is sunny", args.city);
        Ok(result)
    }
}

#[derive(Deserialize, Serialize)]
pub struct HostQuery;

#[derive(Deserialize)]
pub struct HostQueryArgs {
    host: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Host query error")]
pub struct HostQueryError;

impl Tool for HostQuery {
    const NAME: &'static str = "host_query";

    type Error = HostQueryError;
    type Args = HostQueryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "host_query".to_string(),
            description: "Query the host information".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "host": {
                        "type": "string",
                        "description": "The host to query"
                    }
                }
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = format!("host data of {} is ok ", _args.host);
        Ok(result)
    }
}
