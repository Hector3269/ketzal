use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::kernel::error::{KernelError, KernelResult};

#[derive(Default)]
pub struct Container {
    services: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: 'static + Send + Sync>(&mut self, service: T) -> KernelResult<()> {
        let type_id = TypeId::of::<T>();
        if self.services.contains_key(&type_id) {
            return Err(KernelError::Container(format!(
                "Service of type {} already registered",
                std::any::type_name::<T>()
            )));
        }
        self.services.insert(type_id, Box::new(service));
        Ok(())
    }

    pub fn resolve<T: 'static>(&self) -> KernelResult<&T> {
        let type_id = TypeId::of::<T>();
        self.services
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .ok_or_else(|| {
                KernelError::Container(format!(
                    "Service of type {} not found",
                    std::any::type_name::<T>()
                ))
            })
    }

    pub fn resolve_mut<T: 'static>(&mut self) -> KernelResult<&mut T> {
        let type_id = TypeId::of::<T>();
        self.services
            .get_mut(&type_id)
            .and_then(|boxed| boxed.downcast_mut::<T>())
            .ok_or_else(|| {
                KernelError::Container(format!(
                    "Service of type {} not found",
                    std::any::type_name::<T>()
                ))
            })
    }

    pub fn has<T: 'static>(&self) -> bool {
        self.services.contains_key(&TypeId::of::<T>())
    }
}
