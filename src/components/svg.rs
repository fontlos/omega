use dioxus::prelude::*;

#[inline]
#[component]
pub fn Omega() -> Element {
    rsx!(
        svg {
            "viewBox": "0 0 343.92 271.92",
            xmlns: "http://www.w3.org/2000/svg",
            "xmlns:xlink": "http://www.w3.org/1999/xlink",
            path {
                d: "M124.74,1.09l4.1,9.47c-22.4,12.5-41.59,30.03-56.07,51.2-13.92,17.2-24.75,36.69-32,57.6-6.59,19.81-9.96,40.56-9.98,61.44-1.63,19.08,4.14,38.06,16.13,52.99,10.79,10.13,25.15,15.56,39.94,15.11,17.57-2.79,33.75-11.24,46.08-24.06l2.3-2.05c-10.31-19.07-15.35-40.54-14.59-62.21-.72-11.77,3.03-23.38,10.5-32.51,8.28-9.93,20.61-15.57,33.54-15.36,23.8,0,35.84,11.26,35.84,34.3-2.1,18.32-9.58,35.6-21.5,49.66-6.43,9.27-13.81,17.85-22.02,25.6,15.1,19.64,38.73,30.79,63.49,29.95,16.7-.88,32.29-8.67,43.01-21.5,15.42-14.19,27.41-31.7,35.07-51.2,10.39-21.67,15.89-45.35,16.13-69.38.44-19.76-3.32-39.39-11.01-57.6-7.94-16.3-19.19-30.78-33.03-42.5l5.89-9.47c21.2,14.23,38.75,33.26,51.2,55.55,10.78,20.34,16.32,43.04,16.13,66.05-.56,26.59-6.76,52.77-18.18,76.8-11.51,21.09-27.93,39.09-47.88,52.48-15.91,11.77-35,18.46-54.78,19.2-29.35,1.28-57.66-10.99-76.8-33.28-7.68,6.4-15.62,12.29-23.55,17.92-13.74,9.51-29.9,14.93-46.59,15.62-21.1.78-41.26-8.77-54.02-25.6C7,226.79-.83,203.4.07,179.53c1.34-36.66,14.74-71.85,38.14-100.1C59.13,45.7,89.11,18.56,124.74,1.09h0ZM160.84,129.09c-6.54.16-12.77,2.82-17.41,7.43-4.8,6.29-7.17,14.11-6.66,22.02-.5,17.76,3.46,35.36,11.52,51.2,7.66-7.7,14.37-16.3,19.97-25.6,7.5-10.99,11.84-23.83,12.54-37.12,0-12.03-6.66-17.92-19.97-17.92h0ZM160.84,129.09",
            }
        }
    )
}

#[inline]
#[component]
pub fn Moon() -> Element {
    rsx!(
        svg {
            stroke: "currentColor",
            fill: "none",
            "stroke-width": "1.5",
            "viewBox": "0 0 24 24",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" }
        }
    )
}

#[inline]
#[component]
pub fn Setting() -> Element {
    rsx!(
        svg {
            "stroke-linejoin": "round",
            "viewBox": "0 0 24 24",
            fill: "none",
            "stroke-width": "1.5",
            stroke: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            "stroke-linecap": "round",
            circle { cx: "12", r: "3", cy: "12" }
            path { d: "M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z" }
        }
    )
}

#[inline]
#[component]
pub fn Close() -> Element {
    rsx!(
        svg {
            "stroke-linejoin": "round",
            "stroke-width": "1.5",
            fill: "none",
            "viewBox": "0 0 24 24",
            "stroke-linecap": "round",
            stroke: "currentColor",
            path { d: "M5 5l14 14M5 19l14-14" }
        }
    )
}

#[inline]
#[component]
pub fn Picture() -> Element {
    rsx!(
        svg {
            "stroke-linejoin": "round",
            fill: "none",
            "viewBox": "0 0 24 24",
            "stroke-width": "1.5",
            "stroke-linecap": "round",
            xmlns: "http://www.w3.org/2000/svg",
            stroke: "currentColor",
            class: "feather feather-image",
            rect {
                ry: "2",
                y: "3",
                rx: "2",
                x: "3",
                width: "18",
                height: "18",
            }
            circle { cx: "8.5", r: "1.5", cy: "8.5" }
            path { d: "M21 15l-5-5L5 21" }
        }
    )
}

#[inline]
#[component]
pub fn Paperclip() -> Element {
    rsx!(
        svg {
            stroke: "currentColor",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            xmlns: "http://www.w3.org/2000/svg",
            "viewBox": "0 0 24 24",
            fill: "none",
            "stroke-width": "1.5",
            class: "feather feather-paperclip",
            path { d: "M21.44 11.05l-9.19 9.19a6 6 0 01-8.49-8.49l9.19-9.19a4 4 0 015.66 5.66l-9.2 9.19a2 2 0 01-2.83-2.83l8.49-8.48" }
        }
    )
}

#[inline]
#[component]
pub fn Send(onclick: EventHandler<MouseEvent>) -> Element {
    rsx!(
        svg {
            onclick: move |evt| onclick.call(evt),
            fill: "none",
            stroke: "currentColor",
            "stroke-linecap": "round",
            "viewBox": "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            "stroke-width": "1.5",
            "stroke-linejoin": "round",
            class: "feather feather-thumbs-up",
            path { d: "M14 9V5a3 3 0 00-3-3l-4 9v11h11.28a2 2 0 002-1.7l1.38-9a2 2 0 00-2-2.3zM7 22H4a2 2 0 01-2-2v-7a2 2 0 012-2h3" }
        }
    )
}
