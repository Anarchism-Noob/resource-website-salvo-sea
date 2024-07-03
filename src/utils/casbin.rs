use casbin::prelude::*;
use sea_orm_adapter::SeaOrmAdapter;
use tokio::sync::OnceCell;

use super::db::DB;

pub static CASBIN: OnceCell<Enforcer> = OnceCell::const_new();

pub async fn init_casbin() {
    CASBIN
        .get_or_init(|| async {
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
            // let enforcer = Enforcer::new(model, adapter).await.unwrap();

            // let mut evals = enforcer.enforce(("alice", "data1", "read")).unwrap();
            // debug!("evals: {}", evals);

            // evals = enforcer.enforce(("bob", "data2", "write")).unwrap();
            // debug!("evals: {}", evals);

            Enforcer::new(model, adapter).await.unwrap()
        })
        .await;
}
