#[cfg(test)]
mod tests {
    use super::*;
    use ockam::{Context, TcpTransport, Result};
    use ockam::identity::IdentityVault;

    #[tokio::test]
    async fn test_api_client_send_request_success() -> Result<()> {
        let ctx = Context::new().await?;
        let api_client = ApiClient::new().await?;
        
        // Setup a mock HTTP server for testing
        let mock_server = MockServer::start().await;
        let mock_response = Mock::new()
            .expect_path("/mock-api")
            .expect_method(POST)
            .return_status(200)
            .return_header("content-type", "application/json")
            .return_body(r#"{"result": "success"}"#)
            .create_on(&mock_server)
            .await;

        let payload = serde_json::json!({"data": "test"});
        
        // Create a mock forwarder (this would normally be set up differently)
        let tcp = TcpTransport::create(&ctx).await?;
        let forwarder = tcp.create_forwarder("localhost:4000").await?;
        
        let result = api_client
            .send_request(&ctx, &mock_server.url("/mock-api"), payload, &forwarder)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["result"], "success");

        ctx.stop().await
    }

    #[tokio::test]
    async fn test_api_client_send_request_failure() -> Result<()> {
        let ctx = Context::new().await?;
        let api_client = ApiClient::new().await?;
        
        // Setup a mock HTTP server for testing
        let mock_server = MockServer::start().await;
        let mock_response = Mock::new()
            .expect_path("/mock-api")
            .expect_method(POST)
            .return_status(500)
            .return_header("content-type", "application/json")
            .return_body(r#"{"error": "internal server error"}"#)
            .create_on(&mock_server)
            .await;

        let payload = serde_json::json!({"data": "test"});
        
        // Create a mock forwarder (this would normally be set up differently)
        let tcp = TcpTransport::create(&ctx).await?;
        let forwarder = tcp.create_forwarder("localhost:4000").await?;

        let result = api_client
            .send_request(&ctx, &mock_server.url("/mock-api"), payload, &forwarder)
            .await;

        assert!(result.is_err());

        ctx.stop().await
    }

        #[test]
        fn test_validate_success() {
            let valid_data = json!({
                "required_field": "some_value",
                "other_field": "another_value"
            });

            let result = Validator::validate(&valid_data);
            assert!(result.is_ok());
        }

        #[test]
        fn test_validate_failure() {
            let invalid_data = json!({
                "other_field": "another_value"
            });

            let result = Validator::validate(&invalid_data);
            assert!(result.is_err());
            assert_eq!(format!("{}", result.unwrap_err()), "Validation failed: Missing required field");
        }

        #[test]
        fn test_normalize() {
            let data = json!({
                "FieldOne": "value1",
                "FieldTwo": "value2"
            });

            let normalized = Validator::normalize(&data);

            assert_eq!(normalized["fieldone"], "value1");
            assert_eq!(normalized["fieldtwo"], "value2");
        }
    }
    