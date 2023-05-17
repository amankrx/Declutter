use crate::models::Habit;
use gio::subclass::prelude::*;
use gtk::{gio, glib, prelude::*};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct HabitModel(pub(super) RefCell<Vec<Habit>>);

    /// Basic declaration of our type for the GObject type system
    #[glib::object_subclass]
    impl ObjectSubclass for HabitModel {
        const NAME: &'static str = "HabitModel";
        type Type = super::HabitModel;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for HabitModel {}

    impl ListModelImpl for HabitModel {
        fn item_type(&self) -> glib::Type {
            Habit::static_type()
        }
        fn n_items(&self) -> u32 {
            self.0.borrow().len() as u32
        }
        fn item(&self, position: u32) -> Option<glib::Object> {
            self.0
                .borrow()
                .get(position as usize)
                .map(|o| o.clone().upcast::<glib::Object>())
        }
    }
}

// Public part of the Model type.
glib::wrapper! {
    pub struct HabitModel(ObjectSubclass<imp::HabitModel>) @implements gio::ListModel;
}

// Constructor for new instances. This simply calls glib::Object::new()
impl HabitModel {
    pub fn new() -> HabitModel {
        glib::Object::new()
    }

    pub fn append(&self, obj: &Habit) {
        let imp = self.imp();
        let index = {
            // Borrow the data only once and ensure the borrow guard is dropped
            // before we emit the items_changed signal because the view
            // could call get_item / get_n_item from the signal handler to update its state
            let mut data = imp.0.borrow_mut();
            data.push(obj.clone());
            data.len() - 1
        };
        // Emits a signal that 1 item was added, 0 removed at the position index
        self.items_changed(index as u32, 0, 1);
    }

    pub fn remove(&self, index: u32) {
        let imp = self.imp();
        imp.0.borrow_mut().remove(index as usize);
        // Emits a signal that 1 item was removed, 0 added at the position index
        self.items_changed(index, 1, 0);
    }
}

impl Default for HabitModel {
    fn default() -> Self {
        Self::new()
    }
}
