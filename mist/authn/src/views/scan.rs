use db::models::service::Service;
use maud::{html, Markup, PreEscaped};

pub(crate) fn view(service: &Service, authn_url: &str, qr: &str) -> Markup {
    html! {
        script src="https://cdn.twind.style" crossorigin {}
        link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.10/dist/full.min.css" rel="stylesheet" type="text/css";

        script {
            (PreEscaped(format!(r#"
                document.addEventListener("DOMContentLoaded", () => {{
                    const source = new EventSource("{0}", {{ withCredentials: true }});

                    source.onmessage = (event) => {{
                        if (event.data === "ready") {{
                            window.location.href = "{1}"
                        }}
                    }};
                }});
            "#, format!("{}/waiting", authn_url), service.redirect_url)))
        }

        title { (service.name) " | Mist" }

        body class="bg-gradient-to-t from-slate-200 to-slate-100";

        main class="flex flex-col items-center justify-center h-screen p-12" {
            h1 class="mb-4 text-2xl text-slate-700" { "Scan to sign in to " span class="font-semibold" { (service.name) } }

            img class="shadow-md" src={"data:image/png;base64," (qr)};

            p class="mt-4 text-slate-500" { "Your identity, your data —"
                span class="text-slate-700" { " Anchored in Mist" }
            }

            a class="mt-4 text-black opacity-10 hover:opacity-100 transition ease-in-out hover:-translate-y-1 hover:scale-110 duration-150 tooltip tooltip-bottom"
                href="https://mist.id"
                target="_blank"
                data-tip="Ready to own your identity?"
            {
                svg class="w-12 h-12" {
                    path d="M7.055 25.445c0 9.93 8.015 17.946 17.945 17.946 9.852 0 17.867-8.016 17.945-17.868a4.486 4.486 0 0 1-4.508 4.352c-2.44 0-4.503-1.984-4.503-4.508v-5.32c0-7.406-6.032-13.438-13.442-13.438-7.406 0-13.437 6.032-13.437 13.438Zm6.715-7.843a2.855 2.855 0 0 1 2.855 2.859v4c0 1.578-1.29 2.86-2.855 2.86a2.86 2.86 0 0 1-2.856-2.86v-4a2.86 2.86 0 0 1 2.856-2.86Zm8.37 0A2.856 2.856 0 0 1 25 20.46v4a2.868 2.868 0 0 1-2.86 2.86 2.86 2.86 0 0 1-2.85-2.86v-4a2.86 2.86 0 0 1 2.85-2.86Zm0 0";
                }
            }
        }
    }
}
