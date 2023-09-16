// NOTE
// This code is actually from a [demo project](https://github.com/BB-301/iced-text-input-wrapper-widget) I did earlier.

pub struct MyTextInputWrapper<'a, Message, Renderer> {
    content: iced::Element<'a, Message, Renderer>,
    on_focus_changed: Box<dyn Fn(bool) -> Message>,
}

pub fn my_text_input_wrapper<'a, Message, Renderer>(
    content: iced::widget::TextInput<'a, Message, Renderer>,
    on_focus_changed: impl Fn(bool) -> Message + 'static,
) -> MyTextInputWrapper<'a, Message, Renderer>
where
    Renderer: iced::advanced::text::Renderer + 'a,
    Renderer::Theme: iced::widget::text_input::StyleSheet,
    Message: 'a + Clone,
{
    MyTextInputWrapper {
        content: content.into(),
        on_focus_changed: Box::new(on_focus_changed),
    }
}

impl<'a, Message, Renderer> iced::advanced::Widget<Message, Renderer>
    for MyTextInputWrapper<'a, Message, Renderer>
where
    Renderer: iced::advanced::text::Renderer + 'a,
    Renderer::Theme: iced::widget::text_input::StyleSheet,
    Message: 'a + Clone,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<MyState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(MyState::default())
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::tree::Tree::new(&self.content)]
    }

    fn width(&self) -> iced::Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> iced::Length {
        self.content.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let limits = limits.width(self.width()).height(self.height());
        let content = self.content.as_widget().layout(renderer, &limits);
        iced::advanced::layout::Node::with_children(content.size(), vec![content])
    }

    fn operate(
        &self,
        state: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |op| {
            self.content.as_widget().operate(
                &mut state.children[0],
                layout.children().next().unwrap(),
                renderer,
                op,
            )
        });
    }

    fn on_event(
        &mut self,
        state: &mut iced::advanced::widget::Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::event::Status {
        let status = self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        // NOTE: Maybe this strategy is a little bit aggresive, but I'm not
        // sure. And maybe this won't be enough to cover all "blurred" cases,
        // but again I still need to think about it, so let me know if you find
        // anything missing/wrong with this logic.
        let my_state = state.state.downcast_mut::<MyState>();
        let text_field_state = state.children[0]
            .state
            .downcast_ref::<iced::widget::text_input::State>();
        if text_field_state.is_focused() != my_state.focused {
            my_state.focused = !my_state.focused;
            shell.publish((self.on_focus_changed)(my_state.focused));
        }

        status
    }

    fn draw(
        &self,
        state: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced::advanced::Renderer>::Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        state: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            state,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message, Renderer> std::convert::From<MyTextInputWrapper<'a, Message, Renderer>>
    for iced::Element<'a, Message, Renderer>
where
    Renderer: iced::advanced::text::Renderer + 'a,
    Renderer::Theme: iced::widget::text_input::StyleSheet,
    Message: 'a + Clone,
{
    fn from(value: MyTextInputWrapper<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct MyState {
    focused: bool,
}

impl Default for MyState {
    fn default() -> Self {
        Self { focused: false }
    }
}
