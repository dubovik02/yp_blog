use tonic::{Request, metadata::MetadataValue, transport::Channel};

use crate::{
    blog_proto::{CreatePostRequest, CreatePostResponse, CreateRegisterRequest, 
        DeletePostRequest, DeletePostResponse, GetPostRequest, GetPostResponse, ListPostsRequest, 
        ListPostsResponse, LoginRequest, LoginResponse, RegisterResponse, 
        UpdatePostRequest, UpdatePostResponse, 
        blog_service_client::BlogServiceClient}, error::ClientError
    };

pub struct BlogGrpcClient {
    url: String
}

impl BlogGrpcClient {

    pub fn new(url: String) -> Self {
        Self { url }
    }


    async fn connect(&self) -> Result<BlogServiceClient<Channel>, ClientError> {
        let channel = Channel::from_shared(self.url.to_string())
            .map_err(|e| ClientError::InvalidRequestError(e.to_string()))?
            .connect()
            .await?;
        Ok(BlogServiceClient::new(channel))
    }

    // pub async fn register(&self, url: &str, req: CreateRegisterRequest) -> Result<RegisterResponse, ClientError> {
    //     let mut client = self.connect(url).await?;
    //     let res = client.register(req).await?.into_inner();
    //     Ok(res)
    // }

    // pub async fn login(&self, url: &str, req: LoginRequest) -> Result<LoginResponse, ClientError> {
    //     let mut client = self.connect(url).await?;
    //     let res = client.login(req).await?.into_inner();
    //     Ok(res)
    // }

    // pub async fn new_post(&self, url: &str, token: &str, req_data: CreatePostRequest) -> Result<CreatePostResponse, ClientError> {
    //     let mut client = self.connect(url).await?;
    //     let mut req = Request::new(req_data);
    //     let val: MetadataValue<_> = format!("Bearer {token}")
    //         .parse()
    //         .map_err(|e: tonic::metadata::errors::InvalidMetadataValue| {
    //             ClientError::InvalidRequestError(format!("invalid token format: {e}"))
    //         })?;
    //     req.metadata_mut().insert("authorization", val);
    //     let res = client.create_post(req).await?.into_inner();
    //     Ok(res)
    // }

    // pub async fn get_post(&self, url: &str, id: i64) -> Result<GetPostResponse, ClientError> {
    //     let mut client = self.connect(url).await?;
    //     let res = client.get_post(GetPostRequest { post_id: id }).await?.into_inner();
    //     Ok(res)
    // }

    // pub async fn update_post(&self, url: &str, token: &str, req_data: UpdatePostRequest) -> Result<UpdatePostResponse, ClientError> {
    //     let mut client = self.connect(url).await?;
    //     let mut req = Request::new(req_data);
    //     let val: MetadataValue<_> = format!("Bearer {token}")
    //         .parse()
    //         .map_err(|e: tonic::metadata::errors::InvalidMetadataValue| {
    //             ClientError::InvalidRequestError(format!("invalid token format: {e}"))
    //         })?;
    //     req.metadata_mut().insert("authorization", val);
    //     let res = client.update_post(req).await?.into_inner();
    //     Ok(res)
    // }

    // pub async fn del_post(&self, url: &str, token: &str, req_data: DeletePostRequest) -> Result<DeletePostResponse, ClientError> {
    //     let mut client = self.connect(url).await?;
    //     let mut req = Request::new(req_data);
    //     let val: MetadataValue<_> = format!("Bearer {token}")
    //         .parse()
    //         .map_err(|e: tonic::metadata::errors::InvalidMetadataValue| {
    //             ClientError::InvalidRequestError(format!("invalid token format: {e}"))
    //         })?;
    //     req.metadata_mut().insert("authorization", val);
    //     let res = client.delete_post(req).await?.into_inner();
    //     Ok(res)
    // }

    // pub async fn posts_list(&self, url: &str, req_data: ListPostsRequest) -> Result<ListPostsResponse, ClientError> {
    //     let mut client = self.connect(url).await?;
    //     let res = client.list_posts(req_data).await?.into_inner();
    //     Ok(res)
    // }

}

//impl BlogClientInterface for BlogGrpcClient {
impl BlogGrpcClient {

    pub async fn register(&self, req: CreateRegisterRequest) -> Result<RegisterResponse, ClientError> {
        let mut client = self.connect().await?;
        let res = client.register(req).await?.into_inner();
        Ok(res)
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, ClientError> {
        let mut client = self.connect().await?;
        let res = client.login(req).await?.into_inner();
        Ok(res)
    }

    pub async fn new_post(&self, token: &str, req_data: CreatePostRequest) -> Result<CreatePostResponse, ClientError> {
        let mut client = self.connect().await?;
        let mut req = Request::new(req_data);
        let val: MetadataValue<_> = format!("Bearer {token}")
            .parse()
            .map_err(|e: tonic::metadata::errors::InvalidMetadataValue| {
                ClientError::InvalidRequestError(format!("invalid token format: {e}"))
            })?;
        req.metadata_mut().insert("authorization", val);
        let res = client.create_post(req).await?.into_inner();
        Ok(res)
    }

    pub async fn get_post(&self, id: i64) -> Result<GetPostResponse, ClientError> {
        let mut client = self.connect().await?;
        let res = client.get_post(GetPostRequest { post_id: id }).await?.into_inner();
        Ok(res)
    }

    pub async fn update_post(&self, token: &str, req_data: UpdatePostRequest) -> Result<UpdatePostResponse, ClientError> {
        let mut client = self.connect().await?;
        let mut req = Request::new(req_data);
        let val: MetadataValue<_> = format!("Bearer {token}")
            .parse()
            .map_err(|e: tonic::metadata::errors::InvalidMetadataValue| {
                ClientError::InvalidRequestError(format!("invalid token format: {e}"))
            })?;
        req.metadata_mut().insert("authorization", val);
        let res = client.update_post(req).await?.into_inner();
        Ok(res)
    }

    pub async fn del_post(&self, token: &str, req_data: DeletePostRequest) -> Result<DeletePostResponse, ClientError> {
        let mut client = self.connect().await?;
        let mut req = Request::new(req_data);
        let val: MetadataValue<_> = format!("Bearer {token}")
            .parse()
            .map_err(|e: tonic::metadata::errors::InvalidMetadataValue| {
                ClientError::InvalidRequestError(format!("invalid token format: {e}"))
            })?;
        req.metadata_mut().insert("authorization", val);
        let res = client.delete_post(req).await?.into_inner();
        Ok(res)
    }

    pub async fn posts_list(&self, req_data: ListPostsRequest) -> Result<ListPostsResponse, ClientError> {
        let mut client = self.connect().await?;
        let res = client.list_posts(req_data).await?.into_inner();
        Ok(res)
    }
}