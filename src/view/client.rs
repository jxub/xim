use futures::Future;
use xrl;
use tokio_core::reactor::Handle;

pub struct Client {
    inner: xrl::Client,
    handle: Handle,
    view_id: xrl::ViewId,
}

impl Client {
    pub fn new(client: xrl::Client, handle: Handle, view_id: xrl::ViewId) -> Self {
        Client {
            inner: client,
            handle: handle,
            view_id: view_id,
        }
    }

    pub fn insert(&mut self, character: char) {
        let f = self.inner.char(self.view_id, character).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn scroll(&mut self, start: u64, end: u64) {
        let f = self.inner.scroll(self.view_id, start, end).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn down(&mut self) {
        let f = self.inner.down(self.view_id).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn up(&mut self) {
        let f = self.inner.up(self.view_id).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn right(&mut self) {
        let f = self.inner.right(self.view_id).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn left(&mut self) {
        let f = self.inner.left(self.view_id).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn page_down(&mut self) {
        let f = self.inner.page_up(self.view_id).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn page_up(&mut self) {
        let f = self.inner.page_up(self.view_id).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn delete(&mut self) {
        let f = self.inner.del(self.view_id).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn save(&mut self, file: &str) {
        let f = self.inner.save(self.view_id, file).map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn click(&mut self, line: u64, column: u64) {
        let f = self.inner
            .click(self.view_id, line, column)
            .map_err(|_| ());
        self.handle.spawn(f);
    }

    pub fn drag(&mut self, line: u64, column: u64) {
        let f = self.inner.drag(self.view_id, line, column).map_err(|_| ());
        self.handle.spawn(f);
    }
}
