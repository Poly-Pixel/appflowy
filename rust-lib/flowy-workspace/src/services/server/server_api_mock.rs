use crate::{
    entities::{
        app::{App, CreateAppParams, DeleteAppParams, QueryAppParams, RepeatedApp, UpdateAppParams},
        view::{CreateViewParams, DeleteViewParams, QueryViewParams, RepeatedView, UpdateViewParams, View},
        workspace::{
            CreateWorkspaceParams,
            DeleteWorkspaceParams,
            QueryWorkspaceParams,
            RepeatedWorkspace,
            UpdateWorkspaceParams,
            Workspace,
        },
    },
    errors::WorkspaceError,
    services::server::WorkspaceServerAPI,
};
use flowy_infra::{future::ResultFuture, uuid};

pub struct WorkspaceServerMock {}

impl WorkspaceServerAPI for WorkspaceServerMock {
    fn create_workspace(&self, _token: &str, params: CreateWorkspaceParams) -> ResultFuture<Workspace, WorkspaceError> {
        let workspace = Workspace {
            id: uuid(),
            name: params.name,
            desc: params.desc,
            apps: RepeatedApp::default(),
        };

        ResultFuture::new(async { Ok(workspace) })
    }

    fn read_workspace(&self, _token: &str, _params: QueryWorkspaceParams) -> ResultFuture<RepeatedWorkspace, WorkspaceError> {
        ResultFuture::new(async {
            let repeated_workspace = RepeatedWorkspace { items: vec![] };
            Ok(repeated_workspace)
        })
    }

    fn update_workspace(&self, _token: &str, _params: UpdateWorkspaceParams) -> ResultFuture<(), WorkspaceError> {
        ResultFuture::new(async { Ok(()) })
    }

    fn delete_workspace(&self, _token: &str, _params: DeleteWorkspaceParams) -> ResultFuture<(), WorkspaceError> {
        ResultFuture::new(async { Ok(()) })
    }

    fn create_view(&self, _token: &str, params: CreateViewParams) -> ResultFuture<View, WorkspaceError> {
        let view = View {
            id: uuid(),
            belong_to_id: params.belong_to_id,
            name: params.name,
            desc: params.desc,
            view_type: params.view_type,
            version: 0,
            belongings: RepeatedView::default(),
        };
        ResultFuture::new(async { Ok(view) })
    }

    fn read_view(&self, _token: &str, _params: QueryViewParams) -> ResultFuture<Option<View>, WorkspaceError> {
        ResultFuture::new(async { Ok(None) })
    }

    fn delete_view(&self, _token: &str, _params: DeleteViewParams) -> ResultFuture<(), WorkspaceError> {
        ResultFuture::new(async { Ok(()) })
    }

    fn update_view(&self, _token: &str, _params: UpdateViewParams) -> ResultFuture<(), WorkspaceError> {
        ResultFuture::new(async { Ok(()) })
    }

    fn create_app(&self, _token: &str, params: CreateAppParams) -> ResultFuture<App, WorkspaceError> {
        let app = App {
            id: uuid(),
            workspace_id: params.workspace_id,
            name: params.name,
            desc: params.desc,
            belongings: RepeatedView::default(),
            version: 0,
        };
        ResultFuture::new(async { Ok(app) })
    }

    fn read_app(&self, _token: &str, _params: QueryAppParams) -> ResultFuture<Option<App>, WorkspaceError> {
        ResultFuture::new(async { Ok(None) })
    }

    fn update_app(&self, _token: &str, _params: UpdateAppParams) -> ResultFuture<(), WorkspaceError> { ResultFuture::new(async { Ok(()) }) }

    fn delete_app(&self, _token: &str, _params: DeleteAppParams) -> ResultFuture<(), WorkspaceError> { ResultFuture::new(async { Ok(()) }) }
}