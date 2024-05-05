use edge_lib::data;

pub struct DataManager {
    inner: Box<dyn data::AsDataManager>,
}

impl DataManager {
    pub  fn new() -> Self {
        Self {
            inner: Box::new(data::DataManager::new()),
        }
    }
}

impl data::AsDataManager for DataManager {
    fn divide(&self) -> Box<dyn data::AsDataManager> {
        Box::new(DataManager {
            inner: self.inner.divide()
        })
    }

    fn append_target_v(
        &mut self,
        source: &str,
        code: &str,
        target_v: &Vec<String>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send>> {
        self.inner.append_target_v(source, code, target_v)
    }

    fn append_source_v(
        &mut self,
        source_v: &Vec<String>,
        code: &str,
        target: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send>> {
        self.inner.append_source_v(source_v, code, target)
    }

    fn set_target_v(
        &mut self,
        source: &str,
        code: &str,
        target_v: &Vec<String>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send>> {
        self.inner.set_target_v(source, code, target_v)
    }

    fn set_source_v(
        &mut self,
        source_v: &Vec<String>,
        code: &str,
        target: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send>> {
        self.inner.set_source_v(source_v, code, target)
    }

    fn get_target(
        &mut self,
        source: &str,
        code: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<String>> + Send>> {
        self.inner.get_target(source, code)
    }

    fn get_source(
        &mut self,
        code: &str,
        target: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<String>> + Send>> {
        self.inner.get_source(code, target)
    }

    fn get_target_v(
        &mut self,
        source: &str,
        code: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<Vec<String>>> + Send>> {
        self.inner.get_target_v(source, code)
    }

    fn get_source_v(
        &mut self,
        code: &str,
        target: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<Vec<String>>> + Send>> {
        self.inner.get_source_v(code, target)
    }

    fn commit(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send>> {
        self.inner.commit()
    }
}
