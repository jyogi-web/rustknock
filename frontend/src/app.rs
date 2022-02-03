use yew::prelude::*;

pub struct Navigation;

impl Component for Navigation {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-expand-md navbar-dark bg-dark fixed-top">
                <a class="navbar-brand" href="#">{"Navbar"}</a>
                <button class="navbar-toggler">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarsExampleDefault">
                    <ul class="navbar-nav mr-auto">
                        <li class="nav-item active">
                            <a class="nav-link" href="#">
                                {"Home "}<span class="sr-only">{"(current)"}</span>
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="https://example.com">{"Link"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link disabled" href="#">{"Disabled"}</a>
                        </li>
                    </ul>
                    <form class="form-inline my-2 my-lg-0">
                        <input class="form-control mr-sm-2" type="text" placeholder="Search" />
                        <button class="btn btn-outline-success my-2 my-sm-0" type="submit">{"Search"}</button>
                    </form>
                </div>
            </nav>
        }
    }
}

pub struct Content;

impl Component for Content {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class="container">
                <div class="starter-template">
                    <h1>{"Bootstrap starter template"}</h1>
                    <p class="lead">
                        {"Use this document as a way to quickly start any new project."}
                        <br/>
                        {"All you get is this text and a mostly barebones HTML document."}
                    </p>
                </div>
            </main>
        }
    }
}
pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
          <div>
                <Navigation />
                <Content />
                <button type="button" class="btn btn-primary">{"Primary"}</button>
            </div>
        )
    }
}
