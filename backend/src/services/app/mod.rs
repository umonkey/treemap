use crate::Result;
use crate::services::GoogleAuth;
use crate::services::database::get_database;
use crate::services::trees::Trees;
use crate::types::{AddTreeRequest, Bounds, TreeInfo, TreeList, LoginGoogleRequest, LoginResponse, TreeDetails};

pub struct AppState {
    trees: Trees,
    gauth: GoogleAuth,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let db = get_database().await?;

        Ok(Self {
            trees: Trees::init(&db).await,
            gauth: GoogleAuth::init(&db).await,
        })
    }

    pub async fn add_tree(&self, req: AddTreeRequest) -> Result<TreeInfo> {
        self.trees.add_tree(req).await
    }

    pub async fn get_trees(&self, bounds: Bounds) -> Result<TreeList> {
        self.trees.get_trees(bounds).await
    }

    pub async fn get_tree(&self, id: u64) -> Result<TreeDetails> {
        let tree = self.trees.get_tree(id).await?;
        Ok(TreeDetails::from_tree(&tree))
    }

    pub async fn login_google(&self, req: LoginGoogleRequest) -> Result<LoginResponse> {
        self.gauth.login(req).await
    }
}
