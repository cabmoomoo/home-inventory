use yew::prelude::*;

pub enum MessageContainerAction {
    Change { name: AttrValue, message: AttrValue, additional_actions: Option<Vec<(Callback<MouseEvent>, AttrValue)>> },
    Make(MessageContainer)
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MessageContainer {
    pub name: AttrValue,
    pub message: AttrValue,
    pub additional_actions: Option<Vec<(Callback<MouseEvent>, AttrValue)>>
} impl Reducible for MessageContainer {
    type Action = MessageContainerAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let new_message;

        match action {
            MessageContainerAction::Change { name, message, additional_actions } => {
                new_message = MessageContainer { name, message, additional_actions };
            },
            MessageContainerAction::Make(msg) => new_message = msg
        }

        Self { ..new_message }.into()
    }
}

pub fn success_message(msg: String) -> MessageContainerAction {
    MessageContainerAction::Make(MessageContainer {
        name: "Success".into(),
        message: msg.into(),
        additional_actions: None
    })
}
pub fn error_message(msg: String) -> MessageContainerAction {
    MessageContainerAction::Make(MessageContainer {
        name: "Error".into(),
        message: msg.into(),
        additional_actions: None
    })
}

#[derive(Debug)]
pub enum MessageBoxMsg {
    Change(MessageContainer),
    Close
}

// #[derive(Properties, PartialEq)]
// pub struct MessageBoxProps {
//     pub container: MessageContainer
// }

pub struct MessageBox {
    box_open: bool,
    name: AttrValue,
    message: AttrValue,
    close_button: NodeRef,
    additional_actions: Option<Vec<(Callback<MouseEvent>, AttrValue)>>,
    _context_listener: ContextHandle<MessageContainer>
}

impl Component for MessageBox {
    type Message = MessageBoxMsg;

    type Properties = ();//MessageBoxProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (container, context_listener) = ctx.link().context::<MessageContainer>(ctx.link().callback(MessageBoxMsg::Change)).expect("No ctx found");
        // let container = message_ctx.
        // let container = &ctx.props().container;
        if container.name.eq(&AttrValue::default()) {
            return Self { 
                box_open: false, 
                name: AttrValue::default(), 
                message: AttrValue::default(), 
                close_button: NodeRef::default(),
                additional_actions: None, 
                _context_listener: context_listener };
        } else {
            return Self { 
                box_open: true, 
                name: container.name.clone(), 
                message: container.message.clone(), 
                close_button: NodeRef::default(),
                additional_actions: container.additional_actions.clone(),
                _context_listener: context_listener }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MessageBoxMsg::Change(m) => {
                self.box_open = true;
                self.name = m.name.clone();
                self.message = m.message.clone();
                self.additional_actions = m.additional_actions.clone();
            },
            MessageBoxMsg::Close => self.box_open = false,
        }

        true
    }

    // fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        

    //     true
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let box_open = 
            if !self.box_open {
                Some("hidden")
            } else {
                None
            };

        let close_button: Html = html!(
            <button ref={&self.close_button} onclick={ctx.link().callback(|_| MessageBoxMsg::Close)}>{"Close"}</button>
        );

        let mut additional_buttons: Vec<Html> = vec![];
        if self.additional_actions.is_some() {
            for (callback, message) in self.additional_actions.clone().unwrap() {
                let close_event = ctx.link().callback(|_: MouseEvent| MessageBoxMsg::Close);
                additional_buttons.push(html!(
                    <button onclick={move |e: MouseEvent| {callback.emit(e.clone()); close_event.emit(e);}}>{message}</button>
                ));
            }
        }
        
        html!(<div class={classes!("msg-box", box_open)}>
            <h3>{self.name.clone()}</h3>
            <p>{self.message.clone()}</p>
            <div class="msg-options">
                {for additional_buttons}
                {close_button}
            </div>
        </div>)
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let button_cast = self.close_button.cast::<web_sys::HtmlButtonElement>();
        if let Some(b) = button_cast {
            let _ = b.focus();
        }
    }
}