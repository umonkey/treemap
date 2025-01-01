//! An attempt to implement a service locator.
//!
//! Can create classes that implement the Locatable trait, on demand,
//! keep instances, and return them when requested.
//!
//! Instances are kept in a HashMap, using TypeId as key, so guaranteed
//! to be unique.
//!
//! Keep in mind that the process of creating an instance can be slow and
//! async, so this should only be used while setting up the application,
//! not when actually processing jobs.

use crate::types::Error;
use crate::types::Result;
use log::{debug, error};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// A type alias to simplify typing.
type ServiceMap = HashMap<TypeId, Arc<dyn Any + Sync + Send>>;

pub struct Locator {
    map: Mutex<ServiceMap>,
}

pub trait Locatable {
    fn create(locator: &Locator) -> Result<Self>
    where
        Self: Sized;
}

impl Locator {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(ServiceMap::new()),
        }
    }

    // Get an instance of the desired class from the service map.
    //
    // The class must be Locatable.
    //
    // We perform look-up and on-demand creation in two different steps,
    // releasing the mutex in between.  This is needed to avoid deadlocks,
    // because mutexes are not re-entrant, even from the same thread.
    //
    // So, while we're creating a new instance of the object, we release the
    // mutex, and another thread might request the same object, which would
    // result in multiple instances being created.  This is not a problem,
    // because that's how we did it before this Locator went live.  The point
    // of this locator is not to enforce singletons, but to provide an easy
    // way to create services without passing too much dependency manually.
    //
    // Note that dependency loops must be tracked manually.
    pub fn get<T>(&self) -> Result<Arc<T>>
    where
        T: Locatable + 'static + Send + Sync,
    {
        let id = &TypeId::of::<Arc<T>>();

        // First see if the service already exists.
        match self.map.lock() {
            Ok(hash) => {
                if let Some(value) = hash.get(id) {
                    return match value.downcast_ref::<Arc<T>>() {
                        Some(instance) => {
                            debug!("Found existing instance in service locator: {:?}", id);
                            Ok(instance.clone())
                        }
                        None => {
                            debug!("Error downcasting instance in service locator: {:?}", id);
                            Err(Error::DependencyLoad)
                        }
                    };
                }
            }

            Err(e) => {
                error!("Error locking service map: {:?}", e);
                return Err(Error::DependencyLoad);
            }
        }

        let instance = Arc::new(T::create(self)?);

        match self.map.lock() {
            Ok(mut hash) => {
                hash.insert(*id, Arc::new(instance.clone()));
                debug!("Created new instance in service locator: {:?}", id);
                Ok(instance)
            }

            Err(e) => {
                error!("Error locking service map: {:?}", e);
                Err(Error::DependencyLoad)
            }
        }
    }
}

impl Default for Locator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod locator_tests {
    use super::*;

    fn require_send<T: Send>(_t: &T) {}

    fn require_sync<T: Sync>(_t: &T) {}

    struct TestService {}

    impl Locatable for TestService {
        fn create(_locator: &Locator) -> Result<Self> {
            Ok(Self {})
        }
    }

    fn setup() {
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_create() {
        Locator::new();
    }

    #[test]
    fn test_cached() {
        setup();

        let locator = Locator::new();
        locator.get::<TestService>().unwrap();
        locator.get::<TestService>().unwrap();
    }

    #[test]
    fn test_send_sync_map() {
        let map = ServiceMap::new();
        require_send(&map);
        require_sync(&map);
    }

    #[test]
    fn test_send_sync_locator() {
        let locator = Locator::new();
        require_send(&locator);
        require_sync(&locator);
    }
}
