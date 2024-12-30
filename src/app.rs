use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};

#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;

use crate::components::show_data_from_api::ShowDataFromApi;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="site-root" content={format!("{:?}", options.clone().site_root)}/>
                <meta name="site-pkg-dir" content={format!("{:?}", options.clone().site_pkg_dir)}/>
                <meta name="output-name" content={format!("{:?}", options.clone().output_name)}/>
                <meta name="env" content={format!("{:?}", options.clone().env)}/>
                <meta name="site-addr" content={format!("{:?}", options.clone().site_addr)}/>
                <meta name="reload-port" content={format!("{:?}", options.clone().reload_port)}/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Welcome to Leptos"/>

        <h1>"Hello world!"</h1>
        <ShowDataFromApi />
    }
}
