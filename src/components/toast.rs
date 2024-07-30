use std::time::Duration;

use leptos::prelude::*;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct ToastMessage {
    pub message: String,
    pub toast_type: ToastType,
    pub visible: bool,
}

#[component]
pub fn ToastComponent() -> impl IntoView {
    let (toast, set_toast) = signal::<ToastMessage>(ToastMessage {
        message: String::new(),
        toast_type: ToastType::Success,
        visible: false,
    });

    provide_context::<WriteSignal<ToastMessage>>(set_toast);

    let base_toast_classes =
        "toast toast-top toast-center transition-opacity duration-500 ease-in-out z-50";
    let toast_parent_classes = move || -> String {
        let t = toast.read();
        let opacity_class = if t.visible == true {
            "opacity-1".to_string()
        } else {
            "opacity-0".to_string()
        };

        format!("{} {}", base_toast_classes, opacity_class)
    };

    let toast_child_classes = move || -> String {
        let t = toast.read();
        let background_class = match t.toast_type {
            ToastType::Success => "alert alert-success",
            ToastType::Error => "alert alert-error",
        };

        format!("{}", background_class)
    };

    Effect::new(move |_| {
        let t = toast.read();
        if t.visible {
            set_timeout(
                move || {
                    set_toast.update(|msg| {
                        msg.visible = false;
                    });
                },
                Duration::new(4, 0),
            );
        }
    });

    view! {
        <div id="toast" class=toast_parent_classes>
            <div class={toast_child_classes}>
                <span>{move || toast.get().message}</span>
            </div>
        </div>
    }
}
