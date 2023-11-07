use crate::node::Node;
use std::any::TypeId;

pub fn downcast<T: Node + 'static>(node: &dyn Node) -> Option<&T> {
    // Get `TypeId` of the type this function is instantiated with.
    let t = TypeId::of::<T>();

    // Get `TypeId` of the node we want to downcast.
    let concrete = node.type_id();

    // Compare both `TypeId`s on equality.
    if t == concrete {
        // Get the concrete type pointer from the trait object.
        let concrete = node as *const dyn Node as *const T;

        // Convert it to a reference and return it.
        //
        // SAFETY: This is safe because we know for sure that the pointer
        // is valid and references are only handed out for the lifetime
        // of the function.
        let concrete = unsafe { &*concrete };

        Some(concrete)
    } else {
        None
    }
}

pub fn downcast_mut<T: Node>(node: &mut dyn Node) -> Option<&mut T> {
    let t = TypeId::of::<T>();
    let concrete = node.type_id();

    if t == concrete {
        let concrete = node as *mut dyn Node as *mut T;
        let concrete = unsafe { &mut *concrete };

        Some(concrete)
    } else {
        None
    }
}
