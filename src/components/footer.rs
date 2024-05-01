use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <div id="footer" class="pb-52 lg:pb-6 bg-stone-300 dark:bg-stone-600">
            <div class="flex justify-center items-center h-20">
                <a href="https://www.linkedin.com/in/steinardarri/" target="_blank" class="mr-6 lg:mr-4">
                    <img src="res/images/icons/linkedin.svg" width="36" height="24" />
                </a>
                <a href="https://github.com/steinardarri" target="_blank" class="mr-6 lg:mr-4">
                    <img src="res/images/icons/github.svg" width="24" height="24" />
                </a>
                <a href="mailto:Steinar@steinardth.xyz" class="mr-6 lg:mr-4">
                    <img src="res/images/icons/email.svg" width="24" height="24" />
                </a>
            </div>
            <p class="antialiased text-xl lg:text-md text-gray-700 dark:text-stone-200 text-center font-lg lg:font-medium pt-4 lg:pt-0 px-20">{
            "I constructed this website on the basis of the "}
            <a href="https://yew.rs" target="_blank" class="italic lg:hover:font-bold">{"yew.rs"}</a>
            {" framework. It is in constant development. For more technical insights feel free to access the "}
            <a href="https://github.com/steinardarri/rust-blog-and-stuff" target="_blank" class="italic lg:hover:font-bold">{"GitHub repository"}</a>
            {"."}
            </p>
        </div>
    }
}
