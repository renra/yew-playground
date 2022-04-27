use yew::{Component, Context, html, Html};
use gloo_timers::callback::Interval;

pub enum Direction {
  Right,
  Left,
}

pub enum Msg {
  Tick,
}

pub struct App {
  direction: Direction,
  left_percentage: i8,
  min_left_bound: i8,
  max_right_bound: i8,
  interval: Interval,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let callback = ctx.link().callback(|_| Msg::Tick);
    let interval = Interval::new(1000, move || callback.emit(()));

    Self {
      direction: Direction::Right,
      left_percentage: 0,
      min_left_bound: 0,
      max_right_bound: 90,
      interval: interval
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Tick => {
        match self.direction {
          Direction::Right => {
            if self.left_percentage < self.max_right_bound {
              self.left_percentage = self.left_percentage + 1;
            } else {
              self.direction = Direction::Left;
              self.left_percentage = self.left_percentage - 1;
            }

            true
          }

          Direction::Left => {
            if self.left_percentage > self.min_left_bound {
              self.left_percentage = self.left_percentage - 1;
            } else {
              self.direction = Direction::Right;
              self.left_percentage = self.left_percentage + 1;
            }

            true
          }
        }
      }
    }
  }


  fn view(&self, _ctx: &Context<Self>) -> Html {
    let percentage = self.left_percentage;
    let style = format!("left: {percentage}%;");

    html! {
      <div class="web_workers">
        <h1>{ "Web workers" }</h1>

        <div class="animation_wrapper">
          <div class="animated" style={style}></div>
        </div>
      </div>
    }
  }
}

