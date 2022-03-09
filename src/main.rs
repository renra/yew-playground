use yew::prelude::{Component, Context, Html, html};

enum Msg {
  Increment,
  HiddenIncrement,
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

      Msg::HiddenIncrement => {
        self.value += 1;
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link();

    html! {
      <div>
        <div>
          { format!("Clicked number of times: {}", self.value) }
        </div>

        <div>
          <button onclick={link.callback(|_| Msg::Increment )}>
            { "Press to incremement" }
          </button>
        </div>

        <div>
          <button onclick={link.callback(|_| Msg::HiddenIncrement )}>
            { "Press to incremement without surfacing" }
          </button>
        </div>
      </div>
    }
  }
}

fn main() {
    yew::start_app::<Model>();
}
