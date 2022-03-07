use yew::prelude::{Component, Context, Html, html};

enum Msg {
  Increment,
}

struct Model {
  value: i64,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      value: 0,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Increment => {
        self.value += 1;
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link();
    html! {
      <div>
        <button onclick={link.callback(|_| Msg::Increment )}>
          { "Press to incremement" }
        </button>
      </div>
    }
  }
}

fn main() {
    yew::start_app::<Model>();
}
