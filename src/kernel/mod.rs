pub mod config;
pub mod container;
pub mod error;
pub mod pharomachrus;

use std::future::Future;

use crate::kernel::config::Config;
use crate::kernel::container::Container;
use crate::kernel::error::KernelResult;
use crate::kernel::pharomachrus::Pharomachrus;

pub struct Kernel {
    config: Config,
    container: Container,
    pharomachrus: Pharomachrus,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            container: Container::new(),
            pharomachrus: Pharomachrus::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn container(&self) -> &Container {
        &self.container
    }

    pub fn container_mut(&mut self) -> &mut Container {
        &mut self.container
    }

    pub fn pharomachrus(&self) -> &Pharomachrus {
        &self.pharomachrus
    }

    pub async fn execute<F, T>(&self, future: F) -> KernelResult<T>
    where
        F: Future<Output = KernelResult<T>>,
    {
        self.pharomachrus.execute(future).await
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}
