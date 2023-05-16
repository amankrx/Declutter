use gtk::{glib, prelude::*};

mod imp {
    use gtk::{glib, prelude::*, subclass::prelude::*, CompositeTemplate};

    #[derive(Debug, CompositeTemplate, Default)]
    #[template(resource = "/com/amankrx/Declutter/ui/add_habit_window.ui")]
    pub struct AddHabitWindow {
        #[template_child]
        pub button_cancel: TemplateChild<gtk::Button>,
        #[template_child]
        pub button_ok: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddHabitWindow {
        const NAME: &'static str = "AddHabitWindow";
        type ParentType = gtk::Dialog;
        type Type = super::AddHabitWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::Type::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AddHabitWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.add_action_widget(&*self.button_cancel, gtk::ResponseType::Cancel);
            obj.add_action_widget(&*self.button_ok, gtk::ResponseType::Ok);
        }
    }
    impl WidgetImpl for AddHabitWindow {}
    impl WindowImpl for AddHabitWindow {}
    impl DialogImpl for AddHabitWindow {}
}

glib::wrapper! {
    /// popup dialog box that adds activity/weight/water intake data .
    pub struct AddHabitWindow(ObjectSubclass<imp::AddHabitWindow>)
        @extends gtk::Widget, gtk::Window, gtk::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl AddHabitWindow {
    /// Create a new [AddHabitWindow].
    ///
    /// # Arguments
    /// * `parent` - The [GtkWindow](gtk::Window) who is the transient parent of this dialog.
    pub fn new(parent: &gtk::Window) -> Self {
        gtk::Dialog::with_buttons(
            Some("Add Habit"),
            Some(parent),
            gtk::DialogFlags::MODAL,
            &[
                ("Cancel", gtk::ResponseType::Cancel),
                ("Ok", gtk::ResponseType::Ok),
            ],
        )
        .downcast()
        .expect("Failed to create AddHabitWindow")
    }
}
