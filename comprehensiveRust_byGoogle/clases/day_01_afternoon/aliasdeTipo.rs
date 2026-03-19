enum CarryableConcreteItem {
    Left,
    Right,
}

//Permiten crear un alias. Ambos se pueden usar indistintamente.
type Item = CarryableConcreteItem;

// Los alias resultan de más utilidad con tipos largos y complejos:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;