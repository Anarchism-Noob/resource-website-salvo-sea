use casbin::prelude::*;
use once_cell::sync::Lazy;
use sea_orm_adapter::SeaOrmAdapter;
use tokio::sync::RwLock;

use super::db::DB;

pub static CASBIN: Lazy<RwLock<Option<Enforcer>>> = Lazy::new(|| RwLock::new(None));

pub async fn init_casbin() {
    let mut guard = CASBIN.write().await;

    if guard.is_none() {
        let mut model = DefaultModel::default();
        model.add_def("r", "r", "sub, obj, act");
        model.add_def("p", "p", "sub, obj, act");
        model.add_def("g", "g", "_, _");
        model.add_def("e", "e", "some(where (p.eft == allow))");
        model.add_def(
            "m",
            "m",
            "r.obj == p.obj && g(r.sub, p.sub) && r.act == p.act",
        );

        let db = DB.get().expect("Failed to get DB connection");
        let adapter = SeaOrmAdapter::new((*db).clone()).await.unwrap();

        *guard = Some(Enforcer::new(model, adapter).await.unwrap());
    }
}
