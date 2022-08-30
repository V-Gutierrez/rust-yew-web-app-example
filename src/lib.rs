use yew::prelude::*;

pub enum Msg {
    AddOne,
    MinusOne,
}

pub struct Model {
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
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::MinusOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>
                 {"+1"}
                 </button>
                <p>{self.value}</p>
                <button onclick={link.callback(|_| Msg::MinusOne)}>{"-1"}
                </button>
                <script>
                {"alert('hello, I can execute JS too :D ')"}
                </script>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}

}