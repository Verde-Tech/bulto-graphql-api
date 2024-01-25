//use services::user_management::user_management_client::UserManagementClient;

// use self::services::user_management::CreateUserRequest;

pub mod services;

// pub async fn test_request() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = UserManagementClient::connect("http://[::1]:50051").await?;

//     let request = tonic::Request::new(CreateUserRequest { email: "".to_string(), password: "".to_string(), name: "".to_string(), },);

//     let response = client.create_user(request).await?;
//     println!("{:#?}", response);
//     Ok(())
// }
