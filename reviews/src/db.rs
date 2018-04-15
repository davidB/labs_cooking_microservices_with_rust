use actix::prelude::*;

pub struct DbExecutor {
    pub name: String,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug)]
pub struct GetName;

impl Message for GetName {
    type Result = Result<String, ()>;
}

impl Handler<GetName> for DbExecutor {
    type Result = Result<String, ()>;

    fn handle(&mut self, _msg: GetName, _: &mut Self::Context) -> Self::Result {
        Ok(self.name.clone())
    }
}

#[derive(Debug)]
pub struct SaveName {
    pub name: String,
}

impl Message for SaveName {
    type Result = Result<(), ()>;
}

impl Handler<SaveName> for DbExecutor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: SaveName, _: &mut Self::Context) -> Self::Result {
        self.name = msg.name;

        Ok(())
    }
}
